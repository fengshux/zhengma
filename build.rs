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
        Command::new("cp").args(&["data/zhengma.hash", &format!("{}/zhengma.hash", target_directory)])
            .status().unwrap();
    }

}
