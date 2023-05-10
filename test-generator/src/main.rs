use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::{BufReader, Read, self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct AsyncApi {
    asyncapi: String,
    info: Info,
    servers: HashMap<String, Server>,
    channels: Channels,
}

#[derive(Debug, Serialize, Deserialize)]
struct Server {
    url: String,
    protocol: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Info {
    title: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Channels {
    #[serde(flatten)]
    channels: std::collections::HashMap<String, Channel>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Channel {
    subscribe: Option<Operation>,
    publish: Option<Operation>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Operation {
    operation_id: Option<String>,
    summary: String,
    message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    #[serde(rename = "type")]
    data_type: String,
}

fn parse_asyncapi_yaml_file<P: AsRef<Path>>(path: P) -> Result<AsyncApi, serde_yaml::Error> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let asyncapi_spec = serde_yaml::from_reader(reader)?;
    Ok(asyncapi_spec)
}

fn read_template_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_to_output_file<P: AsRef<Path>>(path: P, code: &str) -> Result<(), io::Error> {
    create_dir_all("output")?;
    let mut file = File::create(path)?;
    file.write_all(code.as_bytes())?;
    Ok(())
}

fn main() {
    let asyncapi_yaml = concat!(env!("CARGO_MANIFEST_DIR"), "/spec/basic.yaml");
    let asyncapi_spec = match parse_asyncapi_yaml_file(asyncapi_yaml) {
        Ok(spec) => spec,
        Err(e) => {
            println!("Error parsing AsyncAPI YAML: {:?}", e);
            return;
        }
    };
    
    let template_path = concat!(env!("CARGO_MANIFEST_DIR"), "/templates/basic.txt");
    let template = match read_template_file(template_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Error reading template file: {:?}", e);
            return;
        }
    };
    let server_url = asyncapi_spec.servers.values().next().unwrap().url.clone();
    let channel_name = asyncapi_spec.channels.channels.keys().next().unwrap().clone();

    let code = template
        .replace("{{.ServerURL}}", &server_url)
        .replace("{{.ChannelName}}", &channel_name)
        .replace("{{.TakeCount}}", "10")
        .replace("{{.PublishCount}}", "10")
        .replace("{{.PublishData}}", "data");

    let output_path = concat!(env!("CARGO_MANIFEST_DIR"), "/output/generated.rs");

    if let Err(e) = write_to_output_file(output_path, &code) {
        println!("Error writing generated code to output file: {:?}", e);
    } else {
        println!("Generated code saved to {}", output_path);
    }
}