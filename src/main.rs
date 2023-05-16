use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    println,
};

pub mod model;
pub mod parser;
pub mod pub_sub_server;
use gtmpl_derive::Gtmpl;

#[derive(Gtmpl)]
struct AsyncAPIDocument {
    version: String,
}

//parse file to json, allowed files are yaml and json
fn parse_test(path: &Path) -> model::AsyncAPI {
    let string_content = fs::read_to_string(path).expect("file could not be read");
    // check if file is yaml or json
    let parsed = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("yaml") => serde_yaml::from_str::<model::AsyncAPI>(&string_content).unwrap(),
            Some("json") => serde_json::from_str::<model::AsyncAPI>(&string_content).unwrap(),
            _ => {
                panic!("file has no extension");
            }
        },
        None => {
            panic!("file has no extension");
        }
    };
    parsed
}

#[tokio::main]
async fn main() {
    //parse configuration
    let async_config = parse_test(Path::new("./example/userSignupSubscriber.yaml"));
    let types_path: &Path = Path::new("./generated/types");

    // iterate over channels
    for (name, state) in async_config.channels {
        println!("{:?}", name);
        println!("{:?}", state);
        // check what
    }

    // parser::extract_types::json_schema_to_file(&async_config["payload"], types_path, Some("payload")).unwrap();

    // let async_config = parse_test(Path::new("./example/userSignupPublisher.yaml"));
    // parser::extract_types::json_schema_to_file(&async_config["components"]["schemas"]["userSignedUpPayload"], types_path, Some("userSignedUpPayload")).unwrap();

    // parse correct templates with config as context with gtmpl
    // let path = Path::new("./templates/publisher_template.tmpl");
    // let contents: Vec<u8> = read_file(path);
    // let string_content = std::str::from_utf8(&contents).expect("could not read file into string");
    // println!("{:?}", string_content);
    // let template_result = gtmpl::template(string_content, async_config).expect("Could not inject template");
    // println!("{:?}", template_result);
    // // embed resulting code
    // // write code into correct file structure
    // let result_path = Path::new("./generated/publisher_template.rs");
    // write_file(result_path, template_result);
}

fn read_file(path: &Path) -> Vec<u8> {
    // read file in UTF-8
    let file_content = fs::read(path).expect("file could not be read");
    file_content
}

pub fn write_file(path: &Path, contents: String) -> () {
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let mut f =
        File::create(path).expect(&format!("could not creat file {}", path.to_str().unwrap()));
    f.write_all(contents.as_bytes())
        .expect("could not write to file");
}
