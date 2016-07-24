extern crate handlebars;
extern crate rustc_serialize;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path};

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

fn write_to_file_or_panic(path: String, context: &Context, handlebars: &Handlebars) {
    let mut write = match File::create(&path) {
        Ok(e) => e,
        Err(_) => panic!("Cannot create output template!"),
    };
    
    match handlebars.renderw("template", &context, &mut write) {
        Ok(e) => e,
        Err(_) => panic!("Cannot write to output template!"),
    };
}

fn write_to_stdout_or_panic(context: &Context, handlebars: &Handlebars) {
    match handlebars.renderw("template", &context, &mut io::stdout()) {
        Ok(e) => e,
        Err(_) => panic!("Cannot write to standard output ¯\\_(ツ)_/¯"),
    };
}


fn main() {

    let args: Vec<String> = env::args().collect();

    if !(args.len() > 2) {
        panic!("Usage: ./hb input_path.hbs context_path.json <output_path.html>");
    } 

    let input_path_str = args[1].clone();
    let input_path = Path::new(&input_path_str);
    let context_path: String = args[2].clone();
    let mut output_path: String = String::new();

    if args.len() == 4 {
        output_path = args[3].clone();
    }

    let mut handlebars = Handlebars::new();

    let json = get_json_or_panic(&context_path);
    let data = Context::wraps(&json);

    match handlebars.register_template_file("template", input_path) {
        Ok(_) => {},
        Err(_) => panic!("Cannot register template, bad syntax?")
    }
    
    if output_path.len() > 0 {
        write_to_file_or_panic(output_path, &data, &handlebars);
    } else {
        write_to_stdout_or_panic(&data, &handlebars);
    }

}
