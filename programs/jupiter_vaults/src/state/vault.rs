use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use crate::Size;
use crate::state::withdraw_request::WithdrawRequest;
use drift_macros::assert_no_slop;

// #[assert_no_slop]
#[account(zero_copy(unsafe))]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Vault {
    /// The name of the vault. Vault pubkey is derived from this name.
    pub name: [u8; 32],
    /// The vault's pubkey. It is a PDA also used as the authority token accounts
    pub pubkey: Pubkey,
    /// The manager of the vault who has ability to update vault config,
    /// and earns a profit share or management fee.
    pub manager: Pubkey,
    /// The token mint the vault deposits into/withdraws from (e.g., USDC).
    pub mint: Pubkey,
    /// The vault token account. Used to receive tokens between deposits and withdrawals.
    /// This is a PDA of the vault signer seed and the mint defined for this vault.
    pub token_account: Pubkey,
    /// The delegate is the "portfolio manager", "trader", or "bot" that manages the vault's assets.
    /// They can swap 100% of vault tokens using Jupiter.
    pub delegate: Pubkey,
    /// The sum of all shares held by the investors
    pub investor_shares: u128,
    /// The sum of all shares: investor deposits, manager deposits, manager profit/fee, and protocol profit/fee.
    /// The manager deposits are total_shares - investor_shares - protocol_profit_and_fee_shares.
    pub total_shares: u128,
    /// Last fee update unix timestamp
    pub last_fee_update_ts: i64,
    /// The period (in seconds) that an investor must wait after requesting a withdrawal to transfer funds.
    /// The maximum is 90 days. 
    /// This is only updatable to lesser values.
    pub redeem_period: i64,
    /// The sum of all outstanding withdraw requests
    pub total_withdraw_requested: u64,
    /// Max token capacity, once hit/passed vault will reject new deposits.
    /// This is only updatable to lesser values.
    pub max_tokens: u64,
    /// The annual fee charged on deposits by the manager.
    /// Traditional funds typically charge 2% per year on assets under management.
    /// This is only updatable to lesser values.
    pub management_fee: i64,
    /// Timestamp vault initialized
    pub init_ts: i64,
    /// The net deposits for the vault
    pub net_deposits: i64,
    /// The net deposits for the manager
    pub manager_net_deposits: i64,
    /// Total deposits
    pub total_deposits: u64,
    /// Total withdraws
    pub total_withdraws: u64,
    /// Total deposits for the manager
    pub manager_total_deposits: u64,
    /// Total withdraws for the manager
    pub manager_total_withdraws: u64,
    /// Total management fee accrued by the manager
    pub manager_total_fee: i64,
    /// Total profit share accrued by the manager
    pub manager_total_profit_share: u64,
    /// The minimum deposit amount.
    /// This is only updatable to lesser values.
    pub min_deposit_amount: u64,
    pub last_manager_withdraw_request: WithdrawRequest,
    /// The base 10 exponent of the shares (given massive share inflation can occur at near zero vault equity)
    pub shares_base: u32,
    /// Percentage the manager charges on all profits realized by depositors (multiplied by PERCENTAGE_PRECISION).
    /// Traditional funds typically charge 20% of profits.
    /// This is only updatable to lesser values.
    pub profit_share: u32,
    /// Vault manager only collect incentive fees during periods when returns are higher than this amount (multiplied by PERCENTAGE_PRECISION).
    pub hurdle_rate: u32,
    /// The bump for the vault PDA
    pub bump: u8,
    /// Whether anyone can be an investor
    pub permissioned: bool,

    /// The protocol, company, or entity that services the product using this vault.
    /// The protocol is not allowed to deposit into the vault but can profit share and collect annual fees just like the manager.
    pub protocol: Pubkey,
    /// The shares from profit share and annual fee unclaimed by the protocol.
    pub protocol_profit_and_fee_shares: u128,
    /// The annual fee charged on deposits by the protocol (traditional hedge funds typically charge 2% per year on assets under management).
    /// Unlike the management fee this can't be negative.
    pub protocol_fee: u64,
    /// Total withdraws for the protocol
    pub protocol_total_withdraws: u64,
    /// Total fee charged by the protocol (annual management fee + profit share).
    /// Unlike the management fee this can't be negative.
    pub protocol_total_fee: u64,
    /// Total profit share charged by the protocol
    pub protocol_total_profit_share: u64,
    pub last_protocol_withdraw_request: WithdrawRequest,
    /// Percentage the protocol charges on all profits realized by depositors: PERCENTAGE_PRECISION
    pub protocol_profit_share: u32,
    pub version: u8,
    /// [`VaultProtocol`] is 117 bytes with padding to 120 bytes to make it a multiple of 8.
    pub padding: [u8; 2],
}

impl Vault {
    pub fn get_vault_signer_seeds<'a>(name: &'a [u8], bump: &'a u8) -> [&'a [u8]; 3] {
        [b"vault".as_ref(), name, bytemuck::bytes_of(bump)]
    }
}

impl Size for Vault {
    const SIZE: usize = 512 + 8;
}
const_assert_eq!(Vault::SIZE, std::mem::size_of::<Vault>() + 8);

#[test]
fn s() {
    let a = Vault::SIZE;
    let b = std::mem::size_of::<Vault>() + 8;
    println!("{} == {}", a, b);
}