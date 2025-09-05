use std::collections::HashMap;

use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, BufReader};

use super::process::SubProcessManager;

#[derive(Debug, Deserialize)]
pub struct LaserConfig {
    laser: HashMap<String, Laser>
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Laser {
    // 基本信息
    name: String,
    name_compatible: String,
    #[serde(rename = "type")]
    type_name: String,
    position: String,
    purpose: String,
    features: String,
    platform: i32,

    // 坐标系
    base_frame_id: String,
    frame_id: String,

    // 网络连接
    connect_type: i32,
    laser_ip: String,
    laser_port: u16,
    local_addr: String,
    local_device: String,

    // 扫描参数
    angle_min: f64,
    angle_max: f64,
    angle_increment: f64,
    angle_resolution: f64,
    scan_frequency: i32,
    intensity: bool,
    inverted: bool,

    // 有效测距
    min_range: f64,
    max_range: f64,

    // 位姿标定
    calib_param: CalibParam,

    // ROS 话题
    topic: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct CalibParam {
    px: f64,
    py: f64,
    pz: f64,
    roll: f64,
    pitch: f64,
    yaw: f64,
}

pub async fn load_laser(config:LaserConfig) -> HashMap<String, SubProcessManager> {
    let mut spm_vec = HashMap::new();
    for (id, info) in config.laser {
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
