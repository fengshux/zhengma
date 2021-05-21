extern crate clap;
use std::fs;
use std::collections::HashMap;
use clap::{Arg, App, SubCommand};
mod data;
mod init;

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
        let dict =  data::get_full_code_map();
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
    
        let dict =  data::get_full_code_map();
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
            Some(code) => coded_content.push_str(&format!("({})",code.iter().max_by(|x, y| x.len() >= y.len()).unwrap())),
            None => (),
        };
        
    }
    return coded_content
}
