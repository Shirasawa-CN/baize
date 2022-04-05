use crate::api::plug::upgrade_plug;
use crate::api::{configuration::get_baize_configuration, plug::download_plug};
use crate::keyboard::*;

use anyhow::Result;
use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

struct Status {
    change: bool,
    buffer: Vec<String>,
}
impl Status {
    fn new() -> Status {
        Status {
            change: false,
            buffer: Vec::new(),
        }
    }
    fn change(&mut self) {
        self.change = true;
    }
    fn reset(&mut self) {
        self.change = false;
        self.buffer.clear();
    }
    fn save(&mut self) -> Result<()> {
        self.change = false;
        todo!("保存文件的功能");
    }
    pub fn get_buffer(&self) -> &Vec<String> {
        &self.buffer
    }
}

pub struct BaizeApplication;

impl BaizeApplication {
    pub async fn run() -> Result<()> {
        let _baize_config = get_baize_configuration();
        let _stdout = stdout().into_raw_mode()?;
        let mut status = Status::new();
        status.get_buffer();

        tokio::spawn(async move {
            todo!();
            /* TODO
            需要先读取配置文件，检查是否有插件缺少，如果有，则下载插件
            然后检查插件是否有更新，如果有，则更新插件
            */
        });

        for b in io::stdin().bytes() {
            let b = b?;
            let _c = b as char;

            if b == to_u8('q') {
                if status.change == false {
                    break;
                } else if status.change == true {
                    status.save()?;
                    break;
                }
            }
        }
        Ok(())
    }
}
