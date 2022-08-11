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
  },
  VoteForAuth{
    magicalid:String,
    ucpiJWTtoken:String,
    timestamp: Option<u64>
  },
  GetKey{
    magicalid:String
  },
  Owner{
  },

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
      ucpi_jwttoken:String     
    },
    /// Return a status message and the current reminder and its timestamp, if it exists
    GenerateTempOwner {
        status: bool,
        address:String,
        timestamp: Option<u64>,
        ucpi_jwttoken:String,
        error:bool
    },
    VoteForAuth{
        status:bool,
        error:bool,
        ucpi_jwttoken:String,
        msg:String
    },
    GetKey{
        status:bool,
        error:bool,
        msg:String,
        key:String
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Gets basic statistics about the use of the contract
    Owner{ }
}
/// Responses from query functions
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    /// Return basic statistics about contract
    OwnerDetail {
        owner: String,
    },
    Ownerprint{
      magicalid:String
    }
  }
// // We define a custom struct for each query response
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct CountResponse {
//     pub count: i32,
// }
