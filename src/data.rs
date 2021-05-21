use std::collections::HashMap;


#[macro_export]
macro_rules! box_hashmap_vec {
    ($( $key: expr => ($($x:expr),*) ),*) => {{
        let mut map = Box::new(::std::collections::HashMap::new());
        $( map.insert($key.to_string(), vec![$($x.to_string()),*]); )*
            map
    }}
}

pub fn get_full_code_map() -> Box<HashMap<String,Vec<String>>> {

    box_hashmap_vec!["A" => ("1","2"), "C" => ("4"), "G" => ("5","6")]
}
