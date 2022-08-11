use cosmwasm_std::{
    debug_print, to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier,
    StdError, StdResult, Storage,
};

use crate::msg::{HandleMsg, InitMsg, QueryMsg, QueryAnswer};
use crate::state::{CONFIG_KEY,Owner,State, save, load};

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
    HandleMsg::AddEncryptionKey { magicalid, keyentropy, timestamp, ucpiJWTtoken } => todo!(),
    HandleMsg::GenerateTempOwner { magicalid, ucpiJWTtoken, timestamp } => todo!(),
    HandleMsg::VoteForAuth { magicalid, ucpiJWTtoken, timestamp } => todo!(),
    HandleMsg::GetKey { magicalid } => todo!(),
    HandleMsg::Owner{}=>todo!(),
}
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
