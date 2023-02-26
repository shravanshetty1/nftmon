use std::{env::current_dir, fs::create_dir_all};

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use cosmwasm_std::Empty;
use cw721_base::{ExecuteMsg, InstantiateMsg, QueryMsg};
use nftmon::msg::CustomExtension;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg<CustomExtension,Empty>), &out_dir);
    export_schema(&schema_for!(QueryMsg<Empty>), &out_dir);
}
