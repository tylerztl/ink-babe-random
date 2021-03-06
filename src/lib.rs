#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
pub enum CustomEnvironment {}

#[derive(Debug, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Randomness {
    pub epoch: u64,
    pub start_slot: u64,
    pub duration: u64,
    pub randomness:<ink_env::DefaultEnvironment as Environment>::Hash,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
pub enum ErrorCode {}

impl ink_env::chain_extension::FromStatusCode for ErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[ink::chain_extension]
pub trait BabeRandomnessExt {
    type ErrorCode = ErrorCode;

    /// Reads from runtime storage.
    #[ink(extension = 0x00010000, returns_result = false)]
    fn current_epoch() -> Randomness;

    #[ink(extension = 0x00010001, returns_result = false)]
    fn next_epoch() -> Randomness;

    #[ink(extension = 0x00010002, returns_result = false)]
    fn randomness_of(epoch: u64) -> <ink_env::DefaultEnvironment as Environment>::Hash;

    #[ink(extension = 0x00010003, returns_result = false)]
    fn random(subject: &[u8]) -> <ink_env::DefaultEnvironment as Environment>::Hash;
}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;

    type ChainExtension = BabeRandomnessExt;
}
