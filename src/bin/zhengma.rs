extern crate clap;
use std::str::Lines;
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
        match search_code("./data/zhengma.hash", world) {           
            Some(codes) =>{
                println!("{}:{}", world, codes)
            }
            None => {
                println!("{}", "没有匹配的郑码")
            }
        };
    }

    
    // let file = matches.value_of("INPUT").unwrap();
    if let Some(matches) = matches.subcommand_matches("trans") {
        // let file = matches.value_of("INPUT").expect("expect input file");
        // let contents = fs::read_to_string(file)
        //     .expect("Something went wrong reading the file");
    
        // let dict =  get_full_code_map();
        // let coded = to_code(dict, &contents);
        // match matches.value_of("OUTPUT") {
        //     Some(path) => fs::write(path, coded.as_bytes()).expect("write file error"),
        //     None => println!("{}",coded),
        // };
        println!("正在维护");
    } 
}

fn count_hash_index(key: &str) -> usize{
    let sum = key.chars().map(|c| {
        let mut b = [0; 8];
        c.encode_utf8(&mut b);
        return u64::from_le_bytes(b)
    } ).sum::<u64>();
    return (sum % 100000) as usize;
}


// 临时方法为了让程序运行
fn search_code(path: &str, key: &str) -> Option<String> {

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    
    let index = count_hash_index(key);

    match contents.lines().nth(index) {
        Some(line) => {
            if line == "" {
                return None;
            }            
            let cells: Vec<&str> = line.split(";").collect();
            for cell in cells {
                let code: Vec<&str> = cell.split(":").collect();
                if code[0] == key {
                    return Some(code[1].to_string());
                }
            }
            return None;
        }
        None => {
            return None;
        }
    }
}

// fn to_code(dict: Box<HashMap<String,Vec<String>>>, contents: &str) -> Box<String>{
//     let mut coded_content = Box::new("".to_owned());
//     for v in contents.chars() {
//         coded_content.push(v);
//         match dict.get(&v.to_string()) {
//             Some(code) => coded_content.push_str(&format!("({})",code.iter().max_by_key(|x| x.len()).unwrap())),
//             None => (),
//         };
        
//     }
//     return coded_content
// }
