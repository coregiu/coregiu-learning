                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module serial_led
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _serial
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
                                    110 	.globl _num
                                    111 	.globl _dat
                                    112 ;--------------------------------------------------------
                                    113 ; special function registers
                                    114 ;--------------------------------------------------------
                                    115 	.area RSEG    (ABS,DATA)
      000000                        116 	.org 0x0000
                           000080   117 _P0	=	0x0080
                           000081   118 _SP	=	0x0081
                           000082   119 _DPL	=	0x0082
                           000083   120 _DPH	=	0x0083
                           000087   121 _PCON	=	0x0087
                           000088   122 _TCON	=	0x0088
                           000089   123 _TMOD	=	0x0089
                           00008A   124 _TL0	=	0x008a
                           00008B   125 _TL1	=	0x008b
                           00008C   126 _TH0	=	0x008c
                           00008D   127 _TH1	=	0x008d
                           000090   128 _P1	=	0x0090
                           000098   129 _SCON	=	0x0098
                           000099   130 _SBUF	=	0x0099
                           0000A0   131 _P2	=	0x00a0
                           0000A8   132 _IE	=	0x00a8
                           0000B0   133 _P3	=	0x00b0
                           0000B8   134 _IP	=	0x00b8
                           0000D0   135 _PSW	=	0x00d0
                           0000E0   136 _ACC	=	0x00e0
                           0000F0   137 _B	=	0x00f0
                                    138 ;--------------------------------------------------------
                                    139 ; special function bits
                                    140 ;--------------------------------------------------------
                                    141 	.area RSEG    (ABS,DATA)
      000000                        142 	.org 0x0000
                           000080   143 _P0_0	=	0x0080
                           000081   144 _P0_1	=	0x0081
                           000082   145 _P0_2	=	0x0082
                           000083   146 _P0_3	=	0x0083
                           000084   147 _P0_4	=	0x0084
                           000085   148 _P0_5	=	0x0085
                           000086   149 _P0_6	=	0x0086
                           000087   150 _P0_7	=	0x0087
                           000088   151 _IT0	=	0x0088
                           000089   152 _IE0	=	0x0089
                           00008A   153 _IT1	=	0x008a
                           00008B   154 _IE1	=	0x008b
                           00008C   155 _TR0	=	0x008c
                           00008D   156 _TF0	=	0x008d
                           00008E   157 _TR1	=	0x008e
                           00008F   158 _TF1	=	0x008f
                           000090   159 _P1_0	=	0x0090
                           000091   160 _P1_1	=	0x0091
                           000092   161 _P1_2	=	0x0092
                           000093   162 _P1_3	=	0x0093
                           000094   163 _P1_4	=	0x0094
                           000095   164 _P1_5	=	0x0095
                           000096   165 _P1_6	=	0x0096
                           000097   166 _P1_7	=	0x0097
                           000098   167 _RI	=	0x0098
                           000099   168 _TI	=	0x0099
                           00009A   169 _RB8	=	0x009a
                           00009B   170 _TB8	=	0x009b
                           00009C   171 _REN	=	0x009c
                           00009D   172 _SM2	=	0x009d
                           00009E   173 _SM1	=	0x009e
                           00009F   174 _SM0	=	0x009f
                           0000A0   175 _P2_0	=	0x00a0
                           0000A1   176 _P2_1	=	0x00a1
                           0000A2   177 _P2_2	=	0x00a2
                           0000A3   178 _P2_3	=	0x00a3
                           0000A4   179 _P2_4	=	0x00a4
                           0000A5   180 _P2_5	=	0x00a5
                           0000A6   181 _P2_6	=	0x00a6
                           0000A7   182 _P2_7	=	0x00a7
                           0000A8   183 _EX0	=	0x00a8
                           0000A9   184 _ET0	=	0x00a9
                           0000AA   185 _EX1	=	0x00aa
                           0000AB   186 _ET1	=	0x00ab
                           0000AC   187 _ES	=	0x00ac
                           0000AF   188 _EA	=	0x00af
                           0000B0   189 _P3_0	=	0x00b0
                           0000B1   190 _P3_1	=	0x00b1
                           0000B2   191 _P3_2	=	0x00b2
                           0000B3   192 _P3_3	=	0x00b3
                           0000B4   193 _P3_4	=	0x00b4
                           0000B5   194 _P3_5	=	0x00b5
                           0000B6   195 _P3_6	=	0x00b6
                           0000B7   196 _P3_7	=	0x00b7
                           0000B0   197 _RXD	=	0x00b0
                           0000B1   198 _TXD	=	0x00b1
                           0000B2   199 _INT0	=	0x00b2
                           0000B3   200 _INT1	=	0x00b3
                           0000B4   201 _T0	=	0x00b4
                           0000B5   202 _T1	=	0x00b5
                           0000B6   203 _WR	=	0x00b6
                           0000B7   204 _RD	=	0x00b7
                           0000B8   205 _PX0	=	0x00b8
                           0000B9   206 _PT0	=	0x00b9
                           0000BA   207 _PX1	=	0x00ba
                           0000BB   208 _PT1	=	0x00bb
                           0000BC   209 _PS	=	0x00bc
                           0000D0   210 _P	=	0x00d0
                           0000D1   211 _F1	=	0x00d1
                           0000D2   212 _OV	=	0x00d2
                           0000D3   213 _RS0	=	0x00d3
                           0000D4   214 _RS1	=	0x00d4
                           0000D5   215 _F0	=	0x00d5
                           0000D6   216 _AC	=	0x00d6
                           0000D7   217 _CY	=	0x00d7
                                    218 ;--------------------------------------------------------
                                    219 ; overlayable register banks
                                    220 ;--------------------------------------------------------
                                    221 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        222 	.ds 8
                                    223 ;--------------------------------------------------------
                                    224 ; internal ram data
                                    225 ;--------------------------------------------------------
                                    226 	.area DSEG    (DATA)
      000008                        227 _dat::
      000008                        228 	.ds 1
      000009                        229 _num::
      000009                        230 	.ds 1
                                    231 ;--------------------------------------------------------
                                    232 ; overlayable items in internal ram 
                                    233 ;--------------------------------------------------------
                                    234 ;--------------------------------------------------------
                                    235 ; Stack segment in internal ram 
                                    236 ;--------------------------------------------------------
                                    237 	.area	SSEG
      00000A                        238 __start__stack:
      00000A                        239 	.ds	1
                                    240 
                                    241 ;--------------------------------------------------------
                                    242 ; indirectly addressable internal ram data
                                    243 ;--------------------------------------------------------
                                    244 	.area ISEG    (DATA)
                                    245 ;--------------------------------------------------------
                                    246 ; absolute internal ram data
                                    247 ;--------------------------------------------------------
                                    248 	.area IABS    (ABS,DATA)
                                    249 	.area IABS    (ABS,DATA)
                                    250 ;--------------------------------------------------------
                                    251 ; bit data
                                    252 ;--------------------------------------------------------
                                    253 	.area BSEG    (BIT)
                                    254 ;--------------------------------------------------------
                                    255 ; paged external ram data
                                    256 ;--------------------------------------------------------
                                    257 	.area PSEG    (PAG,XDATA)
                                    258 ;--------------------------------------------------------
                                    259 ; external ram data
                                    260 ;--------------------------------------------------------
                                    261 	.area XSEG    (XDATA)
                                    262 ;--------------------------------------------------------
                                    263 ; absolute external ram data
                                    264 ;--------------------------------------------------------
                                    265 	.area XABS    (ABS,XDATA)
                                    266 ;--------------------------------------------------------
                                    267 ; external initialized ram data
                                    268 ;--------------------------------------------------------
                                    269 	.area XISEG   (XDATA)
                                    270 	.area HOME    (CODE)
                                    271 	.area GSINIT0 (CODE)
                                    272 	.area GSINIT1 (CODE)
                                    273 	.area GSINIT2 (CODE)
                                    274 	.area GSINIT3 (CODE)
                                    275 	.area GSINIT4 (CODE)
                                    276 	.area GSINIT5 (CODE)
                                    277 	.area GSINIT  (CODE)
                                    278 	.area GSFINAL (CODE)
                                    279 	.area CSEG    (CODE)
                                    280 ;--------------------------------------------------------
                                    281 ; interrupt vector 
                                    282 ;--------------------------------------------------------
                                    283 	.area HOME    (CODE)
      000000                        284 __interrupt_vect:
      000000 02 00 29         [24]  285 	ljmp	__sdcc_gsinit_startup
      000003 32               [24]  286 	reti
      000004                        287 	.ds	7
      00000B 32               [24]  288 	reti
      00000C                        289 	.ds	7
      000013 32               [24]  290 	reti
      000014                        291 	.ds	7
      00001B 32               [24]  292 	reti
      00001C                        293 	.ds	7
      000023 02 01 06         [24]  294 	ljmp	_serial
                                    295 ;--------------------------------------------------------
                                    296 ; global & static initialisations
                                    297 ;--------------------------------------------------------
                                    298 	.area HOME    (CODE)
                                    299 	.area GSINIT  (CODE)
                                    300 	.area GSFINAL (CODE)
                                    301 	.area GSINIT  (CODE)
                                    302 	.globl __sdcc_gsinit_startup
                                    303 	.globl __sdcc_program_startup
                                    304 	.globl __start__stack
                                    305 	.globl __mcs51_genXINIT
                                    306 	.globl __mcs51_genXRAMCLEAR
                                    307 	.globl __mcs51_genRAMCLEAR
                                    308 	.area GSFINAL (CODE)
      000082 02 00 26         [24]  309 	ljmp	__sdcc_program_startup
                                    310 ;--------------------------------------------------------
                                    311 ; Home
                                    312 ;--------------------------------------------------------
                                    313 	.area HOME    (CODE)
                                    314 	.area HOME    (CODE)
      000026                        315 __sdcc_program_startup:
      000026 02 00 88         [24]  316 	ljmp	_main
                                    317 ;	return from main will return to caller
                                    318 ;--------------------------------------------------------
                                    319 ; code
                                    320 ;--------------------------------------------------------
                                    321 	.area CSEG    (CODE)
                                    322 ;------------------------------------------------------------
                                    323 ;Allocation info for local variables in function 'change_led'
                                    324 ;------------------------------------------------------------
                                    325 ;	serial_led.c:34: void change_led(){
                                    326 ;	-----------------------------------------
                                    327 ;	 function change_led
                                    328 ;	-----------------------------------------
      000085                        329 _change_led:
                           000007   330 	ar7 = 0x07
                           000006   331 	ar6 = 0x06
                           000005   332 	ar5 = 0x05
                           000004   333 	ar4 = 0x04
                           000003   334 	ar3 = 0x03
                           000002   335 	ar2 = 0x02
                           000001   336 	ar1 = 0x01
                           000000   337 	ar0 = 0x00
                                    338 ;	serial_led.c:35: P0_7 = !P0_7;
      000085 B2 87            [12]  339 	cpl	_P0_7
                                    340 ;	serial_led.c:36: }
      000087 22               [24]  341 	ret
                                    342 ;------------------------------------------------------------
                                    343 ;Allocation info for local variables in function 'main'
                                    344 ;------------------------------------------------------------
                                    345 ;	serial_led.c:38: void main()
                                    346 ;	-----------------------------------------
                                    347 ;	 function main
                                    348 ;	-----------------------------------------
      000088                        349 _main:
                                    350 ;	serial_led.c:40: TMOD=0x20;		   //用定时器设置串口波特率
      000088 75 89 20         [24]  351 	mov	_TMOD,#0x20
                                    352 ;	serial_led.c:41: TH1=0xfd;
      00008B 75 8D FD         [24]  353 	mov	_TH1,#0xfd
                                    354 ;	serial_led.c:42: TL1=0xfd;
      00008E 75 8B FD         [24]  355 	mov	_TL1,#0xfd
                                    356 ;	serial_led.c:43: TR1=1;
                                    357 ;	assignBit
      000091 D2 8E            [12]  358 	setb	_TR1
                                    359 ;	serial_led.c:44: REN=1;          //串口初始化
                                    360 ;	assignBit
      000093 D2 9C            [12]  361 	setb	_REN
                                    362 ;	serial_led.c:45: SM0=0;
                                    363 ;	assignBit
      000095 C2 9F            [12]  364 	clr	_SM0
                                    365 ;	serial_led.c:46: SM1=1;
                                    366 ;	assignBit
      000097 D2 9E            [12]  367 	setb	_SM1
                                    368 ;	serial_led.c:47: EA=1;           //开启总中断
                                    369 ;	assignBit
      000099 D2 AF            [12]  370 	setb	_EA
                                    371 ;	serial_led.c:48: ES=1;			//开启串口中断
                                    372 ;	assignBit
      00009B D2 AC            [12]  373 	setb	_ES
                                    374 ;	serial_led.c:49: while(1)
      00009D                        375 00114$:
                                    376 ;	serial_led.c:51: if(num==1)
      00009D 74 01            [12]  377 	mov	a,#0x01
      00009F B5 09 FB         [24]  378 	cjne	a,_num,00114$
                                    379 ;	serial_led.c:53: change_led();
      0000A2 12 00 85         [24]  380 	lcall	_change_led
                                    381 ;	serial_led.c:54: switch(dat)    //判断串口接收的数据
      0000A5 74 D0            [12]  382 	mov	a,#0x100 - 0x30
      0000A7 25 08            [12]  383 	add	a,_dat
      0000A9 40 03            [24]  384 	jc	00136$
      0000AB 02 00 FE         [24]  385 	ljmp	00110$
      0000AE                        386 00136$:
      0000AE E5 08            [12]  387 	mov	a,_dat
      0000B0 24 C8            [12]  388 	add	a,#0xff - 0x37
      0000B2 50 03            [24]  389 	jnc	00137$
      0000B4 02 00 FE         [24]  390 	ljmp	00110$
      0000B7                        391 00137$:
      0000B7 E5 08            [12]  392 	mov	a,_dat
      0000B9 24 D0            [12]  393 	add	a,#0xd0
      0000BB FF               [12]  394 	mov	r7,a
      0000BC 24 0A            [12]  395 	add	a,#(00138$-3-.)
      0000BE 83               [24]  396 	movc	a,@a+pc
      0000BF F5 82            [12]  397 	mov	dpl,a
      0000C1 EF               [12]  398 	mov	a,r7
      0000C2 24 0C            [12]  399 	add	a,#(00139$-3-.)
      0000C4 83               [24]  400 	movc	a,@a+pc
      0000C5 F5 83            [12]  401 	mov	dph,a
      0000C7 E4               [12]  402 	clr	a
      0000C8 73               [24]  403 	jmp	@a+dptr
      0000C9                        404 00138$:
      0000C9 F5                     405 	.db	00108$
      0000CA D9                     406 	.db	00101$
      0000CB DD                     407 	.db	00102$
      0000CC E1                     408 	.db	00103$
      0000CD E5                     409 	.db	00104$
      0000CE E9                     410 	.db	00105$
      0000CF ED                     411 	.db	00106$
      0000D0 F1                     412 	.db	00107$
      0000D1                        413 00139$:
      0000D1 00                     414 	.db	00108$>>8
      0000D2 00                     415 	.db	00101$>>8
      0000D3 00                     416 	.db	00102$>>8
      0000D4 00                     417 	.db	00103$>>8
      0000D5 00                     418 	.db	00104$>>8
      0000D6 00                     419 	.db	00105$>>8
      0000D7 00                     420 	.db	00106$>>8
      0000D8 00                     421 	.db	00107$>>8
                                    422 ;	serial_led.c:55: {	case '1':P0_0=0;break;  //点亮第1个灯
      0000D9                        423 00101$:
                                    424 ;	assignBit
      0000D9 C2 80            [12]  425 	clr	_P0_0
                                    426 ;	serial_led.c:56: case '2':P0_1=0;break;  //点亮第2个灯
      0000DB 80 21            [24]  427 	sjmp	00110$
      0000DD                        428 00102$:
                                    429 ;	assignBit
      0000DD C2 81            [12]  430 	clr	_P0_1
                                    431 ;	serial_led.c:57: case '3':P0_2=0;break;  //点亮第3个灯
      0000DF 80 1D            [24]  432 	sjmp	00110$
      0000E1                        433 00103$:
                                    434 ;	assignBit
      0000E1 C2 82            [12]  435 	clr	_P0_2
                                    436 ;	serial_led.c:58: case '4':P0_3=0;break;  //点亮第4个灯
      0000E3 80 19            [24]  437 	sjmp	00110$
      0000E5                        438 00104$:
                                    439 ;	assignBit
      0000E5 C2 83            [12]  440 	clr	_P0_3
                                    441 ;	serial_led.c:59: case '5':P0_4=0;break;  //点亮第5个灯
      0000E7 80 15            [24]  442 	sjmp	00110$
      0000E9                        443 00105$:
                                    444 ;	assignBit
      0000E9 C2 84            [12]  445 	clr	_P0_4
                                    446 ;	serial_led.c:60: case '6':P0_5=0;break;  //点亮第6个灯
      0000EB 80 11            [24]  447 	sjmp	00110$
      0000ED                        448 00106$:
                                    449 ;	assignBit
      0000ED C2 85            [12]  450 	clr	_P0_5
                                    451 ;	serial_led.c:61: case '7':P0_6=0;break;  //点亮第7个灯
      0000EF 80 0D            [24]  452 	sjmp	00110$
      0000F1                        453 00107$:
                                    454 ;	assignBit
      0000F1 C2 86            [12]  455 	clr	_P0_6
                                    456 ;	serial_led.c:62: case '0':P0|=0x7f;break;  //清空所有的灯
      0000F3 80 09            [24]  457 	sjmp	00110$
      0000F5                        458 00108$:
      0000F5 AE 80            [24]  459 	mov	r6,_P0
      0000F7 7F 00            [12]  460 	mov	r7,#0x00
      0000F9 43 06 7F         [24]  461 	orl	ar6,#0x7f
      0000FC 8E 80            [24]  462 	mov	_P0,r6
                                    463 ;	serial_led.c:64: }
      0000FE                        464 00110$:
                                    465 ;	serial_led.c:65: ES=1;		 //打开串口中断	
                                    466 ;	assignBit
      0000FE D2 AC            [12]  467 	setb	_ES
                                    468 ;	serial_led.c:66: num=0;
      000100 75 09 00         [24]  469 	mov	_num,#0x00
                                    470 ;	serial_led.c:70: }
      000103 02 00 9D         [24]  471 	ljmp	00114$
                                    472 ;------------------------------------------------------------
                                    473 ;Allocation info for local variables in function 'serial'
                                    474 ;------------------------------------------------------------
                                    475 ;	serial_led.c:72: void serial() __interrupt 4
                                    476 ;	-----------------------------------------
                                    477 ;	 function serial
                                    478 ;	-----------------------------------------
      000106                        479 _serial:
                                    480 ;	serial_led.c:74: RI=0;
                                    481 ;	assignBit
      000106 C2 98            [12]  482 	clr	_RI
                                    483 ;	serial_led.c:75: dat = SBUF;				//接收数据SBUF，即将计算机的数据接收。
      000108 85 99 08         [24]  484 	mov	_dat,_SBUF
                                    485 ;	serial_led.c:76: ES=0;				    //关闭串口中断
                                    486 ;	assignBit
      00010B C2 AC            [12]  487 	clr	_ES
                                    488 ;	serial_led.c:77: num=1;
      00010D 75 09 01         [24]  489 	mov	_num,#0x01
                                    490 ;	serial_led.c:78: }
      000110 32               [24]  491 	reti
                                    492 ;	eliminated unneeded mov psw,# (no regs used in bank)
                                    493 ;	eliminated unneeded push/pop psw
                                    494 ;	eliminated unneeded push/pop dpl
                                    495 ;	eliminated unneeded push/pop dph
                                    496 ;	eliminated unneeded push/pop b
                                    497 ;	eliminated unneeded push/pop acc
                                    498 	.area CSEG    (CODE)
                                    499 	.area CONST   (CODE)
                                    500 	.area XINIT   (CODE)
                                    501 	.area CABS    (ABS,CODE)
