use crate::message::*;
extern crate term;

//定义一个屏幕数据结构体
pub struct Screen{
    width: usize,
    height: usize,
    status: String,
    buffer: Vec<Vec<u128>>,
    stream: Box<dyn term::Terminal<Output = std::io::Stdout> + Send>,
    info: String,
}

pub struct Color{
    themes: bool,
    themes_name: String,
}

impl Screen{
    pub fn new() -> Screen{
        Screen{
            width: 0,
            height: 0,
            status: String::new(),
            buffer: Vec::new(),
            info: String::new(),
            stream: Box::from(term::stdout().unwrap()),
        }
    }
    fn get_size(&mut self) -> &str{
        if let Some((width, height)) = term_size::dimensions() {
            self.width = width;
            self.height = height;
            SUCCESS

        } else {
            ERROR_MSG_TERMINAL_IS_NOT_WORKING
        }
    }
    fn check_size_error(&self) -> &str{
        if let Some((width, height)) = term_size::dimensions() {
            if width != self.width.try_into().unwrap() || height != self.height.try_into().unwrap() {
                ERROR_MSG_SIZE_IS_NOT_CORRECT
            }else{
                ERROR_MSG_TERMINAL_IS_NOT_WORKING
            }
        } else {
            ERROR_MSG_TERMINAL_IS_NOT_WORKING
        }
    }
    fn out_put_to_screen(&self){
        if self.status == "stop" {
            loop{
                if self.status == "start" {
                    break;
                }
            }
        }
    }
    pub fn print_welcome_info(&mut self){
        self.stream.fg(term::color::WHITE).unwrap();
        writeln!(self.stream, "{}", WELCOME_INFO).unwrap();
    }
    pub fn flush_screen(&mut self){
        self.stream.fg(term::color::WHITE).unwrap();
        writeln!(self.stream, "{}", self.info).unwrap();
        self.stream.flush().unwrap();
    }
}