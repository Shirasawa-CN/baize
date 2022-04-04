use baize::api::plug::upgrade_plug;
use baize::api::{plug::download_plug, read_baize_configuration};
use baize::keyboard::*;

use anyhow::{Ok, Result};
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

#[tokio::main]
async fn main() -> Result<()> {
    let _baize_config = read_baize_configuration();
    let _stdout = stdout().into_raw_mode()?;
    let mut status = Status::new();
    status.get_buffer();

    tokio::spawn(async move {
        download_plug().await;
        upgrade_plug().await;
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
