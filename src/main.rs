extern crate clap;
use std::fs;
use std::collections::HashMap;
use clap::{Arg, App, SubCommand};

fn main() {
   let matches =  App::new("zhengma")
        .version("v1.0-beta")
        .arg(Arg::with_name("WORLD")
             .help("输入汉字，输出郑码")
             .index(1)
        )
        .subcommand(SubCommand::with_name("trans")
                    .about("translate world to zhengma code")
                    .version("1.0")
                    .author("Someone E. <someone_else@other.com>")
                    .arg(Arg::with_name("INPUT")
                         .help("Sets the input file to use")
                         .required(true)
                         .index(1))
                    .arg(Arg::with_name("OUTPUT")
                         .help("output file . default output to console")
                         .long("output")
                         .short("o")
                         .takes_value(true)
                    )                    
        )
        .get_matches();

    if let Some(world) = matches.value_of("WORLD") {
        let dict = load_data_to_map_full_code("/etc/zhengma/data/zhengma.data");
        match dict.get(&world.to_string()) {           
            Some(codes) =>{
                println!("{}:{}", world, codes.join(","))
            }
            None => {
                println!("{}", "没有匹配的郑码")
            }
        };
    }

    
    // let file = matches.value_of("INPUT").unwrap();
    if let Some(matches) = matches.subcommand_matches("trans") {
        let file = matches.value_of("INPUT").expect("expect input file");
        let contents = fs::read_to_string(file)
            .expect("Something went wrong reading the file");
    
        let dict = load_data_to_map("/etc/zhengma/data/zhengma.data");    
        let coded = to_code(dict, &contents);
        match matches.value_of("OUTPUT") {
            Some(path) => fs::write(path, coded.as_bytes()).expect("write file error"),
            None => println!("{}",coded),
        };               
    } 
}


fn to_code(dict: Box<HashMap<String,String>>, contents: &str) -> Box<String>{
    let mut coded_content = Box::new("".to_owned());
    for v in contents.chars() {
        coded_content.push(v);
        match dict.get(&v.to_string()) {
            Some(code) => coded_content.push_str(&format!("({})",code)),
            None => (),
        };
        
    }
    return coded_content
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


fn load_data_to_map_full_code(path: &str) -> Box<HashMap<String,Vec<String>>> {
    let mut dict: Box<HashMap<String, Vec<String>>> = Box::new(HashMap::new());
    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    for line in contents.lines() {
        let items: Vec<&str> = line.split(",").collect();

        // 一个字可能对应多郑码这里可以列出多个
        if dict.contains_key(items[1]) {
            let codes = dict.get_mut(items[1]).unwrap();
            codes.push(items[0].to_string());
        } else {
            let mut codes: Vec<String> = Vec::new();
            codes.push(items[0].to_string());
            dict.insert(items[1].to_string(), codes);
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
