use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// DctBalance  holds information about the dct balance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DctBalance {
    pub token_identifier: String,
    pub balance: String,
}

// DctBalanceDataholds the dct balance data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DctBalanceData {
    pub dcts: HashMap<String, DctBalance>,
}

// DctBalanceResponse holds the dct balance endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DctBalanceResponse {
    pub data: Option<DctBalanceData>,
    pub error: String,
    pub code: String,
}

// DctRolesData holds the dct roles data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DctRolesData {
    pub roles: HashMap<String, Vec<String>>,
}

// DctBalanceResponse holds the dct roles endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DctRolesResponse {
    pub data: Option<DctRolesData>,
    pub error: String,
    pub code: String,
}
