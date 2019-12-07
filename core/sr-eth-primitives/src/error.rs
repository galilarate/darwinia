use codec::{Decode, Encode};

/// Define errors when verifying eth blocks
use super::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
/// Error indicating value found is outside of a valid range.
pub struct OutOfBounds<T> {
	/// Minimum allowed value.
	pub min: Option<T>,
	/// Maximum allowed value.
	pub max: Option<T>,
	/// Value found.
	pub found: T,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Encode, Decode)]
/// Error indicating an expected value was not found.
pub struct Mismatch<T> {
	/// Value expected.
	pub expected: T,
	/// Value found.
	pub found: T,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BlockError {
	InvalidProofOfWork(OutOfBounds<U256>),
	DifficultyOutOfBounds(OutOfBounds<U256>),
	InvalidSealArity(Mismatch<usize>),
	Rlp(&'static str),
}

impl From<BlockError> for &str {
	fn from(e: BlockError) -> Self {
		use BlockError::*;

		match e {
			InvalidProofOfWork(_) => "Proof Of Work - INVALID",
			DifficultyOutOfBounds(_) => "Difficulty - OUT OF BOUNDS",
			InvalidSealArity(_) => "Seal Arity - INVALID",
			Rlp(msg) => msg,
		}
	}
}

//#[cfg(feature = "std")]
//impl Error for BlockError {
//	fn description(&self) -> &str {
//		"Block error"
//	}
//}
