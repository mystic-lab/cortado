use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    // owner and creator come from env
    // collateral comes from env
    pub counter_offer: Vec<Coin>,
    pub expires: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Owner can transfer to a new owner
    Transfer { recipient: String },
    /// Owner can post counter_offer on unexpired option to execute and get the collateral
    Execute {},
    /// Burn will release collateral if expired
    Burn {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
}
