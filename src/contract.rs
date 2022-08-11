use std::borrow::Borrow;

use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, QueryAnswer, HandleAnswer};
use crate::state::{CONFIG_KEY,Owner,State, save, load, Keydetail};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
 let msgg=msg.magicalid;
 let config = Owner{
    magicalid: msgg
}; 

save(&mut deps.storage, CONFIG_KEY, &config)?;
Ok(InitResponse::default()) 
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
match msg{
    HandleMsg::AddEncryptionKey { mid, ket, Jwt } => try_record(deps, env, mid,ket,Jwt),  
    HandleMsg::GenerateTempOwner { magicalid, JWTtoken, timestamp } => todo!(),
    HandleMsg::VoteForAuth { magicalid,JWTtoken:String, timestamp } => todo!(),
    HandleMsg::GetKey { magicalid } => todo!(),
    HandleMsg::Owner{}=>!todo!()

}
}
fn try_record<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    mid: String,
    ket:String,
    Jwt:String
) -> StdResult<HandleResponse> {
    let status: String;
    let Jwttoke = Jwt.clone().as_bytes();
    let magicali=mid.as_bytes();
    let keyentrop=ket.as_bytes();
        // get the canonical address of sender
        let sender_address = deps.api.canonical_address(&env.message.sender)?;

        // create the reminder struct containing content string and timestamp
        let keydetail = Keydetail{
        magicalid:mid.clone(),
        Jwttoken:Jwt.clone(),
        key:ket,
        timeStamp:Some(env.block.time),
        numvote:0,
        };

        // save the reminder using a byte vector representation of the sender's address as the key
        save(&mut deps.storage, &magicali, &keydetail)?;

    
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::AddEncryptionKey{
            status:true,
            error:false,
            msg:String::from("Added key"),
            Jwttoken:Jwt.clone()
        })?),
    })
}
pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
   match msg{
        QueryMsg::Owner {}=> {
            let config: Owner = load(&deps.storage, CONFIG_KEY)?;
            to_binary(&QueryAnswer::OwnerDetail{ owner: config.magicalid })
          }
       
        
    }
}
