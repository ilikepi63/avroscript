use clap::Parser;
use serde_json::Value;

use crate::{
    cli::Args,
    model::Type,
    typescript_conversion::{convert_schema_to_typescript, TypeScriptDecls},
};

pub mod cli;
pub mod model;
pub mod typescript_conversion;

fn main() -> anyhow::Result<()> {
    let Args { mut output, target } = cli::Args::parse();

    if target.is_dir() {
        for entry in (target.read_dir()?).flatten() {
            if entry.path().extension().is_some_and(|s| s == "avsc") {
                let result = std::fs::read_to_string(entry.path())?;

                let (name, body) = convert_str(&result)?;

                let mut cloned_output = output.clone();

                cloned_output.push(name);
                cloned_output.set_extension("ts");

                std::fs::write(cloned_output, body)?;
            }
        }
    } else {
        let result = std::fs::read_to_string(target)?;

        let (name, body) = convert_str(&result)?;

        output.push(name);
        output.set_extension(".ts");

        std::fs::write(output, body)?;
    }

    Ok(())
}

fn convert_str(src: &str) -> anyhow::Result<(String, String)> {
    let value: Value = serde_json::from_str(src)?;

    let mut decls = TypeScriptDecls {
        interfaces: vec![],
        enumerations: vec![],
    };

    let schema = Type::try_from(&value)?;

    let generated = convert_schema_to_typescript(schema, &mut decls);

    let interfaces = decls.interfaces.join("\n");
    let enumerations = decls.enumerations.join("\n");

    Ok((generated, format!("{interfaces}\n\n{enumerations}")))
}
