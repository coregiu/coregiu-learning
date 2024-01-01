                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module servo
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _Key_INT_0
                                     12 	.globl _main
                                     13 	.globl _CY
                                     14 	.globl _AC
                                     15 	.globl _F0
                                     16 	.globl _RS1
                                     17 	.globl _RS0
                                     18 	.globl _OV
                                     19 	.globl _F1
                                     20 	.globl _P
                                     21 	.globl _PS
                                     22 	.globl _PT1
                                     23 	.globl _PX1
                                     24 	.globl _PT0
                                     25 	.globl _PX0
                                     26 	.globl _RD
                                     27 	.globl _WR
                                     28 	.globl _T1
                                     29 	.globl _T0
                                     30 	.globl _INT1
                                     31 	.globl _INT0
                                     32 	.globl _TXD
                                     33 	.globl _RXD
                                     34 	.globl _P3_7
                                     35 	.globl _P3_6
                                     36 	.globl _P3_5
                                     37 	.globl _P3_4
                                     38 	.globl _P3_3
                                     39 	.globl _P3_2
                                     40 	.globl _P3_1
                                     41 	.globl _P3_0
                                     42 	.globl _EA
                                     43 	.globl _ES
                                     44 	.globl _ET1
                                     45 	.globl _EX1
                                     46 	.globl _ET0
                                     47 	.globl _EX0
                                     48 	.globl _P2_7
                                     49 	.globl _P2_6
                                     50 	.globl _P2_5
                                     51 	.globl _P2_4
                                     52 	.globl _P2_3
                                     53 	.globl _P2_2
                                     54 	.globl _P2_1
                                     55 	.globl _P2_0
                                     56 	.globl _SM0
                                     57 	.globl _SM1
                                     58 	.globl _SM2
                                     59 	.globl _REN
                                     60 	.globl _TB8
                                     61 	.globl _RB8
                                     62 	.globl _TI
                                     63 	.globl _RI
                                     64 	.globl _P1_7
                                     65 	.globl _P1_6
                                     66 	.globl _P1_5
                                     67 	.globl _P1_4
                                     68 	.globl _P1_3
                                     69 	.globl _P1_2
                                     70 	.globl _P1_1
                                     71 	.globl _P1_0
                                     72 	.globl _TF1
                                     73 	.globl _TR1
                                     74 	.globl _TF0
                                     75 	.globl _TR0
                                     76 	.globl _IE1
                                     77 	.globl _IT1
                                     78 	.globl _IE0
                                     79 	.globl _IT0
                                     80 	.globl _P0_7
                                     81 	.globl _P0_6
                                     82 	.globl _P0_5
                                     83 	.globl _P0_4
                                     84 	.globl _P0_3
                                     85 	.globl _P0_2
                                     86 	.globl _P0_1
                                     87 	.globl _P0_0
                                     88 	.globl _B
                                     89 	.globl _ACC
                                     90 	.globl _PSW
                                     91 	.globl _IP
                                     92 	.globl _P3
                                     93 	.globl _IE
                                     94 	.globl _P2
                                     95 	.globl _SBUF
                                     96 	.globl _SCON
                                     97 	.globl _P1
                                     98 	.globl _TH1
                                     99 	.globl _TH0
                                    100 	.globl _TL1
                                    101 	.globl _TL0
                                    102 	.globl _TMOD
                                    103 	.globl _TCON
                                    104 	.globl _PCON
                                    105 	.globl _DPH
                                    106 	.globl _DPL
                                    107 	.globl _SP
                                    108 	.globl _P0
                                    109 	.globl _servo_control_PARM_2
                                    110 	.globl _pca9685_write_PARM_2
                                    111 	.globl _delayms
                                    112 	.globl _delayus
                                    113 	.globl _init_iic
                                    114 	.globl _start_iic
                                    115 	.globl _stop_iic
                                    116 	.globl _ack_iic
                                    117 	.globl _write_byte
                                    118 	.globl _read_byte
                                    119 	.globl _pca9685_write
                                    120 	.globl _pca9685_read
                                    121 	.globl _reset_pca9685
                                    122 	.globl _set_pwm_freq
                                    123 	.globl _servo_control
                                    124 	.globl _init_pca9685
                                    125 ;--------------------------------------------------------
                                    126 ; special function registers
                                    127 ;--------------------------------------------------------
                                    128 	.area RSEG    (ABS,DATA)
      000000                        129 	.org 0x0000
                           000080   130 _P0	=	0x0080
                           000081   131 _SP	=	0x0081
                           000082   132 _DPL	=	0x0082
                           000083   133 _DPH	=	0x0083
                           000087   134 _PCON	=	0x0087
                           000088   135 _TCON	=	0x0088
                           000089   136 _TMOD	=	0x0089
                           00008A   137 _TL0	=	0x008a
                           00008B   138 _TL1	=	0x008b
                           00008C   139 _TH0	=	0x008c
                           00008D   140 _TH1	=	0x008d
                           000090   141 _P1	=	0x0090
                           000098   142 _SCON	=	0x0098
                           000099   143 _SBUF	=	0x0099
                           0000A0   144 _P2	=	0x00a0
                           0000A8   145 _IE	=	0x00a8
                           0000B0   146 _P3	=	0x00b0
                           0000B8   147 _IP	=	0x00b8
                           0000D0   148 _PSW	=	0x00d0
                           0000E0   149 _ACC	=	0x00e0
                           0000F0   150 _B	=	0x00f0
                                    151 ;--------------------------------------------------------
                                    152 ; special function bits
                                    153 ;--------------------------------------------------------
                                    154 	.area RSEG    (ABS,DATA)
      000000                        155 	.org 0x0000
                           000080   156 _P0_0	=	0x0080
                           000081   157 _P0_1	=	0x0081
                           000082   158 _P0_2	=	0x0082
                           000083   159 _P0_3	=	0x0083
                           000084   160 _P0_4	=	0x0084
                           000085   161 _P0_5	=	0x0085
                           000086   162 _P0_6	=	0x0086
                           000087   163 _P0_7	=	0x0087
                           000088   164 _IT0	=	0x0088
                           000089   165 _IE0	=	0x0089
                           00008A   166 _IT1	=	0x008a
                           00008B   167 _IE1	=	0x008b
                           00008C   168 _TR0	=	0x008c
                           00008D   169 _TF0	=	0x008d
                           00008E   170 _TR1	=	0x008e
                           00008F   171 _TF1	=	0x008f
                           000090   172 _P1_0	=	0x0090
                           000091   173 _P1_1	=	0x0091
                           000092   174 _P1_2	=	0x0092
                           000093   175 _P1_3	=	0x0093
                           000094   176 _P1_4	=	0x0094
                           000095   177 _P1_5	=	0x0095
                           000096   178 _P1_6	=	0x0096
                           000097   179 _P1_7	=	0x0097
                           000098   180 _RI	=	0x0098
                           000099   181 _TI	=	0x0099
                           00009A   182 _RB8	=	0x009a
                           00009B   183 _TB8	=	0x009b
                           00009C   184 _REN	=	0x009c
                           00009D   185 _SM2	=	0x009d
                           00009E   186 _SM1	=	0x009e
                           00009F   187 _SM0	=	0x009f
                           0000A0   188 _P2_0	=	0x00a0
                           0000A1   189 _P2_1	=	0x00a1
                           0000A2   190 _P2_2	=	0x00a2
                           0000A3   191 _P2_3	=	0x00a3
                           0000A4   192 _P2_4	=	0x00a4
                           0000A5   193 _P2_5	=	0x00a5
                           0000A6   194 _P2_6	=	0x00a6
                           0000A7   195 _P2_7	=	0x00a7
                           0000A8   196 _EX0	=	0x00a8
                           0000A9   197 _ET0	=	0x00a9
                           0000AA   198 _EX1	=	0x00aa
                           0000AB   199 _ET1	=	0x00ab
                           0000AC   200 _ES	=	0x00ac
                           0000AF   201 _EA	=	0x00af
                           0000B0   202 _P3_0	=	0x00b0
                           0000B1   203 _P3_1	=	0x00b1
                           0000B2   204 _P3_2	=	0x00b2
                           0000B3   205 _P3_3	=	0x00b3
                           0000B4   206 _P3_4	=	0x00b4
                           0000B5   207 _P3_5	=	0x00b5
                           0000B6   208 _P3_6	=	0x00b6
                           0000B7   209 _P3_7	=	0x00b7
                           0000B0   210 _RXD	=	0x00b0
                           0000B1   211 _TXD	=	0x00b1
                           0000B2   212 _INT0	=	0x00b2
                           0000B3   213 _INT1	=	0x00b3
                           0000B4   214 _T0	=	0x00b4
                           0000B5   215 _T1	=	0x00b5
                           0000B6   216 _WR	=	0x00b6
                           0000B7   217 _RD	=	0x00b7
                           0000B8   218 _PX0	=	0x00b8
                           0000B9   219 _PT0	=	0x00b9
                           0000BA   220 _PX1	=	0x00ba
                           0000BB   221 _PT1	=	0x00bb
                           0000BC   222 _PS	=	0x00bc
                           0000D0   223 _P	=	0x00d0
                           0000D1   224 _F1	=	0x00d1
                           0000D2   225 _OV	=	0x00d2
                           0000D3   226 _RS0	=	0x00d3
                           0000D4   227 _RS1	=	0x00d4
                           0000D5   228 _F0	=	0x00d5
                           0000D6   229 _AC	=	0x00d6
                           0000D7   230 _CY	=	0x00d7
                                    231 ;--------------------------------------------------------
                                    232 ; overlayable register banks
                                    233 ;--------------------------------------------------------
                                    234 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        235 	.ds 8
                                    236 ;--------------------------------------------------------
                                    237 ; overlayable bit register bank
                                    238 ;--------------------------------------------------------
                                    239 	.area BIT_BANK	(REL,OVR,DATA)
      000020                        240 bits:
      000020                        241 	.ds 1
                           008000   242 	b0 = bits[0]
                           008100   243 	b1 = bits[1]
                           008200   244 	b2 = bits[2]
                           008300   245 	b3 = bits[3]
                           008400   246 	b4 = bits[4]
                           008500   247 	b5 = bits[5]
                           008600   248 	b6 = bits[6]
                           008700   249 	b7 = bits[7]
                                    250 ;--------------------------------------------------------
                                    251 ; internal ram data
                                    252 ;--------------------------------------------------------
                                    253 	.area DSEG    (DATA)
      000008                        254 _pca9685_write_PARM_2:
      000008                        255 	.ds 1
      000009                        256 _servo_control_PARM_2:
      000009                        257 	.ds 1
                                    258 ;--------------------------------------------------------
                                    259 ; overlayable items in internal ram 
                                    260 ;--------------------------------------------------------
                                    261 	.area	OSEG    (OVR,DATA)
                                    262 ;--------------------------------------------------------
                                    263 ; Stack segment in internal ram 
                                    264 ;--------------------------------------------------------
                                    265 	.area	SSEG
      000021                        266 __start__stack:
      000021                        267 	.ds	1
                                    268 
                                    269 ;--------------------------------------------------------
                                    270 ; indirectly addressable internal ram data
                                    271 ;--------------------------------------------------------
                                    272 	.area ISEG    (DATA)
                                    273 ;--------------------------------------------------------
                                    274 ; absolute internal ram data
                                    275 ;--------------------------------------------------------
                                    276 	.area IABS    (ABS,DATA)
                                    277 	.area IABS    (ABS,DATA)
                                    278 ;--------------------------------------------------------
                                    279 ; bit data
                                    280 ;--------------------------------------------------------
                                    281 	.area BSEG    (BIT)
                                    282 ;--------------------------------------------------------
                                    283 ; paged external ram data
                                    284 ;--------------------------------------------------------
                                    285 	.area PSEG    (PAG,XDATA)
                                    286 ;--------------------------------------------------------
                                    287 ; external ram data
                                    288 ;--------------------------------------------------------
                                    289 	.area XSEG    (XDATA)
                                    290 ;--------------------------------------------------------
                                    291 ; absolute external ram data
                                    292 ;--------------------------------------------------------
                                    293 	.area XABS    (ABS,XDATA)
                                    294 ;--------------------------------------------------------
                                    295 ; external initialized ram data
                                    296 ;--------------------------------------------------------
                                    297 	.area XISEG   (XDATA)
                                    298 	.area HOME    (CODE)
                                    299 	.area GSINIT0 (CODE)
                                    300 	.area GSINIT1 (CODE)
                                    301 	.area GSINIT2 (CODE)
                                    302 	.area GSINIT3 (CODE)
                                    303 	.area GSINIT4 (CODE)
                                    304 	.area GSINIT5 (CODE)
                                    305 	.area GSINIT  (CODE)
                                    306 	.area GSFINAL (CODE)
                                    307 	.area CSEG    (CODE)
                                    308 ;--------------------------------------------------------
                                    309 ; interrupt vector 
                                    310 ;--------------------------------------------------------
                                    311 	.area HOME    (CODE)
      000000                        312 __interrupt_vect:
      000000 02 00 09         [24]  313 	ljmp	__sdcc_gsinit_startup
      000003 02 03 D4         [24]  314 	ljmp	_Key_INT_0
                                    315 ;--------------------------------------------------------
                                    316 ; global & static initialisations
                                    317 ;--------------------------------------------------------
                                    318 	.area HOME    (CODE)
                                    319 	.area GSINIT  (CODE)
                                    320 	.area GSFINAL (CODE)
                                    321 	.area GSINIT  (CODE)
                                    322 	.globl __sdcc_gsinit_startup
                                    323 	.globl __sdcc_program_startup
                                    324 	.globl __start__stack
                                    325 	.globl __mcs51_genXINIT
                                    326 	.globl __mcs51_genXRAMCLEAR
                                    327 	.globl __mcs51_genRAMCLEAR
                                    328 	.area GSFINAL (CODE)
      000062 02 00 06         [24]  329 	ljmp	__sdcc_program_startup
                                    330 ;--------------------------------------------------------
                                    331 ; Home
                                    332 ;--------------------------------------------------------
                                    333 	.area HOME    (CODE)
                                    334 	.area HOME    (CODE)
      000006                        335 __sdcc_program_startup:
      000006 02 03 6E         [24]  336 	ljmp	_main
                                    337 ;	return from main will return to caller
                                    338 ;--------------------------------------------------------
                                    339 ; code
                                    340 ;--------------------------------------------------------
                                    341 	.area CSEG    (CODE)
                                    342 ;------------------------------------------------------------
                                    343 ;Allocation info for local variables in function 'delayms'
                                    344 ;------------------------------------------------------------
                                    345 ;z                         Allocated to registers 
                                    346 ;x                         Allocated to registers r6 r7 
                                    347 ;y                         Allocated to registers r4 r5 
                                    348 ;------------------------------------------------------------
                                    349 ;	servo.c:54: void delayms(uint z)
                                    350 ;	-----------------------------------------
                                    351 ;	 function delayms
                                    352 ;	-----------------------------------------
      000065                        353 _delayms:
                           000007   354 	ar7 = 0x07
                           000006   355 	ar6 = 0x06
                           000005   356 	ar5 = 0x05
                           000004   357 	ar4 = 0x04
                           000003   358 	ar3 = 0x03
                           000002   359 	ar2 = 0x02
                           000001   360 	ar1 = 0x01
                           000000   361 	ar0 = 0x00
      000065 AE 82            [24]  362 	mov	r6,dpl
      000067 AF 83            [24]  363 	mov	r7,dph
                                    364 ;	servo.c:57: for (x = z; x > 0; x--)
      000069                        365 00106$:
      000069 EE               [12]  366 	mov	a,r6
      00006A 4F               [12]  367 	orl	a,r7
      00006B 60 1B            [24]  368 	jz	00108$
                                    369 ;	servo.c:58: for (y = 148; y > 0; y--)
      00006D 7C 94            [12]  370 	mov	r4,#0x94
      00006F 7D 00            [12]  371 	mov	r5,#0x00
      000071                        372 00104$:
      000071 EC               [12]  373 	mov	a,r4
      000072 24 FF            [12]  374 	add	a,#0xff
      000074 FA               [12]  375 	mov	r2,a
      000075 ED               [12]  376 	mov	a,r5
      000076 34 FF            [12]  377 	addc	a,#0xff
      000078 FB               [12]  378 	mov	r3,a
      000079 8A 04            [24]  379 	mov	ar4,r2
      00007B 8B 05            [24]  380 	mov	ar5,r3
      00007D EA               [12]  381 	mov	a,r2
      00007E 4B               [12]  382 	orl	a,r3
      00007F 70 F0            [24]  383 	jnz	00104$
                                    384 ;	servo.c:57: for (x = z; x > 0; x--)
      000081 1E               [12]  385 	dec	r6
      000082 BE FF 01         [24]  386 	cjne	r6,#0xff,00133$
      000085 1F               [12]  387 	dec	r7
      000086                        388 00133$:
      000086 80 E1            [24]  389 	sjmp	00106$
      000088                        390 00108$:
                                    391 ;	servo.c:60: }
      000088 22               [24]  392 	ret
                                    393 ;------------------------------------------------------------
                                    394 ;Allocation info for local variables in function 'delayus'
                                    395 ;------------------------------------------------------------
                                    396 ;	servo.c:68: void delayus()
                                    397 ;	-----------------------------------------
                                    398 ;	 function delayus
                                    399 ;	-----------------------------------------
      000089                        400 _delayus:
                                    401 ;	servo.c:70: _nop_(); //在intrins.h文件里
      000089 00               [12]  402 	NOP	
                                    403 ;	servo.c:71: _nop_();
      00008A 00               [12]  404 	NOP	
                                    405 ;	servo.c:72: _nop_();
      00008B 00               [12]  406 	NOP	
                                    407 ;	servo.c:73: _nop_();
      00008C 00               [12]  408 	NOP	
                                    409 ;	servo.c:74: _nop_();
      00008D 00               [12]  410 	NOP	
                                    411 ;	servo.c:75: }
      00008E 22               [24]  412 	ret
                                    413 ;------------------------------------------------------------
                                    414 ;Allocation info for local variables in function 'init_iic'
                                    415 ;------------------------------------------------------------
                                    416 ;	servo.c:79: void init_iic()
                                    417 ;	-----------------------------------------
                                    418 ;	 function init_iic
                                    419 ;	-----------------------------------------
      00008F                        420 _init_iic:
                                    421 ;	servo.c:81: sda = 1; //sda scl使用前总是被拉高
                                    422 ;	assignBit
      00008F D2 90            [12]  423 	setb	_P1_0
                                    424 ;	servo.c:82: delayus();
      000091 12 00 89         [24]  425 	lcall	_delayus
                                    426 ;	servo.c:83: scl = 1;
                                    427 ;	assignBit
      000094 D2 91            [12]  428 	setb	_P1_1
                                    429 ;	servo.c:84: delayus();
                                    430 ;	servo.c:85: }
      000096 02 00 89         [24]  431 	ljmp	_delayus
                                    432 ;------------------------------------------------------------
                                    433 ;Allocation info for local variables in function 'start_iic'
                                    434 ;------------------------------------------------------------
                                    435 ;	servo.c:89: void start_iic()
                                    436 ;	-----------------------------------------
                                    437 ;	 function start_iic
                                    438 ;	-----------------------------------------
      000099                        439 _start_iic:
                                    440 ;	servo.c:91: sda = 1;
                                    441 ;	assignBit
      000099 D2 90            [12]  442 	setb	_P1_0
                                    443 ;	servo.c:92: delayus();
      00009B 12 00 89         [24]  444 	lcall	_delayus
                                    445 ;	servo.c:93: scl = 1; //scl拉高时 sda突然来个低电平 就启动了IIC总线
                                    446 ;	assignBit
      00009E D2 91            [12]  447 	setb	_P1_1
                                    448 ;	servo.c:94: delayus();
      0000A0 12 00 89         [24]  449 	lcall	_delayus
                                    450 ;	servo.c:95: sda = 0;
                                    451 ;	assignBit
      0000A3 C2 90            [12]  452 	clr	_P1_0
                                    453 ;	servo.c:96: delayus();
      0000A5 12 00 89         [24]  454 	lcall	_delayus
                                    455 ;	servo.c:97: scl = 0;
                                    456 ;	assignBit
      0000A8 C2 91            [12]  457 	clr	_P1_1
                                    458 ;	servo.c:98: delayus();
                                    459 ;	servo.c:99: }
      0000AA 02 00 89         [24]  460 	ljmp	_delayus
                                    461 ;------------------------------------------------------------
                                    462 ;Allocation info for local variables in function 'stop_iic'
                                    463 ;------------------------------------------------------------
                                    464 ;	servo.c:103: void stop_iic()
                                    465 ;	-----------------------------------------
                                    466 ;	 function stop_iic
                                    467 ;	-----------------------------------------
      0000AD                        468 _stop_iic:
                                    469 ;	servo.c:105: sda = 0;
                                    470 ;	assignBit
      0000AD C2 90            [12]  471 	clr	_P1_0
                                    472 ;	servo.c:106: delayus();
      0000AF 12 00 89         [24]  473 	lcall	_delayus
                                    474 ;	servo.c:107: scl = 1; //scl拉高时 sda突然来个高电平 就停止了IIC总线
                                    475 ;	assignBit
      0000B2 D2 91            [12]  476 	setb	_P1_1
                                    477 ;	servo.c:108: delayus();
      0000B4 12 00 89         [24]  478 	lcall	_delayus
                                    479 ;	servo.c:109: sda = 1;
                                    480 ;	assignBit
      0000B7 D2 90            [12]  481 	setb	_P1_0
                                    482 ;	servo.c:110: delayus();
                                    483 ;	servo.c:111: }
      0000B9 02 00 89         [24]  484 	ljmp	_delayus
                                    485 ;------------------------------------------------------------
                                    486 ;Allocation info for local variables in function 'ack_iic'
                                    487 ;------------------------------------------------------------
                                    488 ;i                         Allocated to registers r7 
                                    489 ;------------------------------------------------------------
                                    490 ;	servo.c:115: void ack_iic()
                                    491 ;	-----------------------------------------
                                    492 ;	 function ack_iic
                                    493 ;	-----------------------------------------
      0000BC                        494 _ack_iic:
                                    495 ;	servo.c:118: scl = 1;
                                    496 ;	assignBit
      0000BC D2 91            [12]  497 	setb	_P1_1
                                    498 ;	servo.c:119: delayus();
      0000BE 12 00 89         [24]  499 	lcall	_delayus
                                    500 ;	servo.c:120: while ((sda = 1) && (i < 255))
      0000C1 7F 00            [12]  501 	mov	r7,#0x00
      0000C3                        502 00102$:
                                    503 ;	assignBit
      0000C3 D2 90            [12]  504 	setb	_P1_0
      0000C5 30 90 08         [24]  505 	jnb	_P1_0,00104$
      0000C8 BF FF 00         [24]  506 	cjne	r7,#0xff,00121$
      0000CB                        507 00121$:
      0000CB 50 03            [24]  508 	jnc	00104$
                                    509 ;	servo.c:121: i++;
      0000CD 0F               [12]  510 	inc	r7
      0000CE 80 F3            [24]  511 	sjmp	00102$
      0000D0                        512 00104$:
                                    513 ;	servo.c:122: scl = 0;
                                    514 ;	assignBit
      0000D0 C2 91            [12]  515 	clr	_P1_1
                                    516 ;	servo.c:123: delayus();
                                    517 ;	servo.c:124: }
      0000D2 02 00 89         [24]  518 	ljmp	_delayus
                                    519 ;------------------------------------------------------------
                                    520 ;Allocation info for local variables in function 'write_byte'
                                    521 ;------------------------------------------------------------
                                    522 ;byte                      Allocated to registers 
                                    523 ;i                         Allocated to registers r6 
                                    524 ;temp                      Allocated to registers r7 
                                    525 ;------------------------------------------------------------
                                    526 ;	servo.c:128: void write_byte(uchar byte)
                                    527 ;	-----------------------------------------
                                    528 ;	 function write_byte
                                    529 ;	-----------------------------------------
      0000D5                        530 _write_byte:
      0000D5 AF 82            [24]  531 	mov	r7,dpl
                                    532 ;	servo.c:132: for (i = 0; i < 8; i++)
      0000D7 7E 00            [12]  533 	mov	r6,#0x00
      0000D9                        534 00102$:
                                    535 ;	servo.c:134: temp = temp << 1;
      0000D9 8F 05            [24]  536 	mov	ar5,r7
      0000DB ED               [12]  537 	mov	a,r5
      0000DC 2D               [12]  538 	add	a,r5
      0000DD FF               [12]  539 	mov	r7,a
                                    540 ;	servo.c:135: scl = 0;
                                    541 ;	assignBit
      0000DE C2 91            [12]  542 	clr	_P1_1
                                    543 ;	servo.c:136: delayus();
      0000E0 C0 07            [24]  544 	push	ar7
      0000E2 C0 06            [24]  545 	push	ar6
      0000E4 12 00 89         [24]  546 	lcall	_delayus
                                    547 ;	servo.c:137: sda = CY;
                                    548 ;	assignBit
      0000E7 A2 D7            [12]  549 	mov	c,_CY
      0000E9 92 90            [24]  550 	mov	_P1_0,c
                                    551 ;	servo.c:138: delayus();
      0000EB 12 00 89         [24]  552 	lcall	_delayus
                                    553 ;	servo.c:139: scl = 1;
                                    554 ;	assignBit
      0000EE D2 91            [12]  555 	setb	_P1_1
                                    556 ;	servo.c:140: delayus();
      0000F0 12 00 89         [24]  557 	lcall	_delayus
      0000F3 D0 06            [24]  558 	pop	ar6
      0000F5 D0 07            [24]  559 	pop	ar7
                                    560 ;	servo.c:132: for (i = 0; i < 8; i++)
      0000F7 0E               [12]  561 	inc	r6
      0000F8 BE 08 00         [24]  562 	cjne	r6,#0x08,00115$
      0000FB                        563 00115$:
      0000FB 40 DC            [24]  564 	jc	00102$
                                    565 ;	servo.c:142: scl = 0;
                                    566 ;	assignBit
      0000FD C2 91            [12]  567 	clr	_P1_1
                                    568 ;	servo.c:143: delayus();
      0000FF 12 00 89         [24]  569 	lcall	_delayus
                                    570 ;	servo.c:144: sda = 1;
                                    571 ;	assignBit
      000102 D2 90            [12]  572 	setb	_P1_0
                                    573 ;	servo.c:145: delayus();
                                    574 ;	servo.c:146: }
      000104 02 00 89         [24]  575 	ljmp	_delayus
                                    576 ;------------------------------------------------------------
                                    577 ;Allocation info for local variables in function 'read_byte'
                                    578 ;------------------------------------------------------------
                                    579 ;k                         Allocated to registers r7 
                                    580 ;i                         Allocated to registers r6 
                                    581 ;j                         Allocated to registers r5 
                                    582 ;------------------------------------------------------------
                                    583 ;	servo.c:150: uchar read_byte()
                                    584 ;	-----------------------------------------
                                    585 ;	 function read_byte
                                    586 ;	-----------------------------------------
      000107                        587 _read_byte:
                                    588 ;	servo.c:152: uchar k = 0;
      000107 7F 00            [12]  589 	mov	r7,#0x00
                                    590 ;	servo.c:153: scl = 0;
                                    591 ;	assignBit
      000109 C2 91            [12]  592 	clr	_P1_1
                                    593 ;	servo.c:154: delayus();
      00010B C0 07            [24]  594 	push	ar7
      00010D 12 00 89         [24]  595 	lcall	_delayus
                                    596 ;	servo.c:155: sda = 1;
                                    597 ;	assignBit
      000110 D2 90            [12]  598 	setb	_P1_0
                                    599 ;	servo.c:156: delayus();
      000112 12 00 89         [24]  600 	lcall	_delayus
      000115 D0 07            [24]  601 	pop	ar7
                                    602 ;	servo.c:157: for (uchar i = 0; i < 8; i++)
      000117 7E 00            [12]  603 	mov	r6,#0x00
      000119                        604 00106$:
      000119 BE 08 00         [24]  605 	cjne	r6,#0x08,00123$
      00011C                        606 00123$:
      00011C 50 25            [24]  607 	jnc	00104$
                                    608 ;	servo.c:160: delayus();
      00011E C0 07            [24]  609 	push	ar7
      000120 C0 06            [24]  610 	push	ar6
      000122 12 00 89         [24]  611 	lcall	_delayus
                                    612 ;	servo.c:161: scl = 1;
                                    613 ;	assignBit
      000125 D2 91            [12]  614 	setb	_P1_1
                                    615 ;	servo.c:162: delayus();
      000127 12 00 89         [24]  616 	lcall	_delayus
      00012A D0 06            [24]  617 	pop	ar6
      00012C D0 07            [24]  618 	pop	ar7
                                    619 ;	servo.c:163: if (sda == 1)
      00012E 30 90 04         [24]  620 	jnb	_P1_0,00102$
                                    621 ;	servo.c:165: j = 1;
      000131 7D 01            [12]  622 	mov	r5,#0x01
      000133 80 02            [24]  623 	sjmp	00103$
      000135                        624 00102$:
                                    625 ;	servo.c:168: j = 0;
      000135 7D 00            [12]  626 	mov	r5,#0x00
      000137                        627 00103$:
                                    628 ;	servo.c:169: k = (k << 1) | j;
      000137 8F 04            [24]  629 	mov	ar4,r7
      000139 EC               [12]  630 	mov	a,r4
      00013A 2C               [12]  631 	add	a,r4
      00013B FC               [12]  632 	mov	r4,a
      00013C 4D               [12]  633 	orl	a,r5
      00013D FF               [12]  634 	mov	r7,a
                                    635 ;	servo.c:170: scl = 0;
                                    636 ;	assignBit
      00013E C2 91            [12]  637 	clr	_P1_1
                                    638 ;	servo.c:157: for (uchar i = 0; i < 8; i++)
      000140 0E               [12]  639 	inc	r6
      000141 80 D6            [24]  640 	sjmp	00106$
      000143                        641 00104$:
                                    642 ;	servo.c:172: delayus();
      000143 C0 07            [24]  643 	push	ar7
      000145 12 00 89         [24]  644 	lcall	_delayus
      000148 D0 07            [24]  645 	pop	ar7
                                    646 ;	servo.c:173: return k;
      00014A 8F 82            [24]  647 	mov	dpl,r7
                                    648 ;	servo.c:174: }
      00014C 22               [24]  649 	ret
                                    650 ;------------------------------------------------------------
                                    651 ;Allocation info for local variables in function 'pca9685_write'
                                    652 ;------------------------------------------------------------
                                    653 ;date                      Allocated with name '_pca9685_write_PARM_2'
                                    654 ;address                   Allocated to registers r7 
                                    655 ;------------------------------------------------------------
                                    656 ;	servo.c:181: void pca9685_write(uchar address, uchar date)
                                    657 ;	-----------------------------------------
                                    658 ;	 function pca9685_write
                                    659 ;	-----------------------------------------
      00014D                        660 _pca9685_write:
      00014D AF 82            [24]  661 	mov	r7,dpl
                                    662 ;	servo.c:183: start_iic();
      00014F C0 07            [24]  663 	push	ar7
      000151 12 00 99         [24]  664 	lcall	_start_iic
                                    665 ;	servo.c:184: write_byte(PCA9685_adrr); //PCA9685的片选地址
      000154 75 82 80         [24]  666 	mov	dpl,#0x80
      000157 12 00 D5         [24]  667 	lcall	_write_byte
                                    668 ;	servo.c:185: ack_iic();
      00015A 12 00 BC         [24]  669 	lcall	_ack_iic
      00015D D0 07            [24]  670 	pop	ar7
                                    671 ;	servo.c:186: write_byte(address); //写地址控制字节
      00015F 8F 82            [24]  672 	mov	dpl,r7
      000161 12 00 D5         [24]  673 	lcall	_write_byte
                                    674 ;	servo.c:187: ack_iic();
      000164 12 00 BC         [24]  675 	lcall	_ack_iic
                                    676 ;	servo.c:188: write_byte(date); //写数据
      000167 85 08 82         [24]  677 	mov	dpl,_pca9685_write_PARM_2
      00016A 12 00 D5         [24]  678 	lcall	_write_byte
                                    679 ;	servo.c:189: ack_iic();
      00016D 12 00 BC         [24]  680 	lcall	_ack_iic
                                    681 ;	servo.c:190: stop_iic();
                                    682 ;	servo.c:191: }
      000170 02 00 AD         [24]  683 	ljmp	_stop_iic
                                    684 ;------------------------------------------------------------
                                    685 ;Allocation info for local variables in function 'pca9685_read'
                                    686 ;------------------------------------------------------------
                                    687 ;address                   Allocated to registers r7 
                                    688 ;date                      Allocated to registers r7 
                                    689 ;------------------------------------------------------------
                                    690 ;	servo.c:195: uchar pca9685_read(uchar address)
                                    691 ;	-----------------------------------------
                                    692 ;	 function pca9685_read
                                    693 ;	-----------------------------------------
      000173                        694 _pca9685_read:
      000173 AF 82            [24]  695 	mov	r7,dpl
                                    696 ;	servo.c:198: start_iic();
      000175 C0 07            [24]  697 	push	ar7
      000177 12 00 99         [24]  698 	lcall	_start_iic
                                    699 ;	servo.c:199: write_byte(PCA9685_adrr); //PCA9685的片选地址
      00017A 75 82 80         [24]  700 	mov	dpl,#0x80
      00017D 12 00 D5         [24]  701 	lcall	_write_byte
                                    702 ;	servo.c:200: ack_iic();
      000180 12 00 BC         [24]  703 	lcall	_ack_iic
      000183 D0 07            [24]  704 	pop	ar7
                                    705 ;	servo.c:201: write_byte(address);
      000185 8F 82            [24]  706 	mov	dpl,r7
      000187 12 00 D5         [24]  707 	lcall	_write_byte
                                    708 ;	servo.c:202: ack_iic();
      00018A 12 00 BC         [24]  709 	lcall	_ack_iic
                                    710 ;	servo.c:203: start_iic();
      00018D 12 00 99         [24]  711 	lcall	_start_iic
                                    712 ;	servo.c:204: write_byte(PCA9685_adrr | 0x01); //地址的第八位控制数据流方向，就是写或读
      000190 75 82 81         [24]  713 	mov	dpl,#0x81
      000193 12 00 D5         [24]  714 	lcall	_write_byte
                                    715 ;	servo.c:205: ack_iic();
      000196 12 00 BC         [24]  716 	lcall	_ack_iic
                                    717 ;	servo.c:206: date = read_byte();
      000199 12 01 07         [24]  718 	lcall	_read_byte
      00019C AF 82            [24]  719 	mov	r7,dpl
                                    720 ;	servo.c:207: stop_iic();
      00019E C0 07            [24]  721 	push	ar7
      0001A0 12 00 AD         [24]  722 	lcall	_stop_iic
      0001A3 D0 07            [24]  723 	pop	ar7
                                    724 ;	servo.c:208: return date;
      0001A5 8F 82            [24]  725 	mov	dpl,r7
                                    726 ;	servo.c:209: }
      0001A7 22               [24]  727 	ret
                                    728 ;------------------------------------------------------------
                                    729 ;Allocation info for local variables in function 'reset_pca9685'
                                    730 ;------------------------------------------------------------
                                    731 ;	servo.c:213: void reset_pca9685(void)
                                    732 ;	-----------------------------------------
                                    733 ;	 function reset_pca9685
                                    734 ;	-----------------------------------------
      0001A8                        735 _reset_pca9685:
                                    736 ;	servo.c:215: pca9685_write(PCA9685_MODE1, 0x0);
      0001A8 75 08 00         [24]  737 	mov	_pca9685_write_PARM_2,#0x00
      0001AB 75 82 00         [24]  738 	mov	dpl,#0x00
                                    739 ;	servo.c:216: }
      0001AE 02 01 4D         [24]  740 	ljmp	_pca9685_write
                                    741 ;------------------------------------------------------------
                                    742 ;Allocation info for local variables in function 'set_pwm_freq'
                                    743 ;------------------------------------------------------------
                                    744 ;freq                      Allocated to registers r4 r5 r6 r7 
                                    745 ;prescale                  Allocated to registers r6 r7 
                                    746 ;oldmode                   Allocated to registers r5 r4 
                                    747 ;newmode                   Allocated to registers r2 r3 
                                    748 ;prescaleval               Allocated to registers r4 r5 r6 r7 
                                    749 ;------------------------------------------------------------
                                    750 ;	servo.c:221: void set_pwm_freq(float freq)
                                    751 ;	-----------------------------------------
                                    752 ;	 function set_pwm_freq
                                    753 ;	-----------------------------------------
      0001B1                        754 _set_pwm_freq:
      0001B1 AC 82            [24]  755 	mov	r4,dpl
      0001B3 AD 83            [24]  756 	mov	r5,dph
      0001B5 AE F0            [24]  757 	mov	r6,b
      0001B7 FF               [12]  758 	mov	r7,a
                                    759 ;	servo.c:225: freq *= 0.92; // Correct for overshoot in the frequency setting
      0001B8 C0 04            [24]  760 	push	ar4
      0001BA C0 05            [24]  761 	push	ar5
      0001BC C0 06            [24]  762 	push	ar6
      0001BE C0 07            [24]  763 	push	ar7
      0001C0 90 85 1F         [24]  764 	mov	dptr,#0x851f
      0001C3 75 F0 6B         [24]  765 	mov	b,#0x6b
      0001C6 74 3F            [12]  766 	mov	a,#0x3f
      0001C8 12 04 7C         [24]  767 	lcall	___fsmul
      0001CB AC 82            [24]  768 	mov	r4,dpl
      0001CD AD 83            [24]  769 	mov	r5,dph
      0001CF AE F0            [24]  770 	mov	r6,b
      0001D1 FF               [12]  771 	mov	r7,a
      0001D2 E5 81            [12]  772 	mov	a,sp
      0001D4 24 FC            [12]  773 	add	a,#0xfc
      0001D6 F5 81            [12]  774 	mov	sp,a
                                    775 ;	servo.c:228: prescaleval /= freq;
      0001D8 C0 04            [24]  776 	push	ar4
      0001DA C0 05            [24]  777 	push	ar5
      0001DC C0 06            [24]  778 	push	ar6
      0001DE C0 07            [24]  779 	push	ar7
      0001E0 90 BC 20         [24]  780 	mov	dptr,#0xbc20
      0001E3 75 F0 BE         [24]  781 	mov	b,#0xbe
      0001E6 74 45            [12]  782 	mov	a,#0x45
      0001E8 12 06 4D         [24]  783 	lcall	___fsdiv
      0001EB AC 82            [24]  784 	mov	r4,dpl
      0001ED AD 83            [24]  785 	mov	r5,dph
      0001EF AE F0            [24]  786 	mov	r6,b
      0001F1 FF               [12]  787 	mov	r7,a
      0001F2 E5 81            [12]  788 	mov	a,sp
      0001F4 24 FC            [12]  789 	add	a,#0xfc
      0001F6 F5 81            [12]  790 	mov	sp,a
                                    791 ;	servo.c:229: prescaleval -= 1;
      0001F8 E4               [12]  792 	clr	a
      0001F9 C0 E0            [24]  793 	push	acc
      0001FB C0 E0            [24]  794 	push	acc
      0001FD 74 80            [12]  795 	mov	a,#0x80
      0001FF C0 E0            [24]  796 	push	acc
      000201 74 3F            [12]  797 	mov	a,#0x3f
      000203 C0 E0            [24]  798 	push	acc
      000205 8C 82            [24]  799 	mov	dpl,r4
      000207 8D 83            [24]  800 	mov	dph,r5
      000209 8E F0            [24]  801 	mov	b,r6
      00020B EF               [12]  802 	mov	a,r7
      00020C 12 04 71         [24]  803 	lcall	___fssub
      00020F AC 82            [24]  804 	mov	r4,dpl
      000211 AD 83            [24]  805 	mov	r5,dph
      000213 AE F0            [24]  806 	mov	r6,b
      000215 FF               [12]  807 	mov	r7,a
      000216 E5 81            [12]  808 	mov	a,sp
      000218 24 FC            [12]  809 	add	a,#0xfc
      00021A F5 81            [12]  810 	mov	sp,a
                                    811 ;	servo.c:230: prescale = (uint)(prescaleval + 0.5);
      00021C E4               [12]  812 	clr	a
      00021D C0 E0            [24]  813 	push	acc
      00021F C0 E0            [24]  814 	push	acc
      000221 C0 E0            [24]  815 	push	acc
      000223 74 3F            [12]  816 	mov	a,#0x3f
      000225 C0 E0            [24]  817 	push	acc
      000227 8C 82            [24]  818 	mov	dpl,r4
      000229 8D 83            [24]  819 	mov	dph,r5
      00022B 8E F0            [24]  820 	mov	b,r6
      00022D EF               [12]  821 	mov	a,r7
      00022E 12 05 80         [24]  822 	lcall	___fsadd
      000231 AC 82            [24]  823 	mov	r4,dpl
      000233 AD 83            [24]  824 	mov	r5,dph
      000235 AE F0            [24]  825 	mov	r6,b
      000237 FF               [12]  826 	mov	r7,a
      000238 E5 81            [12]  827 	mov	a,sp
      00023A 24 FC            [12]  828 	add	a,#0xfc
      00023C F5 81            [12]  829 	mov	sp,a
      00023E 8C 82            [24]  830 	mov	dpl,r4
      000240 8D 83            [24]  831 	mov	dph,r5
      000242 8E F0            [24]  832 	mov	b,r6
      000244 EF               [12]  833 	mov	a,r7
      000245 12 05 D9         [24]  834 	lcall	___fs2uint
      000248 AE 82            [24]  835 	mov	r6,dpl
      00024A AF 83            [24]  836 	mov	r7,dph
                                    837 ;	servo.c:232: oldmode = pca9685_read(PCA9685_MODE1);
      00024C 75 82 00         [24]  838 	mov	dpl,#0x00
      00024F C0 07            [24]  839 	push	ar7
      000251 C0 06            [24]  840 	push	ar6
      000253 12 01 73         [24]  841 	lcall	_pca9685_read
      000256 AD 82            [24]  842 	mov	r5,dpl
      000258 7C 00            [12]  843 	mov	r4,#0x00
                                    844 ;	servo.c:233: newmode = (oldmode & 0x7F) | 0x10;         // sleep
      00025A 74 7F            [12]  845 	mov	a,#0x7f
      00025C 5D               [12]  846 	anl	a,r5
      00025D FA               [12]  847 	mov	r2,a
      00025E 43 02 10         [24]  848 	orl	ar2,#0x10
                                    849 ;	servo.c:234: pca9685_write(PCA9685_MODE1, newmode);     // go to sleep
      000261 8A 08            [24]  850 	mov	_pca9685_write_PARM_2,r2
      000263 75 82 00         [24]  851 	mov	dpl,#0x00
      000266 C0 05            [24]  852 	push	ar5
      000268 C0 04            [24]  853 	push	ar4
      00026A 12 01 4D         [24]  854 	lcall	_pca9685_write
      00026D D0 04            [24]  855 	pop	ar4
      00026F D0 05            [24]  856 	pop	ar5
      000271 D0 06            [24]  857 	pop	ar6
      000273 D0 07            [24]  858 	pop	ar7
                                    859 ;	servo.c:235: pca9685_write(PCA9685_PRESCALE, prescale); // set the prescaler
      000275 8E 08            [24]  860 	mov	_pca9685_write_PARM_2,r6
      000277 75 82 FE         [24]  861 	mov	dpl,#0xfe
      00027A C0 05            [24]  862 	push	ar5
      00027C C0 04            [24]  863 	push	ar4
      00027E 12 01 4D         [24]  864 	lcall	_pca9685_write
      000281 D0 04            [24]  865 	pop	ar4
      000283 D0 05            [24]  866 	pop	ar5
                                    867 ;	servo.c:236: pca9685_write(PCA9685_MODE1, oldmode);
      000285 8D 08            [24]  868 	mov	_pca9685_write_PARM_2,r5
      000287 75 82 00         [24]  869 	mov	dpl,#0x00
      00028A C0 05            [24]  870 	push	ar5
      00028C C0 04            [24]  871 	push	ar4
      00028E 12 01 4D         [24]  872 	lcall	_pca9685_write
                                    873 ;	servo.c:237: delayms(2);
      000291 90 00 02         [24]  874 	mov	dptr,#0x0002
      000294 12 00 65         [24]  875 	lcall	_delayms
      000297 D0 04            [24]  876 	pop	ar4
      000299 D0 05            [24]  877 	pop	ar5
                                    878 ;	servo.c:238: pca9685_write(PCA9685_MODE1, oldmode | 0xa1);
      00029B 43 05 A1         [24]  879 	orl	ar5,#0xa1
      00029E 8D 08            [24]  880 	mov	_pca9685_write_PARM_2,r5
      0002A0 75 82 00         [24]  881 	mov	dpl,#0x00
                                    882 ;	servo.c:239: }
      0002A3 02 01 4D         [24]  883 	ljmp	_pca9685_write
                                    884 ;------------------------------------------------------------
                                    885 ;Allocation info for local variables in function 'servo_control'
                                    886 ;------------------------------------------------------------
                                    887 ;angle                     Allocated with name '_servo_control_PARM_2'
                                    888 ;num                       Allocated to registers r7 
                                    889 ;off                       Allocated to registers r5 r6 
                                    890 ;------------------------------------------------------------
                                    891 ;	servo.c:247: void servo_control(uchar num, uchar angle)
                                    892 ;	-----------------------------------------
                                    893 ;	 function servo_control
                                    894 ;	-----------------------------------------
      0002A6                        895 _servo_control:
      0002A6 AF 82            [24]  896 	mov	r7,dpl
                                    897 ;	servo.c:249: uint off = 102.4 + 2.275555556 * angle;
      0002A8 85 09 82         [24]  898 	mov	dpl,_servo_control_PARM_2
      0002AB C0 07            [24]  899 	push	ar7
      0002AD 12 07 10         [24]  900 	lcall	___uchar2fs
      0002B0 AB 82            [24]  901 	mov	r3,dpl
      0002B2 AC 83            [24]  902 	mov	r4,dph
      0002B4 AD F0            [24]  903 	mov	r5,b
      0002B6 FE               [12]  904 	mov	r6,a
      0002B7 C0 03            [24]  905 	push	ar3
      0002B9 C0 04            [24]  906 	push	ar4
      0002BB C0 05            [24]  907 	push	ar5
      0002BD C0 06            [24]  908 	push	ar6
      0002BF 90 A2 B4         [24]  909 	mov	dptr,#0xa2b4
      0002C2 75 F0 11         [24]  910 	mov	b,#0x11
      0002C5 74 40            [12]  911 	mov	a,#0x40
      0002C7 12 04 7C         [24]  912 	lcall	___fsmul
      0002CA AB 82            [24]  913 	mov	r3,dpl
      0002CC AC 83            [24]  914 	mov	r4,dph
      0002CE AD F0            [24]  915 	mov	r5,b
      0002D0 FE               [12]  916 	mov	r6,a
      0002D1 E5 81            [12]  917 	mov	a,sp
      0002D3 24 FC            [12]  918 	add	a,#0xfc
      0002D5 F5 81            [12]  919 	mov	sp,a
      0002D7 74 CD            [12]  920 	mov	a,#0xcd
      0002D9 C0 E0            [24]  921 	push	acc
      0002DB 14               [12]  922 	dec	a
      0002DC C0 E0            [24]  923 	push	acc
      0002DE C0 E0            [24]  924 	push	acc
      0002E0 74 42            [12]  925 	mov	a,#0x42
      0002E2 C0 E0            [24]  926 	push	acc
      0002E4 8B 82            [24]  927 	mov	dpl,r3
      0002E6 8C 83            [24]  928 	mov	dph,r4
      0002E8 8D F0            [24]  929 	mov	b,r5
      0002EA EE               [12]  930 	mov	a,r6
      0002EB 12 05 80         [24]  931 	lcall	___fsadd
      0002EE AB 82            [24]  932 	mov	r3,dpl
      0002F0 AC 83            [24]  933 	mov	r4,dph
      0002F2 AD F0            [24]  934 	mov	r5,b
      0002F4 FE               [12]  935 	mov	r6,a
      0002F5 E5 81            [12]  936 	mov	a,sp
      0002F7 24 FC            [12]  937 	add	a,#0xfc
      0002F9 F5 81            [12]  938 	mov	sp,a
      0002FB 8B 82            [24]  939 	mov	dpl,r3
      0002FD 8C 83            [24]  940 	mov	dph,r4
      0002FF 8D F0            [24]  941 	mov	b,r5
      000301 EE               [12]  942 	mov	a,r6
      000302 12 05 D9         [24]  943 	lcall	___fs2uint
      000305 AD 82            [24]  944 	mov	r5,dpl
      000307 AE 83            [24]  945 	mov	r6,dph
      000309 D0 07            [24]  946 	pop	ar7
                                    947 ;	servo.c:250: pca9685_write(LED0_ON_L + 4 * num, 0);
      00030B EF               [12]  948 	mov	a,r7
      00030C 2F               [12]  949 	add	a,r7
      00030D 25 E0            [12]  950 	add	a,acc
      00030F FF               [12]  951 	mov	r7,a
      000310 24 06            [12]  952 	add	a,#0x06
      000312 F5 82            [12]  953 	mov	dpl,a
      000314 75 08 00         [24]  954 	mov	_pca9685_write_PARM_2,#0x00
      000317 C0 07            [24]  955 	push	ar7
      000319 C0 06            [24]  956 	push	ar6
      00031B C0 05            [24]  957 	push	ar5
      00031D 12 01 4D         [24]  958 	lcall	_pca9685_write
      000320 D0 05            [24]  959 	pop	ar5
      000322 D0 06            [24]  960 	pop	ar6
      000324 D0 07            [24]  961 	pop	ar7
                                    962 ;	servo.c:251: pca9685_write(LED0_ON_H + 4 * num, 0);
      000326 74 07            [12]  963 	mov	a,#0x07
      000328 2F               [12]  964 	add	a,r7
      000329 F5 82            [12]  965 	mov	dpl,a
      00032B 75 08 00         [24]  966 	mov	_pca9685_write_PARM_2,#0x00
      00032E C0 07            [24]  967 	push	ar7
      000330 C0 06            [24]  968 	push	ar6
      000332 C0 05            [24]  969 	push	ar5
      000334 12 01 4D         [24]  970 	lcall	_pca9685_write
      000337 D0 05            [24]  971 	pop	ar5
      000339 D0 06            [24]  972 	pop	ar6
      00033B D0 07            [24]  973 	pop	ar7
                                    974 ;	servo.c:252: pca9685_write(LED0_OFF_L + 4 * num, off);
      00033D 74 08            [12]  975 	mov	a,#0x08
      00033F 2F               [12]  976 	add	a,r7
      000340 F5 82            [12]  977 	mov	dpl,a
      000342 8D 08            [24]  978 	mov	_pca9685_write_PARM_2,r5
      000344 C0 07            [24]  979 	push	ar7
      000346 C0 06            [24]  980 	push	ar6
      000348 C0 05            [24]  981 	push	ar5
      00034A 12 01 4D         [24]  982 	lcall	_pca9685_write
      00034D D0 05            [24]  983 	pop	ar5
      00034F D0 06            [24]  984 	pop	ar6
      000351 D0 07            [24]  985 	pop	ar7
                                    986 ;	servo.c:253: pca9685_write(LED0_OFF_H + 4 * num, off >> 8);
      000353 74 09            [12]  987 	mov	a,#0x09
      000355 2F               [12]  988 	add	a,r7
      000356 F5 82            [12]  989 	mov	dpl,a
      000358 8E 08            [24]  990 	mov	_pca9685_write_PARM_2,r6
                                    991 ;	servo.c:254: }
      00035A 02 01 4D         [24]  992 	ljmp	_pca9685_write
                                    993 ;------------------------------------------------------------
                                    994 ;Allocation info for local variables in function 'init_pca9685'
                                    995 ;------------------------------------------------------------
                                    996 ;	servo.c:256: void init_pca9685()
                                    997 ;	-----------------------------------------
                                    998 ;	 function init_pca9685
                                    999 ;	-----------------------------------------
      00035D                       1000 _init_pca9685:
                                   1001 ;	servo.c:258: init_iic();
      00035D 12 00 8F         [24] 1002 	lcall	_init_iic
                                   1003 ;	servo.c:259: reset_pca9685();
      000360 12 01 A8         [24] 1004 	lcall	_reset_pca9685
                                   1005 ;	servo.c:260: set_pwm_freq(50);
      000363 90 00 00         [24] 1006 	mov	dptr,#0x0000
      000366 75 F0 48         [24] 1007 	mov	b,#0x48
      000369 74 42            [12] 1008 	mov	a,#0x42
                                   1009 ;	servo.c:261: }
      00036B 02 01 B1         [24] 1010 	ljmp	_set_pwm_freq
                                   1011 ;------------------------------------------------------------
                                   1012 ;Allocation info for local variables in function 'main'
                                   1013 ;------------------------------------------------------------
                                   1014 ;i                         Allocated to registers r6 r7 
                                   1015 ;------------------------------------------------------------
                                   1016 ;	servo.c:266: void main()
                                   1017 ;	-----------------------------------------
                                   1018 ;	 function main
                                   1019 ;	-----------------------------------------
      00036E                       1020 _main:
                                   1021 ;	servo.c:268: init_pca9685();
      00036E 12 03 5D         [24] 1022 	lcall	_init_pca9685
                                   1023 ;	servo.c:273: while (1)
      000371                       1024 00111$:
                                   1025 ;	servo.c:275: delayms(10);
      000371 90 00 0A         [24] 1026 	mov	dptr,#0x000a
      000374 12 00 65         [24] 1027 	lcall	_delayms
                                   1028 ;	servo.c:276: if (P3_2 == 0) {
      000377 20 B2 2E         [24] 1029 	jb	_P3_2,00103$
                                   1030 ;	servo.c:277: P0_0 = !P0_0;
      00037A B2 80            [12] 1031 	cpl	_P0_0
                                   1032 ;	servo.c:278: for (int i = 0; i < 180; i++)
      00037C 7E 00            [12] 1033 	mov	r6,#0x00
      00037E 7F 00            [12] 1034 	mov	r7,#0x00
      000380                       1035 00114$:
      000380 C3               [12] 1036 	clr	c
      000381 EE               [12] 1037 	mov	a,r6
      000382 94 B4            [12] 1038 	subb	a,#0xb4
      000384 EF               [12] 1039 	mov	a,r7
      000385 64 80            [12] 1040 	xrl	a,#0x80
      000387 94 80            [12] 1041 	subb	a,#0x80
      000389 50 1D            [24] 1042 	jnc	00103$
                                   1043 ;	servo.c:280: servo_control(1, i);
      00038B 8E 09            [24] 1044 	mov	_servo_control_PARM_2,r6
      00038D 75 82 01         [24] 1045 	mov	dpl,#0x01
      000390 C0 07            [24] 1046 	push	ar7
      000392 C0 06            [24] 1047 	push	ar6
      000394 12 02 A6         [24] 1048 	lcall	_servo_control
                                   1049 ;	servo.c:281: delayms(10);
      000397 90 00 0A         [24] 1050 	mov	dptr,#0x000a
      00039A 12 00 65         [24] 1051 	lcall	_delayms
      00039D D0 06            [24] 1052 	pop	ar6
      00039F D0 07            [24] 1053 	pop	ar7
                                   1054 ;	servo.c:278: for (int i = 0; i < 180; i++)
      0003A1 0E               [12] 1055 	inc	r6
      0003A2 BE 00 DB         [24] 1056 	cjne	r6,#0x00,00114$
      0003A5 0F               [12] 1057 	inc	r7
      0003A6 80 D8            [24] 1058 	sjmp	00114$
      0003A8                       1059 00103$:
                                   1060 ;	servo.c:284: if (P3_3 == 0) {
      0003A8 20 B3 0B         [24] 1061 	jb	_P3_3,00105$
                                   1062 ;	servo.c:285: P0_1 = !P0_1;
      0003AB B2 81            [12] 1063 	cpl	_P0_1
                                   1064 ;	servo.c:286: servo_control(1, 0);//right
      0003AD 75 09 00         [24] 1065 	mov	_servo_control_PARM_2,#0x00
      0003B0 75 82 01         [24] 1066 	mov	dpl,#0x01
      0003B3 12 02 A6         [24] 1067 	lcall	_servo_control
      0003B6                       1068 00105$:
                                   1069 ;	servo.c:288: if (P3_4 == 0) {
      0003B6 20 B4 0B         [24] 1070 	jb	_P3_4,00107$
                                   1071 ;	servo.c:289: P0_2 = !P0_2;
      0003B9 B2 82            [12] 1072 	cpl	_P0_2
                                   1073 ;	servo.c:290: servo_control(1, 90);//right
      0003BB 75 09 5A         [24] 1074 	mov	_servo_control_PARM_2,#0x5a
      0003BE 75 82 01         [24] 1075 	mov	dpl,#0x01
      0003C1 12 02 A6         [24] 1076 	lcall	_servo_control
      0003C4                       1077 00107$:
                                   1078 ;	servo.c:292: if (P3_5 == 0) {
      0003C4 20 B5 AA         [24] 1079 	jb	_P3_5,00111$
                                   1080 ;	servo.c:293: P0_3 = !P0_3;
      0003C7 B2 83            [12] 1081 	cpl	_P0_3
                                   1082 ;	servo.c:294: servo_control(1, 176);//right
      0003C9 75 09 B0         [24] 1083 	mov	_servo_control_PARM_2,#0xb0
      0003CC 75 82 01         [24] 1084 	mov	dpl,#0x01
      0003CF 12 02 A6         [24] 1085 	lcall	_servo_control
                                   1086 ;	servo.c:320: }
      0003D2 80 9D            [24] 1087 	sjmp	00111$
                                   1088 ;------------------------------------------------------------
                                   1089 ;Allocation info for local variables in function 'Key_INT_0'
                                   1090 ;------------------------------------------------------------
                                   1091 ;i                         Allocated to registers r6 r7 
                                   1092 ;------------------------------------------------------------
                                   1093 ;	servo.c:322: void Key_INT_0(void) __interrupt 0 //R0 R1 =  0/1   0/1
                                   1094 ;	-----------------------------------------
                                   1095 ;	 function Key_INT_0
                                   1096 ;	-----------------------------------------
      0003D4                       1097 _Key_INT_0:
      0003D4 C0 20            [24] 1098 	push	bits
      0003D6 C0 E0            [24] 1099 	push	acc
      0003D8 C0 F0            [24] 1100 	push	b
      0003DA C0 82            [24] 1101 	push	dpl
      0003DC C0 83            [24] 1102 	push	dph
      0003DE C0 07            [24] 1103 	push	(0+7)
      0003E0 C0 06            [24] 1104 	push	(0+6)
      0003E2 C0 05            [24] 1105 	push	(0+5)
      0003E4 C0 04            [24] 1106 	push	(0+4)
      0003E6 C0 03            [24] 1107 	push	(0+3)
      0003E8 C0 02            [24] 1108 	push	(0+2)
      0003EA C0 01            [24] 1109 	push	(0+1)
      0003EC C0 00            [24] 1110 	push	(0+0)
      0003EE C0 D0            [24] 1111 	push	psw
      0003F0 75 D0 00         [24] 1112 	mov	psw,#0x00
                                   1113 ;	servo.c:324: delayms(10);
      0003F3 90 00 0A         [24] 1114 	mov	dptr,#0x000a
      0003F6 12 00 65         [24] 1115 	lcall	_delayms
                                   1116 ;	servo.c:325: if (P3_2 == 0) {
      0003F9 20 B2 2E         [24] 1117 	jb	_P3_2,00103$
                                   1118 ;	servo.c:326: P0_0 = !P0_0;
      0003FC B2 80            [12] 1119 	cpl	_P0_0
                                   1120 ;	servo.c:327: for (int i = 0; i < 180; i++)
      0003FE 7E 00            [12] 1121 	mov	r6,#0x00
      000400 7F 00            [12] 1122 	mov	r7,#0x00
      000402                       1123 00111$:
      000402 C3               [12] 1124 	clr	c
      000403 EE               [12] 1125 	mov	a,r6
      000404 94 B4            [12] 1126 	subb	a,#0xb4
      000406 EF               [12] 1127 	mov	a,r7
      000407 64 80            [12] 1128 	xrl	a,#0x80
      000409 94 80            [12] 1129 	subb	a,#0x80
      00040B 50 1D            [24] 1130 	jnc	00103$
                                   1131 ;	servo.c:329: servo_control(1, i);
      00040D 8E 09            [24] 1132 	mov	_servo_control_PARM_2,r6
      00040F 75 82 01         [24] 1133 	mov	dpl,#0x01
      000412 C0 07            [24] 1134 	push	ar7
      000414 C0 06            [24] 1135 	push	ar6
      000416 12 02 A6         [24] 1136 	lcall	_servo_control
                                   1137 ;	servo.c:330: delayms(10);
      000419 90 00 0A         [24] 1138 	mov	dptr,#0x000a
      00041C 12 00 65         [24] 1139 	lcall	_delayms
      00041F D0 06            [24] 1140 	pop	ar6
      000421 D0 07            [24] 1141 	pop	ar7
                                   1142 ;	servo.c:327: for (int i = 0; i < 180; i++)
      000423 0E               [12] 1143 	inc	r6
      000424 BE 00 DB         [24] 1144 	cjne	r6,#0x00,00111$
      000427 0F               [12] 1145 	inc	r7
      000428 80 D8            [24] 1146 	sjmp	00111$
      00042A                       1147 00103$:
                                   1148 ;	servo.c:333: if (P3_3 == 0) {
      00042A 20 B3 0B         [24] 1149 	jb	_P3_3,00105$
                                   1150 ;	servo.c:334: P0_1 = !P0_1;
      00042D B2 81            [12] 1151 	cpl	_P0_1
                                   1152 ;	servo.c:335: servo_control(1, 0);//right
      00042F 75 09 00         [24] 1153 	mov	_servo_control_PARM_2,#0x00
      000432 75 82 01         [24] 1154 	mov	dpl,#0x01
      000435 12 02 A6         [24] 1155 	lcall	_servo_control
      000438                       1156 00105$:
                                   1157 ;	servo.c:337: if (P3_4 == 0) {
      000438 20 B4 0B         [24] 1158 	jb	_P3_4,00107$
                                   1159 ;	servo.c:338: P0_2 = !P0_2;
      00043B B2 82            [12] 1160 	cpl	_P0_2
                                   1161 ;	servo.c:339: servo_control(1, 90);//right
      00043D 75 09 5A         [24] 1162 	mov	_servo_control_PARM_2,#0x5a
      000440 75 82 01         [24] 1163 	mov	dpl,#0x01
      000443 12 02 A6         [24] 1164 	lcall	_servo_control
      000446                       1165 00107$:
                                   1166 ;	servo.c:341: if (P3_5 == 0) {
      000446 20 B5 0B         [24] 1167 	jb	_P3_5,00113$
                                   1168 ;	servo.c:342: P0_3 = !P0_3;
      000449 B2 83            [12] 1169 	cpl	_P0_3
                                   1170 ;	servo.c:343: servo_control(1, 179);//right
      00044B 75 09 B3         [24] 1171 	mov	_servo_control_PARM_2,#0xb3
      00044E 75 82 01         [24] 1172 	mov	dpl,#0x01
      000451 12 02 A6         [24] 1173 	lcall	_servo_control
      000454                       1174 00113$:
                                   1175 ;	servo.c:345: }
      000454 D0 D0            [24] 1176 	pop	psw
      000456 D0 00            [24] 1177 	pop	(0+0)
      000458 D0 01            [24] 1178 	pop	(0+1)
      00045A D0 02            [24] 1179 	pop	(0+2)
      00045C D0 03            [24] 1180 	pop	(0+3)
      00045E D0 04            [24] 1181 	pop	(0+4)
      000460 D0 05            [24] 1182 	pop	(0+5)
      000462 D0 06            [24] 1183 	pop	(0+6)
      000464 D0 07            [24] 1184 	pop	(0+7)
      000466 D0 83            [24] 1185 	pop	dph
      000468 D0 82            [24] 1186 	pop	dpl
      00046A D0 F0            [24] 1187 	pop	b
      00046C D0 E0            [24] 1188 	pop	acc
      00046E D0 20            [24] 1189 	pop	bits
      000470 32               [24] 1190 	reti
                                   1191 	.area CSEG    (CODE)
                                   1192 	.area CONST   (CODE)
                                   1193 	.area XINIT   (CODE)
                                   1194 	.area CABS    (ABS,CODE)
