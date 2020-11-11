extern crate clap;
use std::fs;
use std::collections::HashMap;
use clap::App;

fn main() {
    App::new("zhengma").version("v1.0-beta").get_matches();
}

fn init_data() {

    let contents = fs::read_to_string("./data/zhengma.dict.yaml")
        .expect("Something went wrong reading the file");

    let v: Vec<&str> = contents.split("...").collect();
    let zhengma_str = v[1];
    let mut zhengma_content: String = "".to_owned();
    
    for line in zhengma_str.lines() {
        let dict: Vec<&str> = line.split("\t").collect();
        if dict.len() > 1 {
            let mut newline: String = "".to_owned();
            newline.push_str(dict[0]);
            newline.push(',');
            newline.push_str(dict[1]);
            newline.push(',');
            newline.push_str(dict[2]);
            newline.push('\n');
            zhengma_content.push_str(&newline);
        }
    }

    match fs::write("./data/zhengma.data", zhengma_content.as_bytes()) {
        Ok(()) => println!("write, success"),
        Err(e) => println!("write to file error,{}", e),
    }
}

fn to_format_string(dict: &HashMap<String,String>) ->  Box<String> {
    let mut contents: String = "".to_owned();
    for (key, val) in dict.iter() {
        let mut line: String =  key.clone().to_owned();
        line.push(',');
        line.push_str(val);
        line.push('\n');
        contents.push_str(&line);
        
    }
    return Box::new(contents)
}
