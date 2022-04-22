use crate::api::configuration::get_baize_configuration;

use anyhow::Result;
use std::io::{self, stdin, stdout};
use termion::event::Key;

mod terminal;
use crate::application::terminal::Terminal;

 const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct BaizeApplication {
    terminal: Terminal,
    change: bool,
    quit: bool,
    buffer: Vec<String>,
}

impl BaizeApplication {
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new().expect("初始化失败"),
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
    fn refresh_screen(&self) -> Result<()> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        Terminal::flush()?;
        Ok(())
    }

    fn draw_rows(&self) {
        let _terminal = Terminal::new();
        let height = self.terminal.size().height;
        for row in 0..height {
            Terminal::clear_current_line();
            if row == height / 3 {
                let welcome_message = format!("         baize VERSION:{}", VERSION); 
                let width = std::cmp::min(self.terminal.size().width as usize, welcome_message.len());
                println!("{}\r", &welcome_message[..width])
            }else{
                println!("~\r");
            }
        }
    }
    fn process_keypress(&mut self) -> Result<()> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Char('q') => self.quit = true,
            _ => (),
        }
        Ok(())
    }
    fn feedback(&self, err: anyhow::Error) {
        self.refresh_screen().unwrap();
        panic!("{}", err);
        todo!("把panic替换为tracing");
    }
    pub async fn run(&mut self) -> Result<()> {
        let _baize_config = get_baize_configuration();
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
                Terminal::clear_screen();
                break;
            } else {
                self.draw_rows();
            }
            if let Err(err) = self.process_keypress() {
                self.feedback(err);
            }
            Terminal::cursor_show();
            Terminal::flush()?;
        }

        Ok(())
    }
}
