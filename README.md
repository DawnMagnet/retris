# retris(tetris made by rust)

## 介绍

一个使用 rust 语言编写的俄罗斯方块

[![sRuGjA.gif](https://s3.ax1x.com/2021/01/20/sRuGjA.gif)

## 安装说明

如果你只是想玩一玩这个小游戏,那么可以在 Release Page 直接下载  
如果你是开发者，欢迎 Star+Fork，也欢迎联系我

### 源码编译指南

请确保你的系统安装上有`cargo`与`git`命令行工具

```bash
git clone https://gitee.com/dawn_magnet/retris.git
cd retris
cargo run --release
```

本应用正在开发中，如有问题请联系本人
<axccjqh@qq.com>

## 版本日志

### v0.2.0 2022 年 2 月 28 日

#### 重大版本更新发布

1. 消除所有unsafe code，现在的俄罗斯方块更加稳定啦！
2. 修复了许多极端条件下的bug
3. 升级rust版本至2021
4. 优化了多线程代码逻辑

### v0.1.0 2021 年 1 月 19 日

1. 使用了颜色系统，现在的俄罗斯方块更加现代化啦！  
2. 通过 crosstermux 终端 Cursor 控制系统实现了 raw_mode 下的读取及页面绘制工作， 现在本软件可以工作在 Windows10/8.1 及所有符合 Unix/Ansi 标准的终端下面啦！  
3. 修复了在 Unix 终端下界面显示混乱的问题  
4. 修复了在疯狂按住空格造成的线程泄露问题

### v0.0.3 2021 年 1 月 19 日

1. 使用了备用屏幕模式，即在程序退出后不会影响本 terminal 的状态  
2. 优化了光标隐藏的逻辑

### v0.0.2 2021 年 1 月 18 日

1. 加入了分数统计系统  
2. 加入了状态显示模块

### v0.0.1 2021 年 1 月 18 日

1. 整体功能的设计完成  
2. 终端输出的稳定化及多线程处理键盘输入与定时下落之间的逻辑问题  
3. 三种模式：Playing/Pausing/Stopped，通过键盘切换  
4. 界面的优化  
5. bug 修复
 