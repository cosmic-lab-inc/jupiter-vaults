use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[event]
#[derive(Default)]
pub struct VaultRecord {
    pub ts: i64,
    pub spot_market_index: u16,
    pub vault_equity_before: u64,
}

#[event]
#[derive(Default)]
pub struct InvestorRecord {
    pub ts: i64,
    pub vault: Pubkey,
    pub depositor_authority: Pubkey,
    pub action: InvestorAction,
    pub amount: u64,

    pub mint: Pubkey,
    pub vault_shares_before: u128,
    pub vault_shares_after: u128,

    pub vault_equity_before: u64,

    pub user_vault_shares_before: u128,
    pub total_vault_shares_before: u128,

    pub user_vault_shares_after: u128,
    pub total_vault_shares_after: u128,

    pub protocol_profit_share: u64,
    pub protocol_fee: i64,
    pub protocol_fee_shares: i64,

    pub manager_profit_share: u64,
    pub management_fee: i64,
    pub management_fee_shares: i64,
}

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq, Default)]
pub enum InvestorAction {
    #[default]
    Deposit,
    WithdrawRequest,
    CancelWithdrawRequest,
    Withdraw,
    FeePayment,
}
