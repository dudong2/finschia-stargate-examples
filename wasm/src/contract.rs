#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmwasm::wasm::v1::{
    AccessConfig, MsgClearAdmin, MsgExecuteContract, MsgIbcCloseChannel, MsgIbcSend,
    MsgInstantiateContract, MsgInstantiateContract2, MsgMigrateContract, MsgStoreCode,
    MsgUpdateAdmin, WasmQuerier,
};
use finschia_std::types::cosmwasm::wasm::v1::{
    QueryAllContractStateResponse, QueryCodeResponse, QueryCodesResponse,
    QueryContractHistoryResponse, QueryContractInfoResponse, QueryContractsByCodeResponse,
    QueryParamsResponse, QueryPinnedCodesResponse, QueryRawContractStateResponse,
    QuerySmartContractStateResponse,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:finschia-stargate-wasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::StoreCode {
            sender,
            wasm_byte_code,
            instantiate_permission,
        } => try_store_code(deps, info, sender, wasm_byte_code, instantiate_permission),
        ExecuteMsg::InstantiateContract {
            sender,
            admin,
            code_id,
            label,
            msg,
            funds,
        } => try_instantiate_contract(deps, info, sender, admin, code_id, label, msg, funds),
        ExecuteMsg::InstantiateContract2 {
            sender,
            admin,
            code_id,
            label,
            msg,
            funds,
            salt,
            fix_msg,
        } => try_instantiate_contract2(
            deps, info, sender, admin, code_id, label, msg, funds, salt, fix_msg,
        ),
        ExecuteMsg::ExecuteContract {
            sender,
            contract,
            msg,
            funds,
        } => try_execute_contract(deps, info, sender, contract, msg, funds),
        ExecuteMsg::MigrateContract {
            sender,
            contract,
            code_id,
            msg,
        } => try_migrate_contract(deps, info, sender, contract, code_id, msg),
        ExecuteMsg::UpdateAdmin {
            sender,
            new_admin,
            contract,
        } => try_update_admin(deps, info, sender, new_admin, contract),
        ExecuteMsg::ClearAdmin { sender, contract } => {
            try_clear_admin(deps, info, sender, contract)
        }
        ExecuteMsg::IbcSend {
            channel,
            timeout_height,
            timeout_timestamp,
            data,
        } => try_ibc_send(deps, info, channel, timeout_height, timeout_timestamp, data),
        ExecuteMsg::IbcCloseChannel { channel } => try_ibc_close_channel(deps, info, channel),
    }
}

pub fn try_store_code(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    wasm_byte_code: Vec<u8>,
    instantiate_permission: Option<AccessConfig>,
) -> Result<Response, ContractError> {
    let msg_store_code: CosmosMsg = MsgStoreCode {
        sender,
        wasm_byte_code,
        instantiate_permission,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_store_code")
        .add_message(msg_store_code))
}

pub fn try_instantiate_contract(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    admin: String,
    code_id: u64,
    label: String,
    msg: Vec<u8>,
    funds: Vec<Coin>,
) -> Result<Response, ContractError> {
    let msg_instantiate_contract: CosmosMsg = MsgInstantiateContract {
        sender,
        admin,
        code_id,
        label,
        msg,
        funds,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_instantiate_contract")
        .add_message(msg_instantiate_contract))
}

pub fn try_instantiate_contract2(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    admin: String,
    code_id: u64,
    label: String,
    msg: Vec<u8>,
    funds: Vec<Coin>,
    salt: Vec<u8>,
    fix_msg: bool,
) -> Result<Response, ContractError> {
    let msg_instantiate_contract2: CosmosMsg = MsgInstantiateContract2 {
        sender,
        admin,
        code_id,
        label,
        msg,
        funds,
        salt,
        fix_msg,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_instantiate_contract2")
        .add_message(msg_instantiate_contract2))
}

pub fn try_execute_contract(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    contract: String,
    msg: Vec<u8>,
    funds: Vec<Coin>,
) -> Result<Response, ContractError> {
    let msg_execute_contract: CosmosMsg = MsgExecuteContract {
        sender,
        contract,
        msg,
        funds,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_execute_contract")
        .add_message(msg_execute_contract))
}

pub fn try_migrate_contract(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    contract: String,
    code_id: u64,
    msg: Vec<u8>,
) -> Result<Response, ContractError> {
    let msg_migrate_contract: CosmosMsg = MsgMigrateContract {
        sender,
        contract,
        code_id,
        msg,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_migrate_contract")
        .add_message(msg_migrate_contract))
}

pub fn try_update_admin(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    new_admin: String,
    contract: String,
) -> Result<Response, ContractError> {
    let msg_update_admin: CosmosMsg = MsgUpdateAdmin {
        sender,
        new_admin,
        contract,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_update_admin")
        .add_message(msg_update_admin))
}

