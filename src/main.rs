use serde_yaml::Value;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析为结构体
    // let config: Value = serde_yaml::from_str(&contents)?;
    // let config: Config  = serde_yaml::from_str(&contents)?;
    let config: Value = serde_yaml::from_reader(File::open("etc/test.yaml")?)?;
    println!("{}", config["server"]["host"].as_str().unwrap());

    Ok(())
}

