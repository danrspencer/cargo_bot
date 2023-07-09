use cargo_bot_params::update_files::UpdateFilesArgs;
use schemars::schema_for;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let schema = schema_for!(UpdateFilesArgs);
    let schema_json = json!(schema);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("update_files_args_schema.json");
    fs::write(&dest_path, schema_json.to_string()).unwrap();
}
