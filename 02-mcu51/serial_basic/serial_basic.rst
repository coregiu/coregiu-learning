                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module serial_basic
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _ser
                                     12 	.globl _main
                                     13 	.globl _change_led
                                     14 	.globl _CY
                                     15 	.globl _AC
                                     16 	.globl _F0
                                     17 	.globl _RS1
                                     18 	.globl _RS0
                                     19 	.globl _OV
                                     20 	.globl _F1
                                     21 	.globl _P
                                     22 	.globl _PS
                                     23 	.globl _PT1
                                     24 	.globl _PX1
                                     25 	.globl _PT0
                                     26 	.globl _PX0
                                     27 	.globl _RD
                                     28 	.globl _WR
                                     29 	.globl _T1
                                     30 	.globl _T0
                                     31 	.globl _INT1
                                     32 	.globl _INT0
                                     33 	.globl _TXD
                                     34 	.globl _RXD
                                     35 	.globl _P3_7
                                     36 	.globl _P3_6
                                     37 	.globl _P3_5
                                     38 	.globl _P3_4
                                     39 	.globl _P3_3
                                     40 	.globl _P3_2
                                     41 	.globl _P3_1
                                     42 	.globl _P3_0
                                     43 	.globl _EA
                                     44 	.globl _ES
                                     45 	.globl _ET1
                                     46 	.globl _EX1
                                     47 	.globl _ET0
                                     48 	.globl _EX0
                                     49 	.globl _P2_7
                                     50 	.globl _P2_6
                                     51 	.globl _P2_5
                                     52 	.globl _P2_4
                                     53 	.globl _P2_3
                                     54 	.globl _P2_2
                                     55 	.globl _P2_1
                                     56 	.globl _P2_0
                                     57 	.globl _SM0
                                     58 	.globl _SM1
                                     59 	.globl _SM2
                                     60 	.globl _REN
                                     61 	.globl _TB8
                                     62 	.globl _RB8
                                     63 	.globl _TI
                                     64 	.globl _RI
                                     65 	.globl _P1_7
                                     66 	.globl _P1_6
                                     67 	.globl _P1_5
                                     68 	.globl _P1_4
                                     69 	.globl _P1_3
                                     70 	.globl _P1_2
                                     71 	.globl _P1_1
                                     72 	.globl _P1_0
                                     73 	.globl _TF1
                                     74 	.globl _TR1
                                     75 	.globl _TF0
                                     76 	.globl _TR0
                                     77 	.globl _IE1
                                     78 	.globl _IT1
                                     79 	.globl _IE0
                                     80 	.globl _IT0
                                     81 	.globl _P0_7
                                     82 	.globl _P0_6
                                     83 	.globl _P0_5
                                     84 	.globl _P0_4
                                     85 	.globl _P0_3
                                     86 	.globl _P0_2
                                     87 	.globl _P0_1
                                     88 	.globl _P0_0
                                     89 	.globl _B
                                     90 	.globl _ACC
                                     91 	.globl _PSW
                                     92 	.globl _IP
                                     93 	.globl _P3
                                     94 	.globl _IE
                                     95 	.globl _P2
                                     96 	.globl _SBUF
                                     97 	.globl _SCON
                                     98 	.globl _P1
                                     99 	.globl _TH1
                                    100 	.globl _TH0
                                    101 	.globl _TL1
                                    102 	.globl _TL0
                                    103 	.globl _TMOD
                                    104 	.globl _TCON
                                    105 	.globl _PCON
                                    106 	.globl _DPH
                                    107 	.globl _DPL
                                    108 	.globl _SP
                                    109 	.globl _P0
                                    110 	.globl _flag
                                    111 	.globl _led_times
                                    112 	.globl _a
                                    113 	.globl _num
                                    114 ;--------------------------------------------------------
                                    115 ; special function registers
                                    116 ;--------------------------------------------------------
                                    117 	.area RSEG    (ABS,DATA)
      000000                        118 	.org 0x0000
                           000080   119 _P0	=	0x0080
                           000081   120 _SP	=	0x0081
                           000082   121 _DPL	=	0x0082
                           000083   122 _DPH	=	0x0083
                           000087   123 _PCON	=	0x0087
                           000088   124 _TCON	=	0x0088
                           000089   125 _TMOD	=	0x0089
                           00008A   126 _TL0	=	0x008a
                           00008B   127 _TL1	=	0x008b
                           00008C   128 _TH0	=	0x008c
                           00008D   129 _TH1	=	0x008d
                           000090   130 _P1	=	0x0090
                           000098   131 _SCON	=	0x0098
                           000099   132 _SBUF	=	0x0099
                           0000A0   133 _P2	=	0x00a0
                           0000A8   134 _IE	=	0x00a8
                           0000B0   135 _P3	=	0x00b0
                           0000B8   136 _IP	=	0x00b8
                           0000D0   137 _PSW	=	0x00d0
                           0000E0   138 _ACC	=	0x00e0
                           0000F0   139 _B	=	0x00f0
                                    140 ;--------------------------------------------------------
                                    141 ; special function bits
                                    142 ;--------------------------------------------------------
                                    143 	.area RSEG    (ABS,DATA)
      000000                        144 	.org 0x0000
                           000080   145 _P0_0	=	0x0080
                           000081   146 _P0_1	=	0x0081
                           000082   147 _P0_2	=	0x0082
                           000083   148 _P0_3	=	0x0083
                           000084   149 _P0_4	=	0x0084
                           000085   150 _P0_5	=	0x0085
                           000086   151 _P0_6	=	0x0086
                           000087   152 _P0_7	=	0x0087
                           000088   153 _IT0	=	0x0088
                           000089   154 _IE0	=	0x0089
                           00008A   155 _IT1	=	0x008a
                           00008B   156 _IE1	=	0x008b
                           00008C   157 _TR0	=	0x008c
                           00008D   158 _TF0	=	0x008d
                           00008E   159 _TR1	=	0x008e
                           00008F   160 _TF1	=	0x008f
                           000090   161 _P1_0	=	0x0090
                           000091   162 _P1_1	=	0x0091
                           000092   163 _P1_2	=	0x0092
                           000093   164 _P1_3	=	0x0093
                           000094   165 _P1_4	=	0x0094
                           000095   166 _P1_5	=	0x0095
                           000096   167 _P1_6	=	0x0096
                           000097   168 _P1_7	=	0x0097
                           000098   169 _RI	=	0x0098
                           000099   170 _TI	=	0x0099
                           00009A   171 _RB8	=	0x009a
                           00009B   172 _TB8	=	0x009b
                           00009C   173 _REN	=	0x009c
                           00009D   174 _SM2	=	0x009d
                           00009E   175 _SM1	=	0x009e
                           00009F   176 _SM0	=	0x009f
                           0000A0   177 _P2_0	=	0x00a0
                           0000A1   178 _P2_1	=	0x00a1
                           0000A2   179 _P2_2	=	0x00a2
                           0000A3   180 _P2_3	=	0x00a3
                           0000A4   181 _P2_4	=	0x00a4
                           0000A5   182 _P2_5	=	0x00a5
                           0000A6   183 _P2_6	=	0x00a6
                           0000A7   184 _P2_7	=	0x00a7
                           0000A8   185 _EX0	=	0x00a8
                           0000A9   186 _ET0	=	0x00a9
                           0000AA   187 _EX1	=	0x00aa
                           0000AB   188 _ET1	=	0x00ab
                           0000AC   189 _ES	=	0x00ac
                           0000AF   190 _EA	=	0x00af
                           0000B0   191 _P3_0	=	0x00b0
                           0000B1   192 _P3_1	=	0x00b1
                           0000B2   193 _P3_2	=	0x00b2
                           0000B3   194 _P3_3	=	0x00b3
                           0000B4   195 _P3_4	=	0x00b4
                           0000B5   196 _P3_5	=	0x00b5
                           0000B6   197 _P3_6	=	0x00b6
                           0000B7   198 _P3_7	=	0x00b7
                           0000B0   199 _RXD	=	0x00b0
                           0000B1   200 _TXD	=	0x00b1
                           0000B2   201 _INT0	=	0x00b2
                           0000B3   202 _INT1	=	0x00b3
                           0000B4   203 _T0	=	0x00b4
                           0000B5   204 _T1	=	0x00b5
                           0000B6   205 _WR	=	0x00b6
                           0000B7   206 _RD	=	0x00b7
                           0000B8   207 _PX0	=	0x00b8
                           0000B9   208 _PT0	=	0x00b9
                           0000BA   209 _PX1	=	0x00ba
                           0000BB   210 _PT1	=	0x00bb
                           0000BC   211 _PS	=	0x00bc
                           0000D0   212 _P	=	0x00d0
                           0000D1   213 _F1	=	0x00d1
                           0000D2   214 _OV	=	0x00d2
                           0000D3   215 _RS0	=	0x00d3
                           0000D4   216 _RS1	=	0x00d4
                           0000D5   217 _F0	=	0x00d5
                           0000D6   218 _AC	=	0x00d6
                           0000D7   219 _CY	=	0x00d7
                                    220 ;--------------------------------------------------------
                                    221 ; overlayable register banks
                                    222 ;--------------------------------------------------------
                                    223 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        224 	.ds 8
                                    225 ;--------------------------------------------------------
                                    226 ; internal ram data
                                    227 ;--------------------------------------------------------
                                    228 	.area DSEG    (DATA)
      000008                        229 _num::
      000008                        230 	.ds 1
      000009                        231 _a::
      000009                        232 	.ds 1
      00000A                        233 _led_times::
      00000A                        234 	.ds 2
      00000C                        235 _flag::
      00000C                        236 	.ds 2
                                    237 ;--------------------------------------------------------
                                    238 ; overlayable items in internal ram 
                                    239 ;--------------------------------------------------------
                                    240 ;--------------------------------------------------------
                                    241 ; Stack segment in internal ram 
                                    242 ;--------------------------------------------------------
                                    243 	.area	SSEG
      00000E                        244 __start__stack:
      00000E                        245 	.ds	1
                                    246 
                                    247 ;--------------------------------------------------------
                                    248 ; indirectly addressable internal ram data
                                    249 ;--------------------------------------------------------
                                    250 	.area ISEG    (DATA)
                                    251 ;--------------------------------------------------------
                                    252 ; absolute internal ram data
                                    253 ;--------------------------------------------------------
                                    254 	.area IABS    (ABS,DATA)
                                    255 	.area IABS    (ABS,DATA)
                                    256 ;--------------------------------------------------------
                                    257 ; bit data
                                    258 ;--------------------------------------------------------
                                    259 	.area BSEG    (BIT)
                                    260 ;--------------------------------------------------------
                                    261 ; paged external ram data
                                    262 ;--------------------------------------------------------
                                    263 	.area PSEG    (PAG,XDATA)
                                    264 ;--------------------------------------------------------
                                    265 ; external ram data
                                    266 ;--------------------------------------------------------
                                    267 	.area XSEG    (XDATA)
                                    268 ;--------------------------------------------------------
                                    269 ; absolute external ram data
                                    270 ;--------------------------------------------------------
                                    271 	.area XABS    (ABS,XDATA)
                                    272 ;--------------------------------------------------------
                                    273 ; external initialized ram data
                                    274 ;--------------------------------------------------------
                                    275 	.area XISEG   (XDATA)
                                    276 	.area HOME    (CODE)
                                    277 	.area GSINIT0 (CODE)
                                    278 	.area GSINIT1 (CODE)
                                    279 	.area GSINIT2 (CODE)
                                    280 	.area GSINIT3 (CODE)
                                    281 	.area GSINIT4 (CODE)
                                    282 	.area GSINIT5 (CODE)
                                    283 	.area GSINIT  (CODE)
                                    284 	.area GSFINAL (CODE)
                                    285 	.area CSEG    (CODE)
                                    286 ;--------------------------------------------------------
                                    287 ; interrupt vector 
                                    288 ;--------------------------------------------------------
                                    289 	.area HOME    (CODE)
      000000                        290 __interrupt_vect:
      000000 02 00 29         [24]  291 	ljmp	__sdcc_gsinit_startup
      000003 32               [24]  292 	reti
      000004                        293 	.ds	7
      00000B 32               [24]  294 	reti
      00000C                        295 	.ds	7
      000013 32               [24]  296 	reti
      000014                        297 	.ds	7
      00001B 32               [24]  298 	reti
      00001C                        299 	.ds	7
      000023 02 00 FE         [24]  300 	ljmp	_ser
                                    301 ;--------------------------------------------------------
                                    302 ; global & static initialisations
                                    303 ;--------------------------------------------------------
                                    304 	.area HOME    (CODE)
                                    305 	.area GSINIT  (CODE)
                                    306 	.area GSFINAL (CODE)
                                    307 	.area GSINIT  (CODE)
                                    308 	.globl __sdcc_gsinit_startup
                                    309 	.globl __sdcc_program_startup
                                    310 	.globl __start__stack
                                    311 	.globl __mcs51_genXINIT
                                    312 	.globl __mcs51_genXRAMCLEAR
                                    313 	.globl __mcs51_genRAMCLEAR
                                    314 ;	serial_basic.c:28: unsigned int led_times = 0;
      000082 E4               [12]  315 	clr	a
      000083 F5 0A            [12]  316 	mov	_led_times,a
      000085 F5 0B            [12]  317 	mov	(_led_times + 1),a
                                    318 ;	serial_basic.c:29: unsigned int flag = 1;
      000087 75 0C 01         [24]  319 	mov	_flag,#0x01
                                    320 ;	1-genFromRTrack replaced	mov	(_flag + 1),#0x00
      00008A F5 0D            [12]  321 	mov	(_flag + 1),a
                                    322 	.area GSFINAL (CODE)
      00008C 02 00 26         [24]  323 	ljmp	__sdcc_program_startup
                                    324 ;--------------------------------------------------------
                                    325 ; Home
                                    326 ;--------------------------------------------------------
                                    327 	.area HOME    (CODE)
                                    328 	.area HOME    (CODE)
      000026                        329 __sdcc_program_startup:
      000026 02 00 D0         [24]  330 	ljmp	_main
                                    331 ;	return from main will return to caller
                                    332 ;--------------------------------------------------------
                                    333 ; code
                                    334 ;--------------------------------------------------------
                                    335 	.area CSEG    (CODE)
                                    336 ;------------------------------------------------------------
                                    337 ;Allocation info for local variables in function 'change_led'
                                    338 ;------------------------------------------------------------
                                    339 ;	serial_basic.c:30: void change_led(){
                                    340 ;	-----------------------------------------
                                    341 ;	 function change_led
                                    342 ;	-----------------------------------------
      00008F                        343 _change_led:
                           000007   344 	ar7 = 0x07
                           000006   345 	ar6 = 0x06
                           000005   346 	ar5 = 0x05
                           000004   347 	ar4 = 0x04
                           000003   348 	ar3 = 0x03
                           000002   349 	ar2 = 0x02
                           000001   350 	ar1 = 0x01
                           000000   351 	ar0 = 0x00
                                    352 ;	serial_basic.c:31: if (led_times >= 8) {
      00008F C3               [12]  353 	clr	c
      000090 E5 0A            [12]  354 	mov	a,_led_times
      000092 94 08            [12]  355 	subb	a,#0x08
      000094 E5 0B            [12]  356 	mov	a,(_led_times + 1)
      000096 94 00            [12]  357 	subb	a,#0x00
      000098 40 19            [24]  358 	jc	00102$
                                    359 ;	serial_basic.c:32: flag = !flag;
      00009A E5 0C            [12]  360 	mov	a,_flag
      00009C 45 0D            [12]  361 	orl	a,(_flag + 1)
      00009E B4 01 00         [24]  362 	cjne	a,#0x01,00116$
      0000A1                        363 00116$:
      0000A1 E4               [12]  364 	clr	a
      0000A2 33               [12]  365 	rlc	a
      0000A3 FF               [12]  366 	mov	r7,a
      0000A4 8F 0C            [24]  367 	mov	_flag,r7
      0000A6 33               [12]  368 	rlc	a
      0000A7 95 E0            [12]  369 	subb	a,acc
      0000A9 F5 0D            [12]  370 	mov	(_flag + 1),a
                                    371 ;	serial_basic.c:33: P0 = 0xff;
      0000AB 75 80 FF         [24]  372 	mov	_P0,#0xff
                                    373 ;	serial_basic.c:34: led_times = 0;
      0000AE E4               [12]  374 	clr	a
      0000AF F5 0A            [12]  375 	mov	_led_times,a
      0000B1 F5 0B            [12]  376 	mov	(_led_times + 1),a
      0000B3                        377 00102$:
                                    378 ;	serial_basic.c:36: P0 = flag ? P0 << 1 : P0 >> 1;
      0000B3 E5 0C            [12]  379 	mov	a,_flag
      0000B5 45 0D            [12]  380 	orl	a,(_flag + 1)
      0000B7 60 07            [24]  381 	jz	00105$
      0000B9 E5 80            [12]  382 	mov	a,_P0
      0000BB 25 E0            [12]  383 	add	a,acc
      0000BD FF               [12]  384 	mov	r7,a
      0000BE 80 05            [24]  385 	sjmp	00106$
      0000C0                        386 00105$:
      0000C0 E5 80            [12]  387 	mov	a,_P0
      0000C2 C3               [12]  388 	clr	c
      0000C3 13               [12]  389 	rrc	a
      0000C4 FF               [12]  390 	mov	r7,a
      0000C5                        391 00106$:
      0000C5 8F 80            [24]  392 	mov	_P0,r7
                                    393 ;	serial_basic.c:37: led_times++;
      0000C7 05 0A            [12]  394 	inc	_led_times
      0000C9 E4               [12]  395 	clr	a
      0000CA B5 0A 02         [24]  396 	cjne	a,_led_times,00118$
      0000CD 05 0B            [12]  397 	inc	(_led_times + 1)
      0000CF                        398 00118$:
                                    399 ;	serial_basic.c:38: }
      0000CF 22               [24]  400 	ret
                                    401 ;------------------------------------------------------------
                                    402 ;Allocation info for local variables in function 'main'
                                    403 ;------------------------------------------------------------
                                    404 ;	serial_basic.c:40: void main()
                                    405 ;	-----------------------------------------
                                    406 ;	 function main
                                    407 ;	-----------------------------------------
      0000D0                        408 _main:
                                    409 ;	serial_basic.c:42: TMOD=0x20;		   //用定时器设置串口波特率	   9600 
      0000D0 75 89 20         [24]  410 	mov	_TMOD,#0x20
                                    411 ;	serial_basic.c:43: TH1=0xfd;
      0000D3 75 8D FD         [24]  412 	mov	_TH1,#0xfd
                                    413 ;	serial_basic.c:44: TL1=0xfd;
      0000D6 75 8B FD         [24]  414 	mov	_TL1,#0xfd
                                    415 ;	serial_basic.c:45: TR1=1;
                                    416 ;	assignBit
      0000D9 D2 8E            [12]  417 	setb	_TR1
                                    418 ;	serial_basic.c:46: REN=1;          //串口初始化
                                    419 ;	assignBit
      0000DB D2 9C            [12]  420 	setb	_REN
                                    421 ;	serial_basic.c:47: SM0=0;
                                    422 ;	assignBit
      0000DD C2 9F            [12]  423 	clr	_SM0
                                    424 ;	serial_basic.c:48: SM1=1;
                                    425 ;	assignBit
      0000DF D2 9E            [12]  426 	setb	_SM1
                                    427 ;	serial_basic.c:49: EA=1;           //开启总中断
                                    428 ;	assignBit
      0000E1 D2 AF            [12]  429 	setb	_EA
                                    430 ;	serial_basic.c:50: ES=1;
                                    431 ;	assignBit
      0000E3 D2 AC            [12]  432 	setb	_ES
                                    433 ;	serial_basic.c:51: while(1)
      0000E5                        434 00107$:
                                    435 ;	serial_basic.c:53: if(num==1)    //判断是否有串口数据的传送
      0000E5 74 01            [12]  436 	mov	a,#0x01
      0000E7 B5 08 FB         [24]  437 	cjne	a,_num,00107$
                                    438 ;	serial_basic.c:55: change_led();
      0000EA 12 00 8F         [24]  439 	lcall	_change_led
                                    440 ;	serial_basic.c:56: ES=0;
                                    441 ;	assignBit
      0000ED C2 AC            [12]  442 	clr	_ES
                                    443 ;	serial_basic.c:57: num=0;
      0000EF 75 08 00         [24]  444 	mov	_num,#0x00
                                    445 ;	serial_basic.c:58: SBUF=a;			 //发送数据a到SBUF，即将单片机的数据发送到计算机
      0000F2 85 09 99         [24]  446 	mov	_SBUF,_a
                                    447 ;	serial_basic.c:59: while(!TI);
      0000F5                        448 00101$:
                                    449 ;	serial_basic.c:60: TI=0;
                                    450 ;	assignBit
      0000F5 10 99 02         [24]  451 	jbc	_TI,00129$
      0000F8 80 FB            [24]  452 	sjmp	00101$
      0000FA                        453 00129$:
                                    454 ;	serial_basic.c:61: ES=1;
                                    455 ;	assignBit
      0000FA D2 AC            [12]  456 	setb	_ES
                                    457 ;	serial_basic.c:64: }
      0000FC 80 E7            [24]  458 	sjmp	00107$
                                    459 ;------------------------------------------------------------
                                    460 ;Allocation info for local variables in function 'ser'
                                    461 ;------------------------------------------------------------
                                    462 ;	serial_basic.c:65: void ser() __interrupt 4
                                    463 ;	-----------------------------------------
                                    464 ;	 function ser
                                    465 ;	-----------------------------------------
      0000FE                        466 _ser:
                                    467 ;	serial_basic.c:67: RI=0;
                                    468 ;	assignBit
      0000FE C2 98            [12]  469 	clr	_RI
                                    470 ;	serial_basic.c:68: P1=SBUF;			//接收数据SBUF，即将计算机的数据接收。
      000100 85 99 90         [24]  471 	mov	_P1,_SBUF
                                    472 ;	serial_basic.c:69: a=SBUF;
      000103 85 99 09         [24]  473 	mov	_a,_SBUF
                                    474 ;	serial_basic.c:70: num=1;
      000106 75 08 01         [24]  475 	mov	_num,#0x01
                                    476 ;	serial_basic.c:71: }
      000109 32               [24]  477 	reti
                                    478 ;	eliminated unneeded mov psw,# (no regs used in bank)
                                    479 ;	eliminated unneeded push/pop psw
                                    480 ;	eliminated unneeded push/pop dpl
                                    481 ;	eliminated unneeded push/pop dph
                                    482 ;	eliminated unneeded push/pop b
                                    483 ;	eliminated unneeded push/pop acc
                                    484 	.area CSEG    (CODE)
                                    485 	.area CONST   (CODE)
                                    486 	.area XINIT   (CODE)
                                    487 	.area CABS    (ABS,CODE)
