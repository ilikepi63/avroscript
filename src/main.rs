#[macro_use]
extern crate log;
use serde_json::Value;

use crate::{
    model::Type,
    typescript_conversion::{convert_schema_to_typescript, TypeScriptDecls},
};

pub mod model;
pub mod typescript_conversion;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let src = r#"{
        "type": "record",
        "namespace": "ApAssocTestResult",
        "name": "ApAssocTestResult",
        "fields": [
            {
                "type": "double",
                "name": "elapsed_time_seconds"
              },
              {
                "type": [
                  "null",
                  "double"
                ],
                "name": "json_extra"
              },
              {
                "type": {
                  "type": "enum",
                  "symbols": [
                    "AP_ASSOC",
                    "UNREACHABLE"
                  ],
                  "name": "AP_ASSOC"
                },
                "name": "test_type_code"
              }
            
        ]
      }"#;

    info!("Converting!");

    let value: Value = serde_json::from_str(src)?;

    let mut decls = TypeScriptDecls {
        interfaces: vec![],
        enumerations: vec![],
    };

    let schema = Type::try_from(&value)?;

    let generated = convert_schema_to_typescript(schema, &mut decls);

    let interfaces = decls.interfaces.join("\n");
    let enumerations = decls.enumerations.join("\n");
    let _ = std::fs::write("generated.ts", format!("{interfaces}\n\n{enumerations}"));

    Ok(())
}
