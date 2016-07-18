extern crate handlebars;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::io::Read;

use handlebars::{Handlebars, Context};
use rustc_serialize::json::Json;

fn get_json_or_panic(file_name: &str) -> Json {

    let mut data = String::new();

    // Open the file.
    let mut source_json = match File::open(&file_name) {
        Ok(e) => e,
        Err(_) => panic!("Cannot open context file."),
    };

    // Read the file to the data string.
    match source_json.read_to_string(&mut data) {
      Ok(_) => {},
      Err(_) => panic!("Cannot read context file.")
    }

    // Parse Json form the data string.
    match Json::from_str(&mut data) {
        Ok(d) => d,
        Err(_) => panic!("Cannot decode json.")
    }
}


fn main() {

    let args: Vec<String> = env::args().collect();

    if !(args.len() > 2) {
        panic!("Usage: ./hb input_path.hbs context_path.json <output_path.html>");
    } 
    
    let input_path: String = args[1].clone();
    let context_path: String = args[2].clone();
    let output_path: String;

    if args.len() == 4 {
        output_path = args[3].clone();
    } else {
        // Replace input_path's extension with html and set to output_path.
        let mut split: Vec<&str> = input_path.split(".").collect();
        split.pop();
        split.push("html");
        output_path = split.join(".");
    }

    let handlebars = Handlebars::new();

    let json = get_json_or_panic(&context_path);
    let data = Context::wraps(&json);

    let mut source_template = match File::open(&input_path) {
        Ok(e) => e,
        Err(_) => panic!("No template file!"),
    };

    let mut output_file = match File::create(&output_path) {
        Ok(e) => e,
        Err(_) => panic!("Cannot create output template!"),
    };

    if let Ok(_) = handlebars.template_renderw2(&mut source_template, &data, &mut output_file) {
        println!("{} generated", output_path);
    } else {
        panic!("Failed to geneate output.html. Bad hbs syntax?");
    };
}
