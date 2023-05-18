use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GreetResp, InstantiateMsg, QueryMsg};
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
    match msg {
        QueryMsg::Greet {} => to_binary(&query::greet()?),
        QueryMsg::AdminList {} => to_binary(&query::admin_list(deps)?),
    }
}

pub fn execute(
    deps: DepsMut,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddMembers { admins } => exec::add_members(deps, info, admins),
        ExecuteMsg::Leave {} => exec::leave(deps, info).map_err(Into::into),
    }
}

mod exec {
    use cosmwasm_std::{DepsMut, Event, MessageInfo, Response, StdResult};

    use crate::error::ContractError;
    use crate::state::ADMINS;

    pub fn add_members(
        deps: DepsMut,
        info: MessageInfo,
        admins: Vec<String>,
    ) -> Result<Response, ContractError> {
        let mut current_admins = ADMINS.load(deps.storage)?;
        if !current_admins.contains(&info.sender) {
            return Err(ContractError::Unauthorized {
                sender: info.sender,
            });
        }
        let events = admins
            .iter()
            .map(|admin| Event::new("admin_added").add_attribute("addr", admin));
        let resp = Response::new()
            .add_events(events)
            .add_attribute("action", "add_members")
            .add_attribute("added_count", admins.len().to_string());
        let admins: StdResult<Vec<_>> = admins
            .into_iter()
            .map(|addr| deps.api.addr_validate(&addr))
            .collect();
        current_admins.append(&mut admins?);
        ADMINS.save(deps.storage, &current_admins)?;

        Ok(resp)
    }

    pub fn leave(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
        ADMINS.update(deps.storage, move |admins| -> StdResult<_> {
            let admins = admins
                .into_iter()
                .filter(|admin| *admin != info.sender)
                .collect();
            Ok(admins)
        })?;
        Ok(Response::new())
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
