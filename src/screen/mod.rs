use crate::message::*;
extern crate term;

//定义一个屏幕数据结构体
pub struct Screen{
    width: usize,
    height: usize,
    status: String,
    buffer: Box<Vec<String>>,
    stream: Box<dyn term::Terminal<Output = std::io::Stdout> + Send>,
    info: String,
}

pub struct Color{
    themes: bool,
    themes_name: String,
    themes_scheme: String,
}

pub struct Render{

}

impl Screen{
    pub fn new() -> Self{
        Screen{
            width: 0,
            height: 0,
            status: String::new(),
            buffer: Box::new(Vec::new()),
            info: String::new(),
            stream: Box::from(term::stdout().unwrap()),
        }
    }
    fn get_size(&mut self){ 
        if let Some((width, height)) = term_size::dimensions() {
            self.width = width;
            self.height = height;
        }
    }
    pub fn check_size_error(&mut self) -> Result<(),&str>{
        if let Some((width, height)) = term_size::dimensions() {
            if width != self.width || height != self.height {
                self.get_size();
                Ok(()) 
            }else{
                Err(ERROR_MSG_TERMINAL_IS_NOT_WORKING)
            }
        } else {
            Err(ERROR_MSG_TERMINAL_IS_NOT_WORKING)
        }
    }
    pub fn out_put_to_screen(&self){
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
    pub fn new_buffer(&mut self, member_number: String){
        self.buffer.push(member_number); 
    }
}

impl Color{
    pub fn new() -> Self{
        Color{
            themes: false,
            themes_name: String::from("none"),
            themes_scheme: String::from("none"),
        }
    }
    pub fn change_themes(&mut self, themes: bool){
        self.themes = themes;
    }
    pub fn change_themes_name(&mut self, themes_name: String){
        self.themes_name = themes_name;
    }
    pub fn change_themes_scheme(&mut self, themes_scheme: String){
        self.themes_scheme = themes_scheme;
    }
}
