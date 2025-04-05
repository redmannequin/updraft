#![allow(dead_code)]
use borsh::{BorshDeserialize, BorshSerialize};
use sol_ez::*;
pub enum RaydiumCpSwap {
    CreateAmmConfig(CreateAmmConfig),
    UpdateAmmConfig(UpdateAmmConfig),
    UpdatePoolStatus(UpdatePoolStatus),
    CollectProtocolFee(CollectProtocolFee),
    CollectFundFee(CollectFundFee),
    Initialize(Initialize),
    Deposit(Deposit),
    Withdraw(Withdraw),
    SwapBaseInput(SwapBaseInput),
    SwapBaseOutput(SwapBaseOutput),
}
impl RaydiumCpSwap {
    pub const VERSION: (u8, u8, u8) = (0u8, 1u8, 0u8);
    pub fn parse(data: &[u8]) -> Self {
        let (discriminator, ix_data) = data.split_at(8usize);
        let discriminator = {
            let mut ix = [0; 8usize];
            ix.copy_from_slice(discriminator);
            ix
        };
        match discriminator {
            CreateAmmConfig::DISCRIMINATOR => {
                Self::CreateAmmConfig(borsh::from_slice(ix_data).unwrap())
            }
            UpdateAmmConfig::DISCRIMINATOR => {
                Self::UpdateAmmConfig(borsh::from_slice(ix_data).unwrap())
            }
            UpdatePoolStatus::DISCRIMINATOR => {
                Self::UpdatePoolStatus(borsh::from_slice(ix_data).unwrap())
            }
            CollectProtocolFee::DISCRIMINATOR => {
                Self::CollectProtocolFee(borsh::from_slice(ix_data).unwrap())
            }
            CollectFundFee::DISCRIMINATOR => {
                Self::CollectFundFee(borsh::from_slice(ix_data).unwrap())
            }
            Initialize::DISCRIMINATOR => {
                Self::Initialize(borsh::from_slice(ix_data).unwrap())
            }
            Deposit::DISCRIMINATOR => Self::Deposit(borsh::from_slice(ix_data).unwrap()),
            Withdraw::DISCRIMINATOR => {
                Self::Withdraw(borsh::from_slice(ix_data).unwrap())
            }
            SwapBaseInput::DISCRIMINATOR => {
                Self::SwapBaseInput(borsh::from_slice(ix_data).unwrap())
            }
            SwapBaseOutput::DISCRIMINATOR => {
                Self::SwapBaseOutput(borsh::from_slice(ix_data).unwrap())
            }
            _ => panic!("this should be an error"),
        }
    }
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CreateAmmConfig {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
}
impl CreateAmmConfig {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        130u8, 84u8, 4u8, 26u8, 76u8, 172u8, 75u8, 201u8,
    ];
}
pub struct CreateAmmConfigAccounts {
    owner: Account<(), Mutable, Signed>,
    amm_config: Account<(), Mutable, Unsigned>,
    system_program: Account<(), ReadOnly, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdateAmmConfig {
    pub param: u8,
    pub value: u64,
}
impl UpdateAmmConfig {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        152u8, 186u8, 244u8, 244u8, 33u8, 143u8, 40u8, 56u8,
    ];
}
pub struct UpdateAmmConfigAccounts {
    owner: Account<(), ReadOnly, Signed>,
    amm_config: Account<(), Mutable, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct UpdatePoolStatus {
    pub status: u8,
}
impl UpdatePoolStatus {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        109u8, 206u8, 182u8, 23u8, 100u8, 203u8, 122u8, 211u8,
    ];
}
pub struct UpdatePoolStatusAccounts {
    authority: Account<(), ReadOnly, Signed>,
    pool_state: Account<(), Mutable, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CollectProtocolFee {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}
impl CollectProtocolFee {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        210u8, 18u8, 27u8, 122u8, 16u8, 222u8, 78u8, 68u8,
    ];
}
pub struct CollectProtocolFeeAccounts {
    owner: Account<(), ReadOnly, Signed>,
    authority: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    amm_config: Account<(), ReadOnly, Unsigned>,
    token_0_vault: Account<(), Mutable, Unsigned>,
    token_1_vault: Account<(), Mutable, Unsigned>,
    vault_0_mint: Account<(), ReadOnly, Unsigned>,
    vault_1_mint: Account<(), ReadOnly, Unsigned>,
    recipient_token_0_account: Account<(), Mutable, Unsigned>,
    recipient_token_1_account: Account<(), Mutable, Unsigned>,
    token_program: Account<(), ReadOnly, Unsigned>,
    token_program_2022: Account<(), ReadOnly, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct CollectFundFee {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}
impl CollectFundFee {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        179u8, 34u8, 28u8, 102u8, 236u8, 29u8, 144u8, 76u8,
    ];
}
pub struct CollectFundFeeAccounts {
    owner: Account<(), ReadOnly, Signed>,
    authority: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    amm_config: Account<(), ReadOnly, Unsigned>,
    token_0_vault: Account<(), Mutable, Unsigned>,
    token_1_vault: Account<(), Mutable, Unsigned>,
    vault_0_mint: Account<(), ReadOnly, Unsigned>,
    vault_1_mint: Account<(), ReadOnly, Unsigned>,
    recipient_token_0_account: Account<(), Mutable, Unsigned>,
    recipient_token_1_account: Account<(), Mutable, Unsigned>,
    token_program: Account<(), ReadOnly, Unsigned>,
    token_program_2022: Account<(), ReadOnly, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Initialize {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
}
impl Initialize {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8,
    ];
}
pub struct InitializeAccounts {
    creator: Account<(), Mutable, Signed>,
    amm_config: Account<(), ReadOnly, Unsigned>,
    authority: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    token_0_mint: Account<(), ReadOnly, Unsigned>,
    token_1_mint: Account<(), ReadOnly, Unsigned>,
    lp_mint: Account<(), Mutable, Unsigned>,
    creator_token_0: Account<(), Mutable, Unsigned>,
    creator_token_1: Account<(), Mutable, Unsigned>,
    creator_lp_token: Account<(), Mutable, Unsigned>,
    token_0_vault: Account<(), Mutable, Unsigned>,
    token_1_vault: Account<(), Mutable, Unsigned>,
    create_pool_fee: Account<(), Mutable, Unsigned>,
    observation_state: Account<(), Mutable, Unsigned>,
    token_program: Account<(), ReadOnly, Unsigned>,
    token_0_program: Account<(), ReadOnly, Unsigned>,
    token_1_program: Account<(), ReadOnly, Unsigned>,
    associated_token_program: Account<(), ReadOnly, Unsigned>,
    system_program: Account<(), ReadOnly, Unsigned>,
    rent: Account<(), ReadOnly, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Deposit {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}
impl Deposit {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        242u8, 35u8, 198u8, 137u8, 82u8, 225u8, 242u8, 182u8,
    ];
}
pub struct DepositAccounts {
    owner: Account<(), ReadOnly, Signed>,
    authority: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    owner_lp_token: Account<(), Mutable, Unsigned>,
    token_0_account: Account<(), Mutable, Unsigned>,
    token_1_account: Account<(), Mutable, Unsigned>,
    token_0_vault: Account<(), Mutable, Unsigned>,
    token_1_vault: Account<(), Mutable, Unsigned>,
    token_program: Account<(), ReadOnly, Unsigned>,
    token_program_2022: Account<(), ReadOnly, Unsigned>,
    vault_0_mint: Account<(), ReadOnly, Unsigned>,
    vault_1_mint: Account<(), ReadOnly, Unsigned>,
    lp_mint: Account<(), Mutable, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Withdraw {
    pub lp_token_amount: u64,
    pub minimum_token_0_amount: u64,
    pub minimum_token_1_amount: u64,
}
impl Withdraw {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8,
    ];
}
pub struct WithdrawAccounts {
    owner: Account<(), ReadOnly, Signed>,
    authority: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    owner_lp_token: Account<(), Mutable, Unsigned>,
    token_0_account: Account<(), Mutable, Unsigned>,
    token_1_account: Account<(), Mutable, Unsigned>,
    token_0_vault: Account<(), Mutable, Unsigned>,
    token_1_vault: Account<(), Mutable, Unsigned>,
    token_program: Account<(), ReadOnly, Unsigned>,
    token_program_2022: Account<(), ReadOnly, Unsigned>,
    vault_0_mint: Account<(), ReadOnly, Unsigned>,
    vault_1_mint: Account<(), ReadOnly, Unsigned>,
    lp_mint: Account<(), Mutable, Unsigned>,
    memo_program: Account<(), ReadOnly, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct SwapBaseInput {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
impl SwapBaseInput {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        235u8, 33u8, 127u8, 212u8, 246u8, 180u8, 129u8, 99u8,
    ];
}
pub struct SwapBaseInputAccounts {
    payer: Account<(), ReadOnly, Signed>,
    authority: Account<(), ReadOnly, Unsigned>,
    amm_config: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    input_token_account: Account<(), Mutable, Unsigned>,
    output_token_account: Account<(), Mutable, Unsigned>,
    input_vault: Account<(), Mutable, Unsigned>,
    output_vault: Account<(), Mutable, Unsigned>,
    input_token_program: Account<(), ReadOnly, Unsigned>,
    output_token_program: Account<(), ReadOnly, Unsigned>,
    input_token_mint: Account<(), ReadOnly, Unsigned>,
    output_token_mint: Account<(), ReadOnly, Unsigned>,
    observation_state: Account<(), Mutable, Unsigned>,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct SwapBaseOutput {
    pub max_amount_in: u64,
    pub amount_out: u64,
}
impl SwapBaseOutput {
    pub const DISCRIMINATOR: [u8; 8usize] = [
        46u8, 155u8, 21u8, 238u8, 193u8, 118u8, 150u8, 246u8,
    ];
}
pub struct SwapBaseOutputAccounts {
    payer: Account<(), ReadOnly, Signed>,
    authority: Account<(), ReadOnly, Unsigned>,
    amm_config: Account<(), ReadOnly, Unsigned>,
    pool_state: Account<(), Mutable, Unsigned>,
    input_token_account: Account<(), Mutable, Unsigned>,
    output_token_account: Account<(), Mutable, Unsigned>,
    input_vault: Account<(), Mutable, Unsigned>,
    output_vault: Account<(), Mutable, Unsigned>,
    input_token_program: Account<(), ReadOnly, Unsigned>,
    output_token_program: Account<(), ReadOnly, Unsigned>,
    input_token_mint: Account<(), ReadOnly, Unsigned>,
    output_token_mint: Account<(), ReadOnly, Unsigned>,
    observation_state: Account<(), Mutable, Unsigned>,
}
pub struct AmmConfig {
    pub bump: u8,
    pub disable_create_pool: bool,
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub protocol_owner: [u8; 32],
    pub fund_owner: [u8; 32],
    pub padding: [u64; 16usize],
}
pub struct ObservationState {
    pub initialized: bool,
    pub observation_index: u16,
    pub pool_id: [u8; 32],
    pub observations: [Observation; 100usize],
    pub padding: [u64; 4usize],
}
pub struct PoolState {
    pub amm_config: [u8; 32],
    pub pool_creator: [u8; 32],
    pub token_0_vault: [u8; 32],
    pub token_1_vault: [u8; 32],
    pub lp_mint: [u8; 32],
    pub token_0_mint: [u8; 32],
    pub token_1_mint: [u8; 32],
    pub token_0_program: [u8; 32],
    pub token_1_program: [u8; 32],
    pub observation_key: [u8; 32],
    pub auth_bump: u8,
    pub status: u8,
    pub lp_mint_decimals: u8,
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,
    pub lp_supply: u64,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub padding: [u64; 31usize],
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct LpChangeEvent {
    pub pool_id: [u8; 32],
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct SwapEvent {
    pub pool_id: [u8; 32],
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Observation {
    pub block_timestamp: u64,
    pub cumulative_token_0_price_x_32: u128,
    pub cumulative_token_1_price_x_32: u128,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum TradeDirection {
    ZeroForOne,
    OneForZero,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum RoundDirection {
    Floor,
    Ceiling,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum PoolStatusBitIndex {
    Deposit,
    Withdraw,
    Swap,
}
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub enum PoolStatusBitFlag {
    Enable,
    Disable,
}
