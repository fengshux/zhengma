use std::fs;
use std::collections::HashMap;

// init data from ./data/zhengma.dict.yaml into ./data/zhengma.data
pub fn init_data() {

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
pub fn to_format_string(dict: &HashMap<String,String>) ->  Box<String> {
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


// 把data里的数据装载到map里
pub fn load_data_to_map_full_code(path: &str) -> Box<HashMap<String,Vec<String>>> {
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



pub fn data_map_generator() {
    let dict = load_data_to_map_full_code("./data/zhengma.data");
    let mut zhengma_content: String = "".to_owned();
    for (key, val) in dict.iter() {
                       
        zhengma_content.push_str(&format!("\"{}\" => (\"{}\"),\n", key, val.join("\",\"")));
    }
    match fs::write("./data/zhengma.map", zhengma_content.as_bytes()) {
        Ok(_) => println!("write, success"),
        Err(e) => println!("write to file error,{}", e),
    }
}
