use core::panic;
use std::{path::{Path}, fs::{self, File}, io::Write};

pub mod parser;
pub mod pub_sub_server;
use gtmpl_derive::Gtmpl;

#[derive(Gtmpl)]
struct AsyncAPIDocument {
        version : String
}


//parse file to json, allowed files are yaml and json
fn parse_test(path : &Path) -> serde_json::Value {
    let string_content = fs::read_to_string(path).expect("file could not be read");
    // check if file is yaml or json
    let parsed: serde_json::Value = match path.extension() {
        Some(ext) => {
            match ext.to_str() {
                Some("yaml") => {
                    serde_yaml::from_str::<serde_json::Value>(&string_content).unwrap()
                },
                Some("json") => {
                    serde_json::from_str::<serde_json::Value>(&string_content).unwrap()
                },
                _ => {
                    panic!("file has no extension");
                }
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
    let async_config = parse_test(Path::new("./example/userSignupPublisher.yaml"));
    let types_path = Path::new("./generated/types");
    parser::extract_types::json_schema_to_file(&async_config["components"]["schemas"]["userSignedUpPayload"], types_path, Some("userSignedUpPayload")).unwrap();
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



fn read_file(path: &Path) -> Vec<u8>{
    // read file in UTF-8
    let file_content = fs::read(path).expect("file could not be read");
    file_content
}

fn write_file(path :&Path, contents : String) -> (){
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let mut f = File::create(path).expect(&format!("could not creat file {}", path.to_str().unwrap()));
    f.write_all(contents.as_bytes()).expect("could not write to file");
}
