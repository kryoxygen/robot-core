use std::collections::HashMap;

use jzlog::info;
use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, BufReader};

use super::process::SubProcessManager;
#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    camera: HashMap<String, Camera>,
}
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Camera {
    // 基本信息
    name: String,
    #[serde(rename = "type")]
    type_name: String,
    frame_id: String,
}

pub async fn load_camera(config: CameraConfig) -> HashMap<String, SubProcessManager> {
    let mut spm_vec = HashMap::new();
    for (id, info) in config.camera {
        let file_path = format!("/opt/jz/robot-core/etc/launch/{}.launch.py", info.type_name);
        let spm = SubProcessManager::new(
            "stdbuf",
            &[
                "-oL",
                "ros2",
                "launch",
                file_path.as_str(),
                format!("camera_name:={}", info.name).as_str(),
                format!("cloud_frame_id:={}", info.frame_id).as_str()
            ],
        );
        spm_vec.insert(id, spm);
    }
    return spm_vec;
}

pub async fn start_camera(node_name: String, spm: &mut SubProcessManager) -> Result<(), std::io::Error> {
    spm.start()?;
    let node_name_clone = node_name.clone();
    let child_stdout = spm.get_child().unwrap().stdout.take().expect(format!("Failed to get child stdout,launch_file:").as_str());
    tokio::spawn(async move {
        let mut stdout_reader = BufReader::new(child_stdout).lines();
        while let Some(line) = stdout_reader.next_line().await.unwrap() {
            info!("[{}] {}", node_name_clone, line);
        }
    });
    Ok(())
}
