# 🐱 baize   
(🐱猫猫好可爱🐱)  
baize是一个操作习惯类似于NeoVim的简易编辑器。  
~这个项目挺好的就是有点难，不适合我这个大脑发育不完全，小脑完全不发育的人做~

# 快捷键
## NORMAL模式  
```
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
u :更新配置文件  
tt :目录树  
e :TagBar  
p i :安装配置文件里指定的插件  
p u :更新配置文件里指定的插件  
c i : 安装语言支持模块  
c u : 更新语言支持模块  
w :保存文件  
q :退出  
fa CR :搜索单词  
ff CR :搜索文件  
fh CR :搜索记录  
fc CR :修改颜色  
fb CR :标签  
```  
## INSERT模式
```  
jk :ESC  
Tab :下一个补全选项  
Shift Tab :上一个补全选项
```
## VISUAL模式
```
CRTL+C :复制  
CRTL+V :粘贴
```
# TODO LIST
- [ ] 渲染
    - [ ] 无插件渲染
    - [ ] 有插件渲染
    - [ ] 提示（代码，文档）小窗口
- [ ] 识别快捷键
    - [ ] 命令系统
    - [ ] 快捷键系统
- [ ] API
    - [ ] 代码配色接口
    - [ ] 主题接口
    - [ ] LSP接口
    - [ ] 命令接口
    - [ ] other
- [ ] 打包
    - [ ] Windows
    - [ ] Debian
