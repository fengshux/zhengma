
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    
    data_to_hash_file();
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


#[derive(Clone, Debug)]
struct Cell {
    key: String,
    codes: Vec<String>,
}

pub fn data_to_hash_file() {
    let dict = load_data_to_map_full_code("./data/zhengma.data");
    println!("{}", "load map success");
    let keys: Vec<&String> = dict.keys().collect();    
    let mut hash_array: Vec<Vec<Cell>> = vec![Vec::new(); 100000];
    

    for key in &keys {
        println!("{}", key);
        let chars = key.chars();
        println!("chars:{:?}", chars);
        let sum = key.chars().map(|c| {
            let mut b = [0; 8];
            c.encode_utf8(&mut b);
            println!("int:{}",u64::from_le_bytes(b));
            return u64::from_le_bytes(b)
        } ).sum::<u64>();
        let index = (sum % 100000) as usize;
        
        if let Some(arr) = hash_array.get_mut(index) {
            arr.push(Cell{key:key.to_string(), codes: dict.get(*key).unwrap().to_vec()})
        }        
    }

    let mut out_writer = BufWriter::new(Box::new(File::create("./data/zhengma.hash").unwrap()));
    for mut item in hash_array {
        println!("{:?}", item);
        item.sort_by_key(| cell | cell.key.chars().count() );
        if item.len() > 0 {
            let mut str_cells: Vec<String> = Vec::new();
            for cell in item {
                str_cells.push(format!("{}:{}", cell.key, cell.codes.join(",")));
            }            
            out_writer.write(str_cells.join(";").as_bytes()).unwrap();
        }
        out_writer.write("\n".as_bytes()).unwrap();
    }
    
    println!("write, success")
     
}
