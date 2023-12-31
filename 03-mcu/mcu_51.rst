.. _022_mcu51_summary_index:

==============================================================
基于ubuntu 20.4开发51单片机
==============================================================

使用的软件
==============================================================

编译软件： sdcc/packihx
---------------------------------------------

ubuntu下安装： sudo apt-get install sdcc

烧录软件：stcflash
---------------------------------------------

ubuntu下安装： 从社区下载到本地，社区地址：https://github.com/laborer/stcflash

将stcflash.py文件解压到/usr/local/bin。 并去掉后缀名。

依赖： python3, python-pip3, python serial

安装python serial: https://www.geeksforgeeks.org/how-to-install-python-serial-package-on-linux/

如果执行: stcflash报错： usr/bin/env: ‘python’: No such file or directory

执行以下命令解决： sudo ln -s /usr/bin/python3 /usr/bin/python

串口调试工具： comtool
---------------------------------------------

安装： sudo pipe3 install comtool

运行： sudo comtool

开发代码，参照代码目录
==============================================================

编译烧录
---------------------------------------------

第一步，编译：

$sudo sdcc led_basic.c -o led_basic.ihx

第二步，转换成可烧录的hex文件：

$packihx led_basic.ihx > led_basic.hex

第三步，烧录文件到MCU：

$sudo stcflash led_basic.hex

此命令会自动寻找USB口发命令。如果有多个USB口，需要指定相应端口。

$sudo stcflash led_basic.hex --port /dev/ttyUSB0

串口调试
==============================================================
打开comtool工具，在SendReceive页操作：
在左侧connection中选择端口和波特率后点"open"按钮，连接串口。
在右侧下框中输入要发送的字符，点击右下的发送按钮发送。

附录
==============================================================
直流电机驱动： L298N
https://www.bilibili.com/video/BV1Xt411372X/?spm_id_from=333.337.search-card.all.click&vd_source=2d1a1996f721c2be8579fe3af09c7f00