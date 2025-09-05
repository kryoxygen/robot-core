mod hardware;

use crate::hardware::laser::LaserConfig;
use nix::sched::{sched_setaffinity, CpuSet};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml_str = std::fs::read_to_string("etc/config.yaml")?;
    let config: LaserConfig = serde_yaml::from_str(&yaml_str)?;

    // 打印解析结果
    // for (id, laser) in config.laser {
    //     println!("Laser ID: {}", id);
    //     println!("Type: {}", laser.type_name);
    //     println!("  Name: {}", laser.name);
    //     println!("  IP: {}", laser.laser_ip);
    //     println!("  Topic: {}", laser.topic);
    //     println!("  Pose: ({}, {}, {}, {}, {}, {})",
    //         laser.calib_param.px,
    //         laser.calib_param.py,
    //         laser.calib_param.pz,
    //         laser.calib_param.roll,
    //         laser.calib_param.pitch,
    //         laser.calib_param.yaw
    //     );
    // }

    Ok(())
}
