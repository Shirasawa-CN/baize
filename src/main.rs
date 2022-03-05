pub mod api;
pub mod keyboard;
pub mod message;
pub mod screen;



fn main() {
    //采用screen模块渲染欢迎页面
    let mut screen_stream = screen::Screen::new();
    screen_stream.print_welcome_info();
}