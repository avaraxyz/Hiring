pub mod constants;
pub mod elliptic_curves;

/// A struct representing group parameters in cryptographic protocols.
#[derive(Copy, Clone, Debug)]
pub struct GroupParams<T> {
    /// The generator `g` of the group.
    pub g: T,
    /// An additional generator `h` of the group, ensuring it's independent from `g`.
    pub h: T,
    /// The prime modulus `p` defining the size of the group.
    pub p: T,
    /// The order `q` of the subgroup generated by `g` and `h`.
    pub q: T,
}

/// A trait defining the interface for the zero-knowledge protocol.
///
/// This trait provides the necessary methods for implementing the protocol,
/// which is a cryptographic protocol for proving knowledge of a secret without revealing it.
pub trait Protocol {
    type Secret;
    type Response;
    type Challenge;
    type GroupParameters;
    type CommitParameters;
    type CommitmentRandom;

    /// Calculates the commitment in the protocol.
    ///
    /// # Arguments
    /// * `params` - Group parameters used in the protocol.
    /// * `x` - The secret value for which the commitment is calculated.
    ///
    /// # Returns
    /// A tuple containing the commitment parameters and the commitment randomness.
    fn commitment(
        params: &Self::GroupParameters,
        x: &Self::Secret,
    ) -> (Self::CommitParameters, Self::CommitmentRandom)
    where
        Self: Sized;

    /// Generates a challenge in the protocol.
    ///
    /// # Arguments
    /// * `params` - Group parameters used in the protocol.
    ///
    /// # Returns
    /// The challenge value used in the protocol.
    fn challenge(params: &Self::GroupParameters) -> Self::Challenge
    where
        Self: Sized;

    /// Calculates the challenge response in the protocol.
    ///
    /// # Arguments
    /// * `params` - Group parameters used in the protocol.
    /// * `k` - The commitment randomness used in the protocol.
    /// * `c` - The challenge value used in the protocol.
    /// * `x` - The secret value for which the response is calculated.
    ///
    /// # Returns
    /// The response value in the protocol.
    fn challenge_response(
        params: &Self::GroupParameters,
        k: &Self::CommitmentRandom,
        c: &Self::Challenge,
        x: &Self::Secret,
    ) -> Self::Response
    where
        Self: Sized;

    /// Verifies the response in the protocol.
    ///
    /// # Arguments
    /// * `params` - Group parameters used in the protocol.
    /// * `s` - The response value to be verified.
    /// * `c` - The challenge value used in the protocol.
    /// * `cp` - The commitment parameters used in the protocol.
    ///
    /// # Returns
    /// A boolean indicating whether the verification was successful.
    fn verify(
        params: &Self::GroupParameters,
        s: &Self::Response,
        c: &Self::Challenge,
        cp: &Self::CommitParameters,
    ) -> bool
    where
        Self: Sized;
}
