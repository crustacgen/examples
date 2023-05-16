use std::{error::Error, path::Path};
use schemars::schema::RootSchema;
use typify::TypeSpace;


pub fn json_schema_to_file(json: &serde_json::Value, out_dir : &Path, title : Option<&str>) -> Result<(), Box<dyn Error>>{
    let type_space = json_schema_to_types(json, title)?;
    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
    );
    //create parent dir if not exists
    std::fs::create_dir_all(out_dir).unwrap();

    let mut out_file = out_dir.to_path_buf();
    out_file.push("lib.rs");
    std::fs::write(out_file, contents).unwrap();
    Ok(())
}

// parsed a json schema to rust types
pub fn json_schema_to_types(json: &serde_json::Value, title : Option<&str>) -> Result<TypeSpace, Box<dyn Error>>{
    let json_string = serde_json::to_string_pretty(&json).unwrap();
    let mut schema : schemars::schema::RootSchema = match serde_json::from_value(json.clone()) {
        Ok(schema) => schema,
        Err(e) => {
            println!("The given json: {} is not a valid schema: {}", json_string,e);
            return Err(Box::new(e));
        }
    };
    if title.is_some() {
            let s: &mut RootSchema = &mut schema; 
            let mut metadata: &mut schemars::schema::Metadata =s.schema.metadata();
            metadata.title = Some(title.unwrap().to_string());
    }
    let mut type_space = typify::TypeSpace::new(typify::TypeSpaceSettings::default().with_struct_builder(false));
    type_space.add_root_schema(schema).unwrap();
    Ok(type_space)
}