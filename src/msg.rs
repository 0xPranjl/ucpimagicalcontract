use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub magicalid:String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
  AddEncryptionKey{
    magicalid:String,
    keyentropy:String,
    timestamp: Option<u64>,
    ucpiJWTtoken:String,
  },
  GenerateTempOwner{
    magicalid:String,
    ucpiJWTtoken:String,
    timestamp: Option<u64>,
  }
  
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}
/// Responses from handle functions
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
    /// Return a status message to let the user know if it succeeded or failed
    AddEncryptionKey {
      status:bool,
      error:bool,
      msg:String,
      ucpiJWTtoken:String     
    },
    /// Return a status message and the current reminder and its timestamp, if it exists
    GenerateTempOwner {
        status: bool,
        address:String,
        timestamp: Option<u64>,
        ucpiJWTtoken:String
    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
