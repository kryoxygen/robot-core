use std::io;
use std::process::Stdio;

use nix::sys::signal::{self, Signal};
use nix::unistd::Pid;
use tokio::process::{Child, Command};

// 管理子进程的结构体（异步版）
pub struct SubProcessManager {
    child: Option<Child>,
    command: String,
    args: Vec<String>,
}

impl SubProcessManager {
    /// 创建新的子进程管理器
    pub fn new(command: &str, args: &[&str]) -> Self {
        SubProcessManager {
            child: None,
            command: command.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// 获取子进程的可变引用
    pub fn get_child(&mut self) -> Option<&mut Child> {
        self.child.as_mut()
    }

    /// 检查子进程是否在运行
    pub fn is_running(&mut self) -> bool {
        if self.child.is_none() {
            return false;
        }
        self.child.is_some() && self.child.as_mut().unwrap().try_wait().unwrap().is_none()
    }

    /// 同步启动子进程
    pub fn start(&mut self) -> io::Result<()> {
        if self.is_running() {
            println!("子进程已启动");
            return Ok(());
        }
        let child = Command::new(&self.command).args(&self.args).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
        self.child = Some(child);
        println!("子进程启动成功! PID: {}", self.child.as_ref().unwrap().id().unwrap());
        Ok(())
    }

    /// 同步关闭子进程
    pub fn stop(&mut self) -> io::Result<()> {
        if let Some(mut child) = self.child.take() {
            match signal::kill(Pid::from_raw(child.id().unwrap() as i32), Signal::SIGINT) {
                Ok(_) => println!("发送终止信号成功"),
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("发送终止信号失败: {}", e))),
            }
            // 循环等待进程真正退出
            drop(child.stdin.take());
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        println!("子进程退出，状态码: {}", status);
                        break;
                    }
                    Ok(None) => std::thread::sleep(std::time::Duration::from_millis(100)),
                    Err(e) => return Err(e),
                }
            }
        } else {
            println!("子进程未运行");
        }
        Ok(())
    }

    /// 异步关闭子进程
    pub async fn async_stop(&mut self) -> io::Result<()> {
        if let Some(mut child) = self.child.take() {
            match signal::kill(Pid::from_raw(child.id().unwrap() as i32), Signal::SIGINT) {
                Ok(_) => println!("发送终止信号成功"),
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("发送终止信号失败: {}", e))),
            }
            // 等待进程真正退出
            match child.wait().await {
                Ok(status) => println!("子进程退出，状态码: {}", status),
                Err(e) => return Err(e),
            }
        } else {
            println!("子进程未运行");
        }
        Ok(())
    }

    /// 同步重启子进程
    pub fn restart(&mut self) -> io::Result<()> {
        if !self.is_running() {
            println!("子进程未运行，尝试重新启动");
        } else {
            println!("子进程正在运行，尝试重启");
            self.stop()?;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.start()?;
        println!("子进程已重启");
        Ok(())
    }

    /// 异步重启子进程
    pub async fn async_restart(&mut self) -> io::Result<()> {
        if !self.is_running() {
            println!("子进程未运行，尝试重新启动");
        } else {
            println!("子进程正在运行，尝试重启");
            self.async_stop().await?;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        self.start()?;
        println!("子进程已重启");
        Ok(())
    }
}
