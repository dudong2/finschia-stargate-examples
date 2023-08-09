use cosmwasm_schema::cw_serde;
use finschia_std::types::cosmos::base::query::v1beta1::PageRequest;
use finschia_std::types::cosmos::base::v1beta1::Coin;
use finschia_std::types::cosmwasm::wasm::v1::AccessConfig;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    StoreCode {
        sender: String,
        wasm_byte_code: Vec<u8>,
        instantiate_permission: Option<AccessConfig>,
    },
    InstantiateContract {
        sender: String,
        admin: String,
        code_id: u64,
        label: String,
        msg: Vec<u8>,
        funds: Vec<Coin>,
    },
    InstantiateContract2 {
        sender: String,
        admin: String,
        code_id: u64,
        label: String,
        msg: Vec<u8>,
        funds: Vec<Coin>,
        salt: Vec<u8>,
        fix_msg: bool,
    },
    ExecuteContract {
        sender: String,
        contract: String,
        msg: Vec<u8>,
        funds: Vec<Coin>,
    },
    MigrateContract {
        sender: String,
        contract: String,
        code_id: u64,
        msg: Vec<u8>,
    },
    UpdateAdmin {
        sender: String,
        new_admin: String,
        contract: String,
    },
    ClearAdmin {
        sender: String,
        contract: String,
    },
    IbcSend {
        channel: String,
        timeout_height: u64,
        timeout_timestamp: u64,
        data: Vec<u8>,
    },
    IbcCloseChannel {
        channel: String,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Code {
        code_id: u64,
    },
    Codes {
        pagination: Option<PageRequest>,
    },
    PinnedCodes {
        pagination: Option<PageRequest>,
    },
    ContractInfo {
        address: String,
    },
    ContractHistory {
        address: String,
        pagination: Option<PageRequest>,
    },
    ContractsByCode {
        code_id: u64,
        pagination: Option<PageRequest>,
    },
    AllContractState {
        address: String,
        pagination: Option<PageRequest>,
    },
    RawContractState {
        address: String,
        query_data: Vec<u8>,
    },
    SmartContractState {
        address: String,
        query_data: Vec<u8>,
    },
    Params {},
}
