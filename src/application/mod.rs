use crate::api::configuration::get_baize_configuration;

use anyhow::{Ok, Result};
use std::io::{stdin, stdout, Write};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Clone)]
pub struct BaizeApplication {
    change: bool,
    quit: bool,
    buffer: Vec<String>,
}

impl BaizeApplication {
    pub fn new() -> Self {
        Self {
            change: false,
            quit: false,
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
    fn get_buffer(&self) -> &Vec<String> {
        &self.buffer
    }
    fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = stdin().keys().next() {
                return key;
            }
        }
    }
    fn refresh_screen(&self) -> Result<()> {
        println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        stdout().flush()?;
        Ok(())
    }
    fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = self.read_key()?;
        match pressed_key {
            Key::Char('q') => self.quit = true,
            _ => (),
        }
        Ok(())
    }
    fn draw_rows(&self) {
        for _ in 0..24 {
            print!("~\r")
        }
    }
    fn feedback(&self, err: anyhow::Error) {
        self.refresh_screen().unwrap();
        panic!("{}", err);
    }
    pub async fn run(&mut self) -> Result<()> {
        let _baize_config = get_baize_configuration();
        let _stdout = stdout().into_raw_mode()?;
        self.get_buffer();

        /*tokio::spawn(async move {
            todo!();
            需要先读取配置文件，检查是否有插件缺少，如果有，则下载插件
            然后检查插件是否有更新，如果有，则更新插件
        });*/

        loop {
            if let Err(err) = self.refresh_screen() {
                self.feedback(err);
            }
            if self.quit {
                break;
            } else {
                self.draw_rows();
                print!("{}", termion::cursor::Goto(1, 1));
            }
            if let Err(err) = self.process_keypress() {
                self.feedback(err);
            }
        }

        Ok(())
    }
}