pub fn try_clear_admin(
    _deps: DepsMut,
    _info: MessageInfo,
    sender: String,
    contract: String,
) -> Result<Response, ContractError> {
    let msg_clear_admin: CosmosMsg = MsgClearAdmin { sender, contract }.into();

    Ok(Response::new()
        .add_attribute("method", "try_clear_admin")
        .add_message(msg_clear_admin))
}

pub fn try_ibc_send(
    _deps: DepsMut,
    _info: MessageInfo,
    channel: String,
    timeout_height: u64,
    timeout_timestamp: u64,
    data: Vec<u8>,
) -> Result<Response, ContractError> {
    let msg_ibc_send: CosmosMsg = MsgIbcSend {
        channel,
        timeout_height,
        timeout_timestamp,
        data,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_ibc_send")
        .add_message(msg_ibc_send))
}

pub fn try_ibc_close_channel(
    _deps: DepsMut,
    _info: MessageInfo,
    channel: String,
) -> Result<Response, ContractError> {
    let msg_ibc_close_channel: CosmosMsg = MsgIbcCloseChannel { channel }.into();

    Ok(Response::new()
        .add_attribute("method", "try_ibc_close_channel")
        .add_message(msg_ibc_close_channel))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Code { code_id } => to_binary(&query_code(deps, code_id)?),
        QueryMsg::Codes { pagination } => to_binary(&query_codes(deps, pagination)?),
        QueryMsg::PinnedCodes { pagination } => to_binary(&query_pinned_codes(deps, pagination)?),
        QueryMsg::ContractInfo { address } => to_binary(&query_contract_info(deps, address)?),
        QueryMsg::ContractHistory {
            address,
            pagination,
        } => to_binary(&query_contract_history(deps, address, pagination)?),
        QueryMsg::ContractsByCode {
            code_id,
            pagination,
        } => to_binary(&query_contracts_by_code(deps, code_id, pagination)?),
        QueryMsg::AllContractState {
            address,
            pagination,
        } => to_binary(&query_all_contract_state(deps, address, pagination)?),
        QueryMsg::RawContractState {
            address,
            query_data,
        } => to_binary(&query_raw_contract_state(deps, address, query_data)?),
        QueryMsg::SmartContractState {
            address,
            query_data,
        } => to_binary(&query_smart_contract_state(deps, address, query_data)?),
        QueryMsg::Params {} => to_binary(&query_params(deps)?),
    }
}

fn query_code(deps: Deps, code_id: u64) -> StdResult<QueryCodeResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.code(code_id)?;
    Ok(QueryCodeResponse {
        code_info: res.code_info,
        data: res.data,
    })
}

fn query_codes(deps: Deps, pagination: Option<PageRequest>) -> StdResult<QueryCodesResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.codes(pagination)?;
    Ok(QueryCodesResponse {
        code_infos: res.code_infos,
        pagination: res.pagination,
    })
}

fn query_pinned_codes(
    deps: Deps,
    pagination: Option<PageRequest>,
) -> StdResult<QueryPinnedCodesResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.pinned_codes(pagination)?;
    Ok(QueryPinnedCodesResponse {
        code_ids: res.code_ids,
        pagination: res.pagination,
    })
}

fn query_contract_info(deps: Deps, address: String) -> StdResult<QueryContractInfoResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.contract_info(address)?;
    Ok(QueryContractInfoResponse {
        address: res.address,
        contract_info: res.contract_info,
    })
}

fn query_contract_history(
    deps: Deps,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryContractHistoryResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.contract_history(address, pagination)?;
    Ok(QueryContractHistoryResponse {
        entries: res.entries,
        pagination: res.pagination,
    })
}

fn query_contracts_by_code(
    deps: Deps,
    code_id: u64,
    pagination: Option<PageRequest>,
) -> StdResult<QueryContractsByCodeResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.contracts_by_code(code_id, pagination)?;
    Ok(QueryContractsByCodeResponse {
        contracts: res.contracts,
        pagination: res.pagination,
    })
}

fn query_all_contract_state(
    deps: Deps,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllContractStateResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.all_contract_state(address, pagination)?;
    Ok(QueryAllContractStateResponse {
        models: res.models,
        pagination: res.pagination,
    })
}

fn query_raw_contract_state(
    deps: Deps,
    address: String,
    query_data: Vec<u8>,
) -> StdResult<QueryRawContractStateResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.raw_contract_state(address, query_data)?;
    Ok(QueryRawContractStateResponse { data: res.data })
}

fn query_smart_contract_state(
    deps: Deps,
    address: String,
    query_data: Vec<u8>,
) -> StdResult<QuerySmartContractStateResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.smart_contract_state(address, query_data)?;
    Ok(QuerySmartContractStateResponse { data: res.data })
}

fn query_params(deps: Deps) -> StdResult<QueryParamsResponse> {
    let wq = WasmQuerier::new(&deps.querier);
    let res = wq.params()?;
    Ok(QueryParamsResponse { params: res.params })
}
