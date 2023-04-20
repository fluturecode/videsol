// Import necessary Solana SDK structs and macros
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

// Define data structure for VideoAccount
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VideoAccount {
    pub is_initialized: bool,
    pub video_data: Vec<u8>,
}

// Implement Pack trait for VideoAccount to enable serialization
impl Pack for VideoAccount {
    const LEN: usize = 1 + 4 + 1024; // is_initialized (1 byte) + video_data length (4 bytes) + max video data length (1024 bytes)

    fn pack_into_slice(&self, output: &mut [u8]) {
        let (is_initialized_bytes, video_data_len_bytes, video_data_bytes) = self.to_bytes();

        output[0] = is_initialized_bytes[0];
        output[1..5].copy_from_slice(&video_data_len_bytes);
        output[5..5 + self.video_data.len()].copy_from_slice(&video_data_bytes);
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let is_initialized = match input[0] {
            0 => false,
            1 => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let video_data_len = u32::from_le_bytes(input[1..5].try_into().unwrap()) as usize;
        let video_data = input[5..5 + video_data_len].to_vec();

        Ok(VideoAccount {
            is_initialized,
            video_data,
        })
    }
}

// Implement IsInitialized and Sealed traits for VideoAccount
impl IsInitialized for VideoAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Sealed for VideoAccount {}

// Define constants for VideoAccount account state
pub const VIDEO_ACCOUNT_LEN: usize = VideoAccount::LEN;
pub const VIDEO_ACCOUNT_SEED: &str = "video_account";

// Define functions for VideoAccount
impl VideoAccount {
    // Create new VideoAccount instance
    pub fn new() -> Self {
        VideoAccount {
            is_initialized: true,
            video_data: vec![],
        }
    }

    // Set video data for VideoAccount
    pub fn set_video_data(&mut self, video_data: &[u8]) {
        self.video_data = video_data.to_vec();
    }
}
