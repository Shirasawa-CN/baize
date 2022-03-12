pub mod api;
pub mod keyboard;
pub mod screen;
use crate::api::read_baize_configuration;

use anyhow::Result;

fn main() -> Result<()>{

    let _baize_config = read_baize_configuration()?;
    //采用screen模块渲染欢迎页面
    let mut screen_stream = screen::Screen::new();
    screen_stream.print_welcome_info();
    Ok(())
}
