use std::fs;
use std::collections::HashMap;

fn main() {

    let contents = fs::read_to_string("./data/zhengma.dict.yaml")
        .expect("Something went wrong reading the file");

    let v: Vec<&str> = contents.split("...").collect();
    let zhengma_str = v[1];
    let mut zhengma_dict = HashMap::new();
    for (_, line) in zhengma_str.lines().enumerate() {
        let dict: Vec<&str> = line.split("\t").collect();
        if dict.len() > 1 && !zhengma_dict.contains_key(dict[0]){
            zhengma_dict.insert(dict[0].to_string(), dict[1].to_string());
        }
    }

    match fs::write("./data/zhengma.data", to_format_string(&zhengma_dict).as_bytes()) {
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
