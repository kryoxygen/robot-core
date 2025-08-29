use serde::Deserialize;
use serde_yaml::Value;

use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取文件
    let mut file = File::open("test.yaml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 解析为结构体
    // let config: Value = serde_yaml::from_str(&contents)?;
    // let config: Config  = serde_yaml::from_str(&contents)?;
let config: Value = serde_yaml::from_reader(File::open("test.yaml")?)?;
    println!("{}", config["server"]["host"].as_str().unwrap());

    Ok(())
}

