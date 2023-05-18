use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum QueryMsg {
    Greet {},
    AdminList {},
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct AdminListResp {
    pub admins: Vec<Addr>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum ExecuteMsg {
    AddMembers { admins: Vec<String> },
    Leave {},
}
