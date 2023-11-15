use crate::model::{Enum, Map, Record, Type};

#[derive(Debug)]
pub struct TypeScriptDecls {
    pub interfaces: Vec<String>,
    pub enumerations: Vec<String>,
}

// Returns the type name of the last type
pub fn convert_schema_to_typescript(schema: Type, decls: &mut TypeScriptDecls) -> String {
    match schema {
        Type::Record(record) => {
            let return_name = record.name.clone();

            let record = convert_record(*record, decls);

            decls.interfaces.push(record);

            return_name
        }
        Type::Enum(e) => {
            let return_name = e.name.clone();

            decls.enumerations.push(convert_enum(e));

            return_name
        }
        Type::Boolean => "boolean".to_string(),
        Type::Bytes => "UInt8Array".to_string(),
        Type::Double | Type::Float | Type::Int | Type::Long => "number".to_string(),
        Type::Null => "null".to_string(),
        Type::String => "string".to_string(),
        Type::Union(union) => convert_union(union, decls),
        Type::Array(_) => todo!(),
        Type::Map(m) => convert_map(*m, decls),
        Type::Fixed(_) => todo!(),
    }
}

pub fn convert_record(v: Record, decls: &mut TypeScriptDecls) -> String {
    let Record { name, fields, .. } = v;

    let field_decls = fields
        .iter()
        .filter_map(|f| match Type::try_from(&f.type_name) {
            Ok(v) => {
                let typename = convert_schema_to_typescript(v, decls);
                let name = &f.name;

                Some(format!("\t{name}: {typename};\n"))
            }
            Err(_e) => None,
        })
        .collect::<String>();

    format!("export interface {name} {{ \n{field_decls} \n}}")
}

pub fn convert_enum(v: Enum) -> String {
    let Enum { name, symbols, .. } = v;

    let symbol_decls = symbols
        .iter()
        .map(|sym| format!("\t{sym} = \"{sym}\""))
        .collect::<Vec<String>>()
        .join(",\n");

    format!("export enum {name}{{ \n{symbol_decls} \n}}")
}

pub fn convert_union(u: Vec<Type>, decls: &mut TypeScriptDecls) -> String {
    let mut v = vec![];

    for t in u {
        let name = convert_schema_to_typescript(t, decls);

        v.push(name)
    }

    v.join(" | ")
}

pub fn convert_map(m: Map, decls: &mut TypeScriptDecls) -> String {
    let value_type = Type::try_from(&m.values).unwrap_or(Type::Null);

    let name = convert_schema_to_typescript(value_type, decls);

    format!("{{ [index:string]: {name} }}")
}
