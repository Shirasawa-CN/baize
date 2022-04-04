/*
NORMAL MODE

fs :保存会话
fl :加载会话
sv :左右切割屏幕
sp :上下切割屏幕
sh :切换至左边屏幕
sl :切换至右边屏幕
sj :切换至下面屏幕
sk :切换至上面屏幕
tn :新建TAB
tl :切换至下一个TAB
th :切换至上一个TAB
tb :切换至映射Buffer区，并可用字母快速切换Buffer
tc :关闭TAB
tt :目录树
e :TagBar
w :保存文件
q :退出
fa CR :搜索单词
ff CR :搜索文件
fh CR :搜索记录
fc CR :修改颜色
fb CR :标签

INSERT MODE
jk :ESC
Tab :下一个补全选项
Shift Tab :上一个补全选项
*/

pub fn ctrl_byte(key: char) -> u8 {
    let byte = key as u8;
    byte & 0x11
}
pub fn shift_byte(key: char) -> u8 {
    let byte = key as u8;
    byte & 0x10
}
pub fn alt_byte(key: char) -> u8 {
    let byte = key as u8;
    byte & 0x12
}
pub fn tab_byte(key: char) -> u8 {
    let byte = key as u8;
    byte & 0x09
}
pub fn to_u8(key: char) -> u8 {
    key as u8
}
