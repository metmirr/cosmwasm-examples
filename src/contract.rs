use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{GreetResp, InstantiateMsg, QueryMsg};
use crate::state::ADMINS;

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<_>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
    ADMINS.save(deps.storage, &admins?)?;
    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;

    match msg {
        Greet {} => to_binary(&query::greet()?),
        AdminList {} => to_binary(&query::admin_list(deps)?),
    }
}

mod query {
    use crate::msg::AdminListResp;

    use super::*;

    pub fn greet() -> StdResult<GreetResp> {
        let resp = GreetResp {
            message: "Hello World".to_owned(),
        };
        Ok(resp)
    }

    pub fn admin_list(deps: Deps) -> StdResult<AdminListResp> {
        let admins = ADMINS.load(deps.storage).unwrap();
        Ok(AdminListResp { admins })
    }
}
