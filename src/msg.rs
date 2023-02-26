use cosmwasm_schema::cw_serde;
use cosmwasm_std::Timestamp;

#[cw_serde]
pub struct CustomExtension {
    pub score: u64,
    pub last_action: Timestamp,
}
