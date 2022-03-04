pub mod api;
pub mod keyboard;
pub mod message;
pub mod screen;

const WELCOME_INFO: &str = "
Copyright (c) 2022 by Ruigang Zhang
This Source Code Form is subject to the terms of the MIT LICENSE. 
Baize is an experimental project for junior middle school students.
Because I'm a junior high school student, I can't fix and find bugs many times, so welcome to submit pull requests.
中考加油！
";

fn main() {
    //待screen模块完成后，采用screen模块渲染欢迎页面
    println!("{}",WELCOME_INFO);
}