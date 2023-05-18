use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use error::ContractError;
use msg::ExecuteMsg;

mod contract;
mod error;
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
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    contract::execute(deps, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<Binary> {
    contract::query(deps, env, msg)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, Addr};
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
                &msg::InstantiateMsg {
                    admins: vec![],
                    donation_denom: "cosmos".to_owned(),
                },
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
                &InstantiateMsg {
                    admins: vec![],
                    donation_denom: "cosmos".to_owned(),
                },
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
                    donation_denom: "cosmos".to_owned(),
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

    #[test]
    fn unauthorized() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec![],
                    donation_denom: "cosmos".to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();
        let err = app
            .execute_contract(
                Addr::unchecked("user"),
                addr,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::Unauthorized {
                sender: Addr::unchecked("user")
            },
            err.downcast().unwrap()
        );
    }

    #[test]
    fn add_members() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["owner".to_owned()],
                    donation_denom: "cosmos".to_owned(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();
        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr.clone(),
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap();
        let wasm_event = resp.events.iter().find(|ev| ev.ty == "wasm").unwrap();
        assert_eq!(
            wasm_event
                .attributes
                .iter()
                .find(|attr| attr.key == "action")
                .unwrap()
                .value,
            "add_members"
        );
        assert_eq!(
            wasm_event
                .attributes
                .iter()
                .find(|attr| attr.key == "added_count")
                .unwrap()
                .value,
            "1"
        );
        let admin_added: Vec<_> = resp
            .events
            .iter()
            .filter(|ev| ev.ty == "wasm-admin_added")
            .collect();
        assert_eq!(admin_added.len(), 1);
        assert_eq!(
            admin_added[0]
                .attributes
                .iter()
                .find(|attr| attr.key == "addr")
                .unwrap()
                .value,
            "user"
        );
        let resp = app
            .execute_contract(
                Addr::unchecked("owner"),
                addr,
                &ExecuteMsg::AddMembers {
                    admins: vec!["user".to_owned()],
                },
                &[],
            )
            .unwrap_err();
        assert_eq!(
            ContractError::AdminAlreadyExists {
                admin: Addr::unchecked("user")
            },
            resp.downcast().unwrap()
        );
    }

    #[test]
    fn donations() {
        let funds = coins(5, "cosmos");
        let mut app = App::new(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked("user"), funds.clone())
                .unwrap();
        });

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &InstantiateMsg {
                    admins: vec!["admin1".to_owned(), "admin2".to_owned()],
                    donation_denom: "cosmos".to_string(),
                },
                &[],
                "Contract",
                None,
            )
            .unwrap();
        app.execute_contract(
            Addr::unchecked("user"),
            addr,
            &ExecuteMsg::Donate {},
            &funds,
        )
        .unwrap();
        assert_eq!(
            app.wrap()
                .query_balance("user", "cosmos")
                .unwrap()
                .amount
                .u128(),
            0
        );
    }
}
