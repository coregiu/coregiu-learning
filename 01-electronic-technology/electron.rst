.. _electron_summary_index:

============
1 电子元器件
============
这是我见过讲的比较透彻的一个视频了：

[三极管是如何实现放大的（上）赵全老师纯干货讲解！](https://m.toutiao.com/is/i8FKY8Y3/)

[三极管是如何实现放大的（下）赵全老师纯干货讲解！](https://m.toutiao.com/is/i8FKrSBh/)

PN结及三极管放大电路是人类史上的一个丰碑，他让人类实现了信息化、数字化。

==========
2 放大电路
==========
[三极管的工作原理-赵全（上）2021版](https://m.toutiao.com/is/i8FK286g/)

[三极管的工作原理-赵全（下）2021版](https://m.toutiao.com/is/i8FwoM9b/)

经典放大电路：

.. image:: images/ttl_amplifier.jpg

放大电路要工作，必须满足三极管0.6-0.7V工作区间要求。低于0.6V，三极管不导通，叫截止。
大于0.7V叫饱和，再高了也不再放大了。
因此实际的放大电路是在0.6-0.7V之间上下信号放大。
那么该电路首先是通过一个偏置电路将Q的基极电压置到0.65V，那么X1信号来的时候（在正负0.5V区间），
可以通过Q的放大作用将信号放大到X2输出端。其中C1电容起到防止静态直流工作电压通过X1短路的作用。
该电路中，关键术语解释:

* 1 偏置电路，提供整体放大电路的静态工作电压的电路。核心关键是X3电源，R1,R2,R3，R4电阻以及Q三极管。
* 2 偏置电阻，R1,R2,RP分压，让Q的基极获得0.65V的静态工作电压，管这几个电阻叫偏置电阻。
* 3 输出电阻，R3电阻固定，那么在Q放大不同信号即不同电流时，Ic电流随之变化，那么R3两端电压也随之变化。
  进而Q的集电极电位也随之动态变化。因此管R3叫输出电阻。

有了放大电路，配合上光敏、压敏、红外、温度、湿度等等传感器，我们实现了自动化。

================
3 集成运算放大器
================
放大电路中的零点飘移问题（晶体管导电性因温度变化而变化），另外因为像炉温变化等场景，不宜电容耦合放大电路，需要直接耦合。

为解决零点飘移问题，有人发明了差分放大电路，即两个完全对称的放大电路，这样就在飘移大家一起飘移，然后相互一减，就抵消了突变情况，让电路保持稳定。

视频教程: https://m.toutiao.com/is/i8a239Tb

直接的差分放大电路：

.. image:: images/R-C.png

这里的差分电路必须双端输入，双端输出。

在上面差分电路接地端加上一个恒流源，可以做到单输入单输出。

.. image:: images/R-C-2.png

详细计算推导过程见视频。

集成运算放大器内部实现如下：

.. image:: images/integrated-amplifier.png


==================
4 逻辑数学与门电路
==================
数字电路本质上是高电平为1，低电平为0的逻辑运算。

【 1. 基本逻辑运算 】

① 与
Y=A · B =AB, 与运算：有0出0，全1出1.

② 或
Y=A+B , 或运算：有1出1，全0出0.

③ 非
Y=A‘

.. image:: images/logic_compute.png

【 2.复合逻辑运算】
① 与非
Y=(A·B)’, 有0出1，全1出0

② 或非
Y=(A+B)‘， 有1出0，全0出1.

③ 与或非
Y=(A·B+C·D)‘

④ 异或
Y=A⊕B= A · B’ + A’ · B， 不同出1，相同出0

⑤ 同或
Y=A⊙B= A · B +A’ · B’， 相同出1，不同出0

PS：异或逻辑与同或逻辑互为反函数、对偶函数。

.. image:: images/logic_compute_4.png

.. image:: images/logic_compute_2.png

.. image:: images/logic_compute_3.png

======================
5 数制与运算
======================

十进制转二进制
------------------------

整数部分
^^^^^^^^^^^^^^^^^^^^^^^^

将要转换的十进制整数除以2，取余数；再用商除以2，直到商等于0为止，将每次得到的余数按倒序的方法排列起来即为结果。
例如：125=1111101B

.. image:: images/o-b.png

小数部分
^^^^^^^^^^^^^^^^^^^^^^^^
原理：十进制小数转换成二进制小数采用 “乘2取整，顺序输出” 法。

例题： 0.68D = ______ B（精确到小数点后5位）

如下所示，0.68乘以2，取整，然后再将小数乘以2，取整，直到达到题目要求精度。得到结果：0.10101B.

例如：十进制小数0.68转换为二进制数

具体步骤：

0.68* 2=1.36 -->1

0.36* 2=0.72 -->0

0.72* 2=1.44 -->1

0.44* 2=0.88–>0

0.88* 2=1.76 -->1

已经达到了题目要求的精度，最后将取出的整数部分顺序输出即可
则为：0.68D–>0.10101B

二进制转十进制
------------------------

整数部分
^^^^^^^^^^^^^^^^^^^^^^^^

计算机中的数是用二进制数表示的，它的特点是逢二进一，因此在二进制中只有0和1两个数字符号。
特点:

（1）基数为2，数值部分用0和1表示

（2）逢二进一

（3）后缀用B或2表示，例如：（1010）2、（1010）B

（4）位权值为2^k （k为数位）

例如：

.. image:: images/b-o.png

小数部分
^^^^^^^^^^^^^^^^^^^^^^^^

（1）原理：整数部分按上述进行操作即可，小数部分从小数点后一位指数为-1开始算起，以后依次为-2、-3……

（2）具体运用以及步骤举例说明：

.. image:: images/b-o-f.webp

二进制运算
------------------------
加法
^^^^^^^^^^^^
同十进制类似，只是缝2进1，如：

1011 + 1001 = 10100

减法
^^^^^^^^^^^^
通过加法实现减法，只是加的是被减数的补码。

补码： 原数的取反 + 1

例如： 0101 - 0011 = 0010， 利用加法实现为 0101 + 1101 = 0010

1010(10) - 1111(15) = 01010 + 10001 = 11011， 11011除符号位外取反+1得 10101 为 -5

乘法
^^^^^^^^^^^^
无符号数二进制乘法运算法则：按位相乘，再按位进行二进制加法

.. image:: images/multiple.webp

除法
^^^^^^^^^^^^
1） 每一步做减法

.. image:: images/sub_1.png

2） 每一步做异或

.. image:: images/sub_2.png

======================
6 触发器及时序逻辑电路
======================
RS触发器
---------------
有与非门和或非门RS触发器两种，分别如下图：

.. image:: images/r-s.jpg

或非门RS触发器状态表达： Q(n+1) = S + (!R)Q(n)
简单记是与非门0控制，不能全0；或非门1控制，不能全1.

时钟脉冲RS触发器
--------------------------------
触发器+同步信号控制：让触发器按某个信号节奏转换状态，
带接在在系统时钟信号上的同步控制信号CP(clk)的触发器

.. image:: images/c-r-s.webp

CLK=1，两与非门只受SR输入控制

SR的输入可直接传递到基本RS触发器的输入端并寄存

同步RS触发器在CLK=1期间输出取决于输入S和R，输入变化→输出变化(见上方激励表)

CLK=0，所有输入被封锁，基本RS触发器的SR=00，触发器输出不变

.. image:: images/c-r-s-r.png

D触发器
-------------------

D触发器是一种最简单的触发器，在触发边沿到来时，将输入端的值存入其中，并且这个值与当前存储的值无关。在两个有效的脉冲边沿之间，D的跳转不会影响触发器存储的值，
但是在脉冲边沿到来之前，输入端D必须有足够的建立时间，保证信号稳定。D触发器的逻辑符号如下图所示。

.. image:: images/d-logo.png

.. image:: images/d.png

D触发器工作脉冲如下图：

.. image:: images/d-r.jpeg

寄存器
-------------

寄存器是CPU内部用来存放数据的一些小型存储区域，用来暂时存放参与运算的数据和运算结果。其实寄存器就是一种常用的时序逻辑电路，但这种时序逻辑电路只包含存储电路。寄存器的存储电路是由锁存器或触发器构成的，因为一个锁存器或触发器能存储1位二进制数，所以由N个锁存器或触发器可以构成N位寄存器。寄存器是中央处理器内的组成部分。
寄存器是有限存储容量的高速存储部件，它们可用来暂存指令、数据和位址。

.. image:: images/register.png

