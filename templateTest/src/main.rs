
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;

use rust_embed::RustEmbed;
use tera::{Context, Tera};

#[derive(Debug, Deserialize, Serialize)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    field_type: String,
}

#[derive(Debug, Deserialize)]
struct Specification {
    server: String, 
    subject: String,
    payload: String,
    publish: bool, 
}

#[derive(RustEmbed)]
#[folder = "src/templates/"]
struct Templates;

fn main() {

    // Load the specification file
    let specification_file =
        std::fs::read_to_string("./src/spec.yaml").expect("Failed to read specification file");
    
    println!("read spec as String: {}", specification_file);


    // Parse the specification into a Rust struct
    let specification: Specification =
        serde_yaml::from_str(&specification_file).expect("Failed to parse specification file");

    // Create a Tera context with the specification fields
    let mut context = Context::new();
    context.insert("server", &specification.server);
    context.insert("subject", &specification.subject);
    context.insert("payload", &specification.payload);
    context.insert("publish", &specification.publish);


    // Render a template located in the `templates` folder
    let template = Templates::get("./pub.rs.tera").unwrap();

    let template_content = String::from_utf8_lossy(&template.data);

    let tera = Tera::one_off(&template_content, &context, false).unwrap();
        

    // Write the generated Rust code to a file
    let mut out_file =
        File::create("out.rs").expect("Failed to create output Rust file");
    out_file
        .write_all(tera.as_bytes())
        .expect("Failed to write to output Rust file");
}