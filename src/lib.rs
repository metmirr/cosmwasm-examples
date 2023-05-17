use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

mod contract;
mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: msg::InstantiateMsg,
) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}

#[allow(dead_code)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::msg::{AdminListResp, GreetResp, InstantiateMsg, QueryMsg};

    use super::*;

    #[test]
    fn greet_query() {
        // Represents the virtual blockchain
        let mut app = App::default();

        // Representation of the code that will be uploaded to blockchain
        let code = ContractWrapper::new(execute, instantiate, query);
        // Store the object on the blockchain
        let code_id = app.store_code(Box::new(code));
        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &msg::InstantiateMsg { admins: vec![] },
                &[],
                "Contract",
                None,
            )
            .unwrap();
        let resp: GreetResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::Greet {})
            .unwrap();
        assert_eq!(
            resp,
            GreetResp {
                message: "Hello World".to_string()
            }
        );
    }

    #[test]
    fn instantiation() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg { admins: vec![] },
                &[],
                "Contract",
                None,
            )
            .unwrap();
        let resp: AdminListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminList {})
            .unwrap();
        assert_eq!(resp, AdminListResp { admins: vec![] });

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                },
                &[],
                "Contract 2",
                None,
            )
            .unwrap();
        let resp: AdminListResp = app
            .wrap()
            .query_wasm_smart(addr, &QueryMsg::AdminList {})
            .unwrap();
        assert_eq!(
            resp,
            AdminListResp {
                admins: vec![Addr::unchecked("admin1"), Addr::unchecked("admin2")],
            }
        );
    }
}
