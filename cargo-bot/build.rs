use cargo_bot_functions::update_files::UpdateFilesArgs;
use schemars::gen::SchemaGenerator;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // let schema = SchemaGenerator::
    let mut schema_settings = schemars::gen::SchemaSettings::draft07();
    schema_settings.inline_subschemas = true;

    let schema = SchemaGenerator::new(schema_settings).into_root_schema_for::<UpdateFilesArgs>();
    let schema_json = json!(schema);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("update_files_args_schema.json");
    fs::write(dest_path, schema_json.to_string()).unwrap();
}
