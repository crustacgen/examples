use std::{path::{Path}, fs::{self, File}, os::unix::prelude::FileExt, io::Write};

use crate::parser::parse_yaml;
pub mod parser;
pub mod pub_sub_server;
use gtmpl_derive::Gtmpl;

#[derive(Gtmpl)]
struct AsyncAPIDocument {
        version : String
}
fn parse_test() -> AsyncAPIDocument {
    let parsed: serde_yaml::Mapping =
        parse_yaml::parse_yaml("example/userSignupPublisher.yaml").unwrap();
    println!("{:?}", parsed);
    let version: String = parsed.get("asyncapi").expect("Version is required").as_str().expect("Version has to be string").to_owned();
    AsyncAPIDocument {
        version: version
    }
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


#[tokio::main]
async fn main() {
    //parse configuration 
    let async_config = parse_test();
    
    // parse correct templates with config as context with gtmpl
    let path = Path::new("./templates/publisher_template.tmpl");
    let contents: Vec<u8> = read_file(path);
    let string_content = std::str::from_utf8(&contents).expect("could not read file into string");
    println!("{:?}", string_content);
    let template_result = gtmpl::template(string_content, async_config).expect("Could not inject template");
    println!("{:?}", template_result);
    // embed resulting code
    // write code into correct file structure
    let result_path = Path::new("./generated/publisher_template.rs");
    write_file(result_path, template_result);
}

