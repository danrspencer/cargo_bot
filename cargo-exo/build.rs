use cargo_exo_functions::explain::ExplainParams;
use cargo_exo_functions::update_files::UpdateFilesParams;
use schemars::gen::SchemaGenerator;
use serde_json::json;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // let schema = SchemaGenerator::
    let mut schema_settings = schemars::gen::SchemaSettings::draft07();
    schema_settings.inline_subschemas = true;

    let schemars = vec![
        (
            "update_files_schema.json",
            SchemaGenerator::new(schema_settings.clone())
                .into_root_schema_for::<UpdateFilesParams>(),
        ),
        (
            "explain_schema.json",
            SchemaGenerator::new(schema_settings).into_root_schema_for::<ExplainParams>(),
        ),
    ];

    for (filename, schema) in schemars {
        let schema_json = json!(schema);

        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join(filename);
        fs::write(dest_path, schema_json.to_string()).unwrap();
    }
}
