use crate::message::*;

//定义一个屏幕数据结构体
pub struct Screen{
    width: usize,
    height: usize,
    terimal: String,
    status: String,
    buffer: Vec<Vec<u128>>,
}

pub struct Color{
    themes: bool,
    themes_name: String,
}

impl Screen{
    
    fn get_size(&mut self) -> &str{
        if let Some((width, height)) = term_size::dimensions() {
            self.width = width;
            self.height = height;
            SUCCESS

        } else {
            ERROR_MSG_TERMINAL_IS_NOT_SUPPORTED
        }
    }
    fn check_size_error(&self) -> &str{
        if let Some((width, height)) = term_size::dimensions() {
            if width != self.width.try_into().unwrap() || height != self.height.try_into().unwrap() {
                ERROR_MSG_SIZE_IS_NOT_CORRECT
            }else{
                ERROR_MSG_TERMINAL_IS_NOT_SUPPORTED
            }
        } else {
            ERROR_MSG_TERMINAL_IS_NOT_SUPPORTED
        }
    }
}