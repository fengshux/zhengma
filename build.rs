use std::fs;
use std::process::Command;
use std::env;
use std::path::Path;


fn main() {
    
    println!("build on {}", env::var("PROFILE").unwrap());
    
    if env::var("PROFILE").unwrap() == "release" {
        let target_directory: String;
        match env::var("CARGO_HOME") {
            Ok(cargo_home) => {
                target_directory = format!("{}/data/zhengma/",cargo_home);
                let path = Path::new(&target_directory);
                if !path.exists() {
                    fs::create_dir_all(target_directory.clone()).unwrap();
                }                                             
            }
            Err(_) => {
                target_directory = format!("{}/.cargo/", env::var("HOME").unwrap());
            }
        }
        let target = Path::new(&target_directory).join("zhengma.hash");
        Command::new("cp").args(&["data/zhengma.hash", target.to_str().unwrap()])
            .status().unwrap();


        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("file.rs");
        
        fs::write(
            &dest_path,
            format!("fn get_hash_file_path() -> String {{
                    \"{}\".to_string()
            }}
           ", target.to_str().unwrap())
        ).unwrap();
        println!("cargo:rerun-if-changed=build.rs");
    }

}
