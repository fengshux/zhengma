extern crate clap;
use std::fs;
use std::collections::HashMap;
use clap::{Arg, App, SubCommand};

fn main() {
   let matches =  App::new("zhengma")
        .version("v1.0-beta")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    println!("Using input file: {}", file);
    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");
    
    let dict = load_data_to_map("./data/zhengma.data");
    
    for v in contents.chars() {
        print!("{}",v);
        match dict.get(&v.to_string()) {
            Some(code) => print!("({})",code),
            None => (),
        };
   
    }    
}

fn load_data_to_map(path: &str) -> Box<HashMap<String,String>> {
    let mut dict: Box<HashMap<String, String>> = Box::new(HashMap::new());
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    for line in contents.lines() {
        let items: Vec<&str> = line.split(",").collect();
        // one world may has more than one code.
        // dict里保留最简单的code. 不存在则认为复杂code
        let is_simple_code: bool = match dict.get(items[1]) {
            Some(code) => code.len() < items[0].len(),
            None => false,
        };
        
        if  !is_simple_code {
            dict.insert(items[1].to_string(),items[0].to_string());
        }
    }
    return dict
}

// init data from ./data/zhengma.dict.yaml into ./data/zhengma.data
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
        Ok(_) => println!("write, success"),
        Err(e) => println!("write to file error,{}", e),
    }
}

// format a hashmap to string in key,vale each line
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
