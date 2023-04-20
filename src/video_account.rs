use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

// Solana SDK attribute macro that is used to annotate the VideoAccount account initialization function.
#[account(init, payer = user, space = 8 + name_len + desc_len + contact_len)]
pub struct CreateVideoAccount<'info> {
    pub video_account: Account<'info, VideoAccount>,
    pub user: AccountInfo<'info>,
}

pub struct VideoAccount {
    pub is_initialized: bool,
    pub name: String,
    pub description: String,
    pub content: Vec<u8>,
}
