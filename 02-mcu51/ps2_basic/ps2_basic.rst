                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module ps2_basic
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _main
                                     12 	.globl _Read_PS2
                                     13 	.globl _scanout
                                     14 	.globl _psin
                                     15 	.globl _delay
                                     16 	.globl _uart_sendata
                                     17 	.globl _uart_init
                                     18 	.globl _delayms
                                     19 	.globl _CY
                                     20 	.globl _AC
                                     21 	.globl _F0
                                     22 	.globl _RS1
                                     23 	.globl _RS0
                                     24 	.globl _OV
                                     25 	.globl _F1
                                     26 	.globl _P
                                     27 	.globl _PS
                                     28 	.globl _PT1
                                     29 	.globl _PX1
                                     30 	.globl _PT0
                                     31 	.globl _PX0
                                     32 	.globl _RD
                                     33 	.globl _WR
                                     34 	.globl _T1
                                     35 	.globl _T0
                                     36 	.globl _INT1
                                     37 	.globl _INT0
                                     38 	.globl _TXD
                                     39 	.globl _RXD
                                     40 	.globl _P3_7
                                     41 	.globl _P3_6
                                     42 	.globl _P3_5
                                     43 	.globl _P3_4
                                     44 	.globl _P3_3
                                     45 	.globl _P3_2
                                     46 	.globl _P3_1
                                     47 	.globl _P3_0
                                     48 	.globl _EA
                                     49 	.globl _ES
                                     50 	.globl _ET1
                                     51 	.globl _EX1
                                     52 	.globl _ET0
                                     53 	.globl _EX0
                                     54 	.globl _P2_7
                                     55 	.globl _P2_6
                                     56 	.globl _P2_5
                                     57 	.globl _P2_4
                                     58 	.globl _P2_3
                                     59 	.globl _P2_2
                                     60 	.globl _P2_1
                                     61 	.globl _P2_0
                                     62 	.globl _SM0
                                     63 	.globl _SM1
                                     64 	.globl _SM2
                                     65 	.globl _REN
                                     66 	.globl _TB8
                                     67 	.globl _RB8
                                     68 	.globl _TI
                                     69 	.globl _RI
                                     70 	.globl _P1_7
                                     71 	.globl _P1_6
                                     72 	.globl _P1_5
                                     73 	.globl _P1_4
                                     74 	.globl _P1_3
                                     75 	.globl _P1_2
                                     76 	.globl _P1_1
                                     77 	.globl _P1_0
                                     78 	.globl _TF1
                                     79 	.globl _TR1
                                     80 	.globl _TF0
                                     81 	.globl _TR0
                                     82 	.globl _IE1
                                     83 	.globl _IT1
                                     84 	.globl _IE0
                                     85 	.globl _IT0
                                     86 	.globl _P0_7
                                     87 	.globl _P0_6
                                     88 	.globl _P0_5
                                     89 	.globl _P0_4
                                     90 	.globl _P0_3
                                     91 	.globl _P0_2
                                     92 	.globl _P0_1
                                     93 	.globl _P0_0
                                     94 	.globl _B
                                     95 	.globl _ACC
                                     96 	.globl _PSW
                                     97 	.globl _IP
                                     98 	.globl _P3
                                     99 	.globl _IE
                                    100 	.globl _P2
                                    101 	.globl _SBUF
                                    102 	.globl _SCON
                                    103 	.globl _P1
                                    104 	.globl _TH1
                                    105 	.globl _TH0
                                    106 	.globl _TL1
                                    107 	.globl _TL0
                                    108 	.globl _TMOD
                                    109 	.globl _TCON
                                    110 	.globl _PCON
                                    111 	.globl _DPH
                                    112 	.globl _DPL
                                    113 	.globl _SP
                                    114 	.globl _P0
                                    115 	.globl _out
                                    116 	.globl _scan
                                    117 ;--------------------------------------------------------
                                    118 ; special function registers
                                    119 ;--------------------------------------------------------
                                    120 	.area RSEG    (ABS,DATA)
      000000                        121 	.org 0x0000
                           000080   122 _P0	=	0x0080
                           000081   123 _SP	=	0x0081
                           000082   124 _DPL	=	0x0082
                           000083   125 _DPH	=	0x0083
                           000087   126 _PCON	=	0x0087
                           000088   127 _TCON	=	0x0088
                           000089   128 _TMOD	=	0x0089
                           00008A   129 _TL0	=	0x008a
                           00008B   130 _TL1	=	0x008b
                           00008C   131 _TH0	=	0x008c
                           00008D   132 _TH1	=	0x008d
                           000090   133 _P1	=	0x0090
                           000098   134 _SCON	=	0x0098
                           000099   135 _SBUF	=	0x0099
                           0000A0   136 _P2	=	0x00a0
                           0000A8   137 _IE	=	0x00a8
                           0000B0   138 _P3	=	0x00b0
                           0000B8   139 _IP	=	0x00b8
                           0000D0   140 _PSW	=	0x00d0
                           0000E0   141 _ACC	=	0x00e0
                           0000F0   142 _B	=	0x00f0
                                    143 ;--------------------------------------------------------
                                    144 ; special function bits
                                    145 ;--------------------------------------------------------
                                    146 	.area RSEG    (ABS,DATA)
      000000                        147 	.org 0x0000
                           000080   148 _P0_0	=	0x0080
                           000081   149 _P0_1	=	0x0081
                           000082   150 _P0_2	=	0x0082
                           000083   151 _P0_3	=	0x0083
                           000084   152 _P0_4	=	0x0084
                           000085   153 _P0_5	=	0x0085
                           000086   154 _P0_6	=	0x0086
                           000087   155 _P0_7	=	0x0087
                           000088   156 _IT0	=	0x0088
                           000089   157 _IE0	=	0x0089
                           00008A   158 _IT1	=	0x008a
                           00008B   159 _IE1	=	0x008b
                           00008C   160 _TR0	=	0x008c
                           00008D   161 _TF0	=	0x008d
                           00008E   162 _TR1	=	0x008e
                           00008F   163 _TF1	=	0x008f
                           000090   164 _P1_0	=	0x0090
                           000091   165 _P1_1	=	0x0091
                           000092   166 _P1_2	=	0x0092
                           000093   167 _P1_3	=	0x0093
                           000094   168 _P1_4	=	0x0094
                           000095   169 _P1_5	=	0x0095
                           000096   170 _P1_6	=	0x0096
                           000097   171 _P1_7	=	0x0097
                           000098   172 _RI	=	0x0098
                           000099   173 _TI	=	0x0099
                           00009A   174 _RB8	=	0x009a
                           00009B   175 _TB8	=	0x009b
                           00009C   176 _REN	=	0x009c
                           00009D   177 _SM2	=	0x009d
                           00009E   178 _SM1	=	0x009e
                           00009F   179 _SM0	=	0x009f
                           0000A0   180 _P2_0	=	0x00a0
                           0000A1   181 _P2_1	=	0x00a1
                           0000A2   182 _P2_2	=	0x00a2
                           0000A3   183 _P2_3	=	0x00a3
                           0000A4   184 _P2_4	=	0x00a4
                           0000A5   185 _P2_5	=	0x00a5
                           0000A6   186 _P2_6	=	0x00a6
                           0000A7   187 _P2_7	=	0x00a7
                           0000A8   188 _EX0	=	0x00a8
                           0000A9   189 _ET0	=	0x00a9
                           0000AA   190 _EX1	=	0x00aa
                           0000AB   191 _ET1	=	0x00ab
                           0000AC   192 _ES	=	0x00ac
                           0000AF   193 _EA	=	0x00af
                           0000B0   194 _P3_0	=	0x00b0
                           0000B1   195 _P3_1	=	0x00b1
                           0000B2   196 _P3_2	=	0x00b2
                           0000B3   197 _P3_3	=	0x00b3
                           0000B4   198 _P3_4	=	0x00b4
                           0000B5   199 _P3_5	=	0x00b5
                           0000B6   200 _P3_6	=	0x00b6
                           0000B7   201 _P3_7	=	0x00b7
                           0000B0   202 _RXD	=	0x00b0
                           0000B1   203 _TXD	=	0x00b1
                           0000B2   204 _INT0	=	0x00b2
                           0000B3   205 _INT1	=	0x00b3
                           0000B4   206 _T0	=	0x00b4
                           0000B5   207 _T1	=	0x00b5
                           0000B6   208 _WR	=	0x00b6
                           0000B7   209 _RD	=	0x00b7
                           0000B8   210 _PX0	=	0x00b8
                           0000B9   211 _PT0	=	0x00b9
                           0000BA   212 _PX1	=	0x00ba
                           0000BB   213 _PT1	=	0x00bb
                           0000BC   214 _PS	=	0x00bc
                           0000D0   215 _P	=	0x00d0
                           0000D1   216 _F1	=	0x00d1
                           0000D2   217 _OV	=	0x00d2
                           0000D3   218 _RS0	=	0x00d3
                           0000D4   219 _RS1	=	0x00d4
                           0000D5   220 _F0	=	0x00d5
                           0000D6   221 _AC	=	0x00d6
                           0000D7   222 _CY	=	0x00d7
                                    223 ;--------------------------------------------------------
                                    224 ; overlayable register banks
                                    225 ;--------------------------------------------------------
                                    226 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        227 	.ds 8
                                    228 ;--------------------------------------------------------
                                    229 ; internal ram data
                                    230 ;--------------------------------------------------------
                                    231 	.area DSEG    (DATA)
      000008                        232 _scan::
      000008                        233 	.ds 9
      000011                        234 _out::
      000011                        235 	.ds 9
                                    236 ;--------------------------------------------------------
                                    237 ; overlayable items in internal ram 
                                    238 ;--------------------------------------------------------
                                    239 	.area	OSEG    (OVR,DATA)
                                    240 	.area	OSEG    (OVR,DATA)
                                    241 	.area	OSEG    (OVR,DATA)
                                    242 ;--------------------------------------------------------
                                    243 ; Stack segment in internal ram 
                                    244 ;--------------------------------------------------------
                                    245 	.area	SSEG
      00001C                        246 __start__stack:
      00001C                        247 	.ds	1
                                    248 
                                    249 ;--------------------------------------------------------
                                    250 ; indirectly addressable internal ram data
                                    251 ;--------------------------------------------------------
                                    252 	.area ISEG    (DATA)
                                    253 ;--------------------------------------------------------
                                    254 ; absolute internal ram data
                                    255 ;--------------------------------------------------------
                                    256 	.area IABS    (ABS,DATA)
                                    257 	.area IABS    (ABS,DATA)
                                    258 ;--------------------------------------------------------
                                    259 ; bit data
                                    260 ;--------------------------------------------------------
                                    261 	.area BSEG    (BIT)
                                    262 ;--------------------------------------------------------
                                    263 ; paged external ram data
                                    264 ;--------------------------------------------------------
                                    265 	.area PSEG    (PAG,XDATA)
                                    266 ;--------------------------------------------------------
                                    267 ; external ram data
                                    268 ;--------------------------------------------------------
                                    269 	.area XSEG    (XDATA)
                                    270 ;--------------------------------------------------------
                                    271 ; absolute external ram data
                                    272 ;--------------------------------------------------------
                                    273 	.area XABS    (ABS,XDATA)
                                    274 ;--------------------------------------------------------
                                    275 ; external initialized ram data
                                    276 ;--------------------------------------------------------
                                    277 	.area XISEG   (XDATA)
                                    278 	.area HOME    (CODE)
                                    279 	.area GSINIT0 (CODE)
                                    280 	.area GSINIT1 (CODE)
                                    281 	.area GSINIT2 (CODE)
                                    282 	.area GSINIT3 (CODE)
                                    283 	.area GSINIT4 (CODE)
                                    284 	.area GSINIT5 (CODE)
                                    285 	.area GSINIT  (CODE)
                                    286 	.area GSFINAL (CODE)
                                    287 	.area CSEG    (CODE)
                                    288 ;--------------------------------------------------------
                                    289 ; interrupt vector 
                                    290 ;--------------------------------------------------------
                                    291 	.area HOME    (CODE)
      000000                        292 __interrupt_vect:
      000000 02 00 06         [24]  293 	ljmp	__sdcc_gsinit_startup
                                    294 ;--------------------------------------------------------
                                    295 ; global & static initialisations
                                    296 ;--------------------------------------------------------
                                    297 	.area HOME    (CODE)
                                    298 	.area GSINIT  (CODE)
                                    299 	.area GSFINAL (CODE)
                                    300 	.area GSINIT  (CODE)
                                    301 	.globl __sdcc_gsinit_startup
                                    302 	.globl __sdcc_program_startup
                                    303 	.globl __start__stack
                                    304 	.globl __mcs51_genXINIT
                                    305 	.globl __mcs51_genXRAMCLEAR
                                    306 	.globl __mcs51_genRAMCLEAR
                                    307 ;	ps2_basic.c:24: uchar scan[9] = {0x01, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00};
      00005F 75 08 01         [24]  308 	mov	_scan,#0x01
      000062 75 09 42         [24]  309 	mov	(_scan + 0x0001),#0x42
      000065 75 0A 00         [24]  310 	mov	(_scan + 0x0002),#0x00
      000068 75 0B 00         [24]  311 	mov	(_scan + 0x0003),#0x00
      00006B 75 0C 00         [24]  312 	mov	(_scan + 0x0004),#0x00
      00006E 75 0D 00         [24]  313 	mov	(_scan + 0x0005),#0x00
      000071 75 0E 00         [24]  314 	mov	(_scan + 0x0006),#0x00
      000074 75 0F 00         [24]  315 	mov	(_scan + 0x0007),#0x00
      000077 75 10 00         [24]  316 	mov	(_scan + 0x0008),#0x00
                                    317 	.area GSFINAL (CODE)
      00007A 02 00 03         [24]  318 	ljmp	__sdcc_program_startup
                                    319 ;--------------------------------------------------------
                                    320 ; Home
                                    321 ;--------------------------------------------------------
                                    322 	.area HOME    (CODE)
                                    323 	.area HOME    (CODE)
      000003                        324 __sdcc_program_startup:
      000003 02 02 3D         [24]  325 	ljmp	_main
                                    326 ;	return from main will return to caller
                                    327 ;--------------------------------------------------------
                                    328 ; code
                                    329 ;--------------------------------------------------------
                                    330 	.area CSEG    (CODE)
                                    331 ;------------------------------------------------------------
                                    332 ;Allocation info for local variables in function 'delayms'
                                    333 ;------------------------------------------------------------
                                    334 ;xms                       Allocated to registers 
                                    335 ;i                         Allocated to registers r6 r7 
                                    336 ;j                         Allocated to registers r4 r5 
                                    337 ;------------------------------------------------------------
                                    338 ;	ps2_basic.c:33: void delayms(uint xms)
                                    339 ;	-----------------------------------------
                                    340 ;	 function delayms
                                    341 ;	-----------------------------------------
      00007D                        342 _delayms:
                           000007   343 	ar7 = 0x07
                           000006   344 	ar6 = 0x06
                           000005   345 	ar5 = 0x05
                           000004   346 	ar4 = 0x04
                           000003   347 	ar3 = 0x03
                           000002   348 	ar2 = 0x02
                           000001   349 	ar1 = 0x01
                           000000   350 	ar0 = 0x00
      00007D AE 82            [24]  351 	mov	r6,dpl
      00007F AF 83            [24]  352 	mov	r7,dph
                                    353 ;	ps2_basic.c:36: for (i = xms; i > 0; i--)
      000081                        354 00106$:
      000081 EE               [12]  355 	mov	a,r6
      000082 4F               [12]  356 	orl	a,r7
      000083 60 1B            [24]  357 	jz	00108$
                                    358 ;	ps2_basic.c:37: for (j = 70; j > 0; j--)
      000085 7C 46            [12]  359 	mov	r4,#0x46
      000087 7D 00            [12]  360 	mov	r5,#0x00
      000089                        361 00104$:
      000089 EC               [12]  362 	mov	a,r4
      00008A 24 FF            [12]  363 	add	a,#0xff
      00008C FA               [12]  364 	mov	r2,a
      00008D ED               [12]  365 	mov	a,r5
      00008E 34 FF            [12]  366 	addc	a,#0xff
      000090 FB               [12]  367 	mov	r3,a
      000091 8A 04            [24]  368 	mov	ar4,r2
      000093 8B 05            [24]  369 	mov	ar5,r3
      000095 EA               [12]  370 	mov	a,r2
      000096 4B               [12]  371 	orl	a,r3
      000097 70 F0            [24]  372 	jnz	00104$
                                    373 ;	ps2_basic.c:36: for (i = xms; i > 0; i--)
      000099 1E               [12]  374 	dec	r6
      00009A BE FF 01         [24]  375 	cjne	r6,#0xff,00133$
      00009D 1F               [12]  376 	dec	r7
      00009E                        377 00133$:
      00009E 80 E1            [24]  378 	sjmp	00106$
      0000A0                        379 00108$:
                                    380 ;	ps2_basic.c:39: }
      0000A0 22               [24]  381 	ret
                                    382 ;------------------------------------------------------------
                                    383 ;Allocation info for local variables in function 'uart_init'
                                    384 ;------------------------------------------------------------
                                    385 ;	ps2_basic.c:46: void uart_init()
                                    386 ;	-----------------------------------------
                                    387 ;	 function uart_init
                                    388 ;	-----------------------------------------
      0000A1                        389 _uart_init:
                                    390 ;	ps2_basic.c:48: TMOD=0x20;		   //用定时器设置串口波特率	   9600 
      0000A1 75 89 20         [24]  391 	mov	_TMOD,#0x20
                                    392 ;	ps2_basic.c:49: TH1=0xfd;
      0000A4 75 8D FD         [24]  393 	mov	_TH1,#0xfd
                                    394 ;	ps2_basic.c:50: TL1=0xfd;
      0000A7 75 8B FD         [24]  395 	mov	_TL1,#0xfd
                                    396 ;	ps2_basic.c:51: TR1=1;
                                    397 ;	assignBit
      0000AA D2 8E            [12]  398 	setb	_TR1
                                    399 ;	ps2_basic.c:52: REN=1;          //串口初始化
                                    400 ;	assignBit
      0000AC D2 9C            [12]  401 	setb	_REN
                                    402 ;	ps2_basic.c:53: SM0=0;
                                    403 ;	assignBit
      0000AE C2 9F            [12]  404 	clr	_SM0
                                    405 ;	ps2_basic.c:54: SM1=1;
                                    406 ;	assignBit
      0000B0 D2 9E            [12]  407 	setb	_SM1
                                    408 ;	ps2_basic.c:55: EA=1;           //开启总中断
                                    409 ;	assignBit
      0000B2 D2 AF            [12]  410 	setb	_EA
                                    411 ;	ps2_basic.c:56: ES=1;
                                    412 ;	assignBit
      0000B4 D2 AC            [12]  413 	setb	_ES
                                    414 ;	ps2_basic.c:57: }
      0000B6 22               [24]  415 	ret
                                    416 ;------------------------------------------------------------
                                    417 ;Allocation info for local variables in function 'uart_sendata'
                                    418 ;------------------------------------------------------------
                                    419 ;n                         Allocated to registers r7 
                                    420 ;------------------------------------------------------------
                                    421 ;	ps2_basic.c:65: void uart_sendata(uchar n)
                                    422 ;	-----------------------------------------
                                    423 ;	 function uart_sendata
                                    424 ;	-----------------------------------------
      0000B7                        425 _uart_sendata:
      0000B7 AF 82            [24]  426 	mov	r7,dpl
                                    427 ;	ps2_basic.c:67: ES = 0;
                                    428 ;	assignBit
      0000B9 C2 AC            [12]  429 	clr	_ES
                                    430 ;	ps2_basic.c:68: TI = 0;
                                    431 ;	assignBit
      0000BB C2 99            [12]  432 	clr	_TI
                                    433 ;	ps2_basic.c:69: SBUF = n;
      0000BD 8F 99            [24]  434 	mov	_SBUF,r7
                                    435 ;	ps2_basic.c:70: while (!TI)
      0000BF                        436 00101$:
                                    437 ;	ps2_basic.c:72: TI = 0;
                                    438 ;	assignBit
      0000BF 10 99 02         [24]  439 	jbc	_TI,00114$
      0000C2 80 FB            [24]  440 	sjmp	00101$
      0000C4                        441 00114$:
                                    442 ;	ps2_basic.c:73: ES = 1;
                                    443 ;	assignBit
      0000C4 D2 AC            [12]  444 	setb	_ES
                                    445 ;	ps2_basic.c:74: }
      0000C6 22               [24]  446 	ret
                                    447 ;------------------------------------------------------------
                                    448 ;Allocation info for local variables in function 'delay'
                                    449 ;------------------------------------------------------------
                                    450 ;n                         Allocated to registers r6 r7 
                                    451 ;i                         Allocated to registers r4 r5 
                                    452 ;------------------------------------------------------------
                                    453 ;	ps2_basic.c:76: void delay(uint n) //delay(x)=(2.5+x)us;
                                    454 ;	-----------------------------------------
                                    455 ;	 function delay
                                    456 ;	-----------------------------------------
      0000C7                        457 _delay:
      0000C7 AE 82            [24]  458 	mov	r6,dpl
      0000C9 AF 83            [24]  459 	mov	r7,dph
                                    460 ;	ps2_basic.c:79: for (i = 0; i < n; i++)
      0000CB 7C 00            [12]  461 	mov	r4,#0x00
      0000CD 7D 00            [12]  462 	mov	r5,#0x00
      0000CF                        463 00103$:
      0000CF C3               [12]  464 	clr	c
      0000D0 EC               [12]  465 	mov	a,r4
      0000D1 9E               [12]  466 	subb	a,r6
      0000D2 ED               [12]  467 	mov	a,r5
      0000D3 9F               [12]  468 	subb	a,r7
      0000D4 50 08            [24]  469 	jnc	00105$
                                    470 ;	ps2_basic.c:80: _nop_();
      0000D6 00               [12]  471 	NOP	
                                    472 ;	ps2_basic.c:79: for (i = 0; i < n; i++)
      0000D7 0C               [12]  473 	inc	r4
      0000D8 BC 00 F4         [24]  474 	cjne	r4,#0x00,00103$
      0000DB 0D               [12]  475 	inc	r5
      0000DC 80 F1            [24]  476 	sjmp	00103$
      0000DE                        477 00105$:
                                    478 ;	ps2_basic.c:81: }
      0000DE 22               [24]  479 	ret
                                    480 ;------------------------------------------------------------
                                    481 ;Allocation info for local variables in function 'psin'
                                    482 ;------------------------------------------------------------
                                    483 ;command                   Allocated to registers r7 
                                    484 ;i                         Allocated to registers r6 
                                    485 ;------------------------------------------------------------
                                    486 ;	ps2_basic.c:83: void psin(uchar command) //手柄发送子程序
                                    487 ;	-----------------------------------------
                                    488 ;	 function psin
                                    489 ;	-----------------------------------------
      0000DF                        490 _psin:
      0000DF AF 82            [24]  491 	mov	r7,dpl
                                    492 ;	ps2_basic.c:86: for (i = 0; i <= 7; i++) //逐位接收
      0000E1 7E 00            [12]  493 	mov	r6,#0x00
      0000E3                        494 00105$:
                                    495 ;	ps2_basic.c:88: if (command & 0x01) //此if下5行语句用时1us
      0000E3 EF               [12]  496 	mov	a,r7
      0000E4 30 E0 04         [24]  497 	jnb	acc.0,00102$
                                    498 ;	ps2_basic.c:89: CMND = 1;
                                    499 ;	assignBit
      0000E7 D2 91            [12]  500 	setb	_P1_1
      0000E9 80 02            [24]  501 	sjmp	00103$
      0000EB                        502 00102$:
                                    503 ;	ps2_basic.c:91: CMND = 0;
                                    504 ;	assignBit
      0000EB C2 91            [12]  505 	clr	_P1_1
      0000ED                        506 00103$:
                                    507 ;	ps2_basic.c:92: command = command >> 1;
      0000ED EF               [12]  508 	mov	a,r7
      0000EE C3               [12]  509 	clr	c
      0000EF 13               [12]  510 	rrc	a
      0000F0 FF               [12]  511 	mov	r7,a
                                    512 ;	ps2_basic.c:93: _nop_();
      0000F1 00               [12]  513 	NOP	
                                    514 ;	ps2_basic.c:94: _nop_();
      0000F2 00               [12]  515 	NOP	
                                    516 ;	ps2_basic.c:95: CLK = 0;
                                    517 ;	assignBit
      0000F3 C2 93            [12]  518 	clr	_P1_3
                                    519 ;	ps2_basic.c:96: delay(3);
      0000F5 90 00 03         [24]  520 	mov	dptr,#0x0003
      0000F8 C0 07            [24]  521 	push	ar7
      0000FA C0 06            [24]  522 	push	ar6
      0000FC 12 00 C7         [24]  523 	lcall	_delay
                                    524 ;	ps2_basic.c:97: CLK = 1;
                                    525 ;	assignBit
      0000FF D2 93            [12]  526 	setb	_P1_3
                                    527 ;	ps2_basic.c:98: delay(1);
      000101 90 00 01         [24]  528 	mov	dptr,#0x0001
      000104 12 00 C7         [24]  529 	lcall	_delay
      000107 D0 06            [24]  530 	pop	ar6
      000109 D0 07            [24]  531 	pop	ar7
                                    532 ;	ps2_basic.c:86: for (i = 0; i <= 7; i++) //逐位接收
      00010B 0E               [12]  533 	inc	r6
      00010C EE               [12]  534 	mov	a,r6
      00010D 24 F8            [12]  535 	add	a,#0xff - 0x07
      00010F 50 D2            [24]  536 	jnc	00105$
                                    537 ;	ps2_basic.c:100: CMND = 1;
                                    538 ;	assignBit
      000111 D2 91            [12]  539 	setb	_P1_1
                                    540 ;	ps2_basic.c:101: }
      000113 22               [24]  541 	ret
                                    542 ;------------------------------------------------------------
                                    543 ;Allocation info for local variables in function 'scanout'
                                    544 ;------------------------------------------------------------
                                    545 ;command                   Allocated to registers r7 
                                    546 ;i                         Allocated to registers r4 
                                    547 ;j                         Allocated to registers r6 
                                    548 ;res                       Allocated to registers r5 
                                    549 ;------------------------------------------------------------
                                    550 ;	ps2_basic.c:103: uchar scanout(uchar command) //手柄发送子程序
                                    551 ;	-----------------------------------------
                                    552 ;	 function scanout
                                    553 ;	-----------------------------------------
      000114                        554 _scanout:
      000114 AF 82            [24]  555 	mov	r7,dpl
                                    556 ;	ps2_basic.c:105: uchar i, j = 1;
      000116 7E 01            [12]  557 	mov	r6,#0x01
                                    558 ;	ps2_basic.c:106: uchar res = 0;
      000118 7D 00            [12]  559 	mov	r5,#0x00
                                    560 ;	ps2_basic.c:107: for (i = 0; i <= 7; i++) //逐位接收
      00011A 7C 00            [12]  561 	mov	r4,#0x00
      00011C                        562 00107$:
                                    563 ;	ps2_basic.c:109: if (command & 0x01)
      00011C EF               [12]  564 	mov	a,r7
      00011D 30 E0 04         [24]  565 	jnb	acc.0,00102$
                                    566 ;	ps2_basic.c:110: CMND = 1;
                                    567 ;	assignBit
      000120 D2 91            [12]  568 	setb	_P1_1
      000122 80 02            [24]  569 	sjmp	00103$
      000124                        570 00102$:
                                    571 ;	ps2_basic.c:112: CMND = 0;
                                    572 ;	assignBit
      000124 C2 91            [12]  573 	clr	_P1_1
      000126                        574 00103$:
                                    575 ;	ps2_basic.c:113: command = command >> 1;
      000126 EF               [12]  576 	mov	a,r7
      000127 C3               [12]  577 	clr	c
      000128 13               [12]  578 	rrc	a
      000129 FF               [12]  579 	mov	r7,a
                                    580 ;	ps2_basic.c:114: _nop_();
      00012A 00               [12]  581 	NOP	
                                    582 ;	ps2_basic.c:115: _nop_();
      00012B 00               [12]  583 	NOP	
                                    584 ;	ps2_basic.c:116: CLK = 0;
                                    585 ;	assignBit
      00012C C2 93            [12]  586 	clr	_P1_3
                                    587 ;	ps2_basic.c:118: if (DATA)
      00012E 30 90 03         [24]  588 	jnb	_P1_0,00105$
                                    589 ;	ps2_basic.c:119: res = res + j;
      000131 EE               [12]  590 	mov	a,r6
      000132 2D               [12]  591 	add	a,r5
      000133 FD               [12]  592 	mov	r5,a
      000134                        593 00105$:
                                    594 ;	ps2_basic.c:120: j = j << 1;
      000134 8E 03            [24]  595 	mov	ar3,r6
      000136 EB               [12]  596 	mov	a,r3
      000137 2B               [12]  597 	add	a,r3
      000138 FE               [12]  598 	mov	r6,a
                                    599 ;	ps2_basic.c:121: CLK = 1;
                                    600 ;	assignBit
      000139 D2 93            [12]  601 	setb	_P1_3
                                    602 ;	ps2_basic.c:122: delay(3);
      00013B 90 00 03         [24]  603 	mov	dptr,#0x0003
      00013E C0 07            [24]  604 	push	ar7
      000140 C0 06            [24]  605 	push	ar6
      000142 C0 05            [24]  606 	push	ar5
      000144 C0 04            [24]  607 	push	ar4
      000146 12 00 C7         [24]  608 	lcall	_delay
      000149 D0 04            [24]  609 	pop	ar4
      00014B D0 05            [24]  610 	pop	ar5
      00014D D0 06            [24]  611 	pop	ar6
      00014F D0 07            [24]  612 	pop	ar7
                                    613 ;	ps2_basic.c:107: for (i = 0; i <= 7; i++) //逐位接收
      000151 0C               [12]  614 	inc	r4
      000152 EC               [12]  615 	mov	a,r4
      000153 24 F8            [12]  616 	add	a,#0xff - 0x07
      000155 50 C5            [24]  617 	jnc	00107$
                                    618 ;	ps2_basic.c:124: CMND = 1;
                                    619 ;	assignBit
      000157 D2 91            [12]  620 	setb	_P1_1
                                    621 ;	ps2_basic.c:125: return res;
      000159 8D 82            [24]  622 	mov	dpl,r5
                                    623 ;	ps2_basic.c:126: }
      00015B 22               [24]  624 	ret
                                    625 ;------------------------------------------------------------
                                    626 ;Allocation info for local variables in function 'Read_PS2'
                                    627 ;------------------------------------------------------------
                                    628 ;i                         Allocated to registers r7 
                                    629 ;------------------------------------------------------------
                                    630 ;	ps2_basic.c:127: void Read_PS2(void) //手柄读取程序
                                    631 ;	-----------------------------------------
                                    632 ;	 function Read_PS2
                                    633 ;	-----------------------------------------
      00015C                        634 _Read_PS2:
                                    635 ;	ps2_basic.c:130: ATT = 0;
                                    636 ;	assignBit
      00015C C2 92            [12]  637 	clr	_P1_2
                                    638 ;	ps2_basic.c:131: for (i = 0; i < 9; i++) //扫描按键
      00015E 7F 00            [12]  639 	mov	r7,#0x00
      000160                        640 00109$:
                                    641 ;	ps2_basic.c:133: out[i] = scanout(scan[i]);
      000160 EF               [12]  642 	mov	a,r7
      000161 24 11            [12]  643 	add	a,#_out
      000163 F9               [12]  644 	mov	r1,a
      000164 EF               [12]  645 	mov	a,r7
      000165 24 08            [12]  646 	add	a,#_scan
      000167 F8               [12]  647 	mov	r0,a
      000168 86 82            [24]  648 	mov	dpl,@r0
      00016A C0 07            [24]  649 	push	ar7
      00016C C0 01            [24]  650 	push	ar1
      00016E 12 01 14         [24]  651 	lcall	_scanout
      000171 E5 82            [12]  652 	mov	a,dpl
      000173 D0 01            [24]  653 	pop	ar1
      000175 D0 07            [24]  654 	pop	ar7
      000177 F7               [12]  655 	mov	@r1,a
                                    656 ;	ps2_basic.c:131: for (i = 0; i < 9; i++) //扫描按键
      000178 0F               [12]  657 	inc	r7
      000179 BF 09 00         [24]  658 	cjne	r7,#0x09,00141$
      00017C                        659 00141$:
      00017C 40 E2            [24]  660 	jc	00109$
                                    661 ;	ps2_basic.c:135: ATT = 1;
                                    662 ;	assignBit
      00017E D2 92            [12]  663 	setb	_P1_2
                                    664 ;	ps2_basic.c:136: for (i = 0; i < 9; i++)
      000180 7F 00            [12]  665 	mov	r7,#0x00
      000182                        666 00111$:
                                    667 ;	ps2_basic.c:138: uart_sendata(0x30);
      000182 75 82 30         [24]  668 	mov	dpl,#0x30
      000185 C0 07            [24]  669 	push	ar7
      000187 12 00 B7         [24]  670 	lcall	_uart_sendata
                                    671 ;	ps2_basic.c:139: uart_sendata(0x78);
      00018A 75 82 78         [24]  672 	mov	dpl,#0x78
      00018D 12 00 B7         [24]  673 	lcall	_uart_sendata
      000190 D0 07            [24]  674 	pop	ar7
                                    675 ;	ps2_basic.c:140: if (out[i] / 16 < 10)
      000192 EF               [12]  676 	mov	a,r7
      000193 24 11            [12]  677 	add	a,#_out
      000195 F9               [12]  678 	mov	r1,a
      000196 87 06            [24]  679 	mov	ar6,@r1
      000198 7D 00            [12]  680 	mov	r5,#0x00
      00019A 75 1A 10         [24]  681 	mov	__divsint_PARM_2,#0x10
                                    682 ;	1-genFromRTrack replaced	mov	(__divsint_PARM_2 + 1),#0x00
      00019D 8D 1B            [24]  683 	mov	(__divsint_PARM_2 + 1),r5
      00019F 8E 82            [24]  684 	mov	dpl,r6
      0001A1 8D 83            [24]  685 	mov	dph,r5
      0001A3 C0 07            [24]  686 	push	ar7
      0001A5 12 03 6C         [24]  687 	lcall	__divsint
      0001A8 AD 82            [24]  688 	mov	r5,dpl
      0001AA AE 83            [24]  689 	mov	r6,dph
      0001AC D0 07            [24]  690 	pop	ar7
      0001AE C3               [12]  691 	clr	c
      0001AF ED               [12]  692 	mov	a,r5
      0001B0 94 0A            [12]  693 	subb	a,#0x0a
      0001B2 EE               [12]  694 	mov	a,r6
      0001B3 64 80            [12]  695 	xrl	a,#0x80
      0001B5 94 80            [12]  696 	subb	a,#0x80
      0001B7 50 10            [24]  697 	jnc	00103$
                                    698 ;	ps2_basic.c:141: uart_sendata(out[i] / 16 + 0x30);
      0001B9 8D 04            [24]  699 	mov	ar4,r5
      0001BB 74 30            [12]  700 	mov	a,#0x30
      0001BD 2C               [12]  701 	add	a,r4
      0001BE F5 82            [12]  702 	mov	dpl,a
      0001C0 C0 07            [24]  703 	push	ar7
      0001C2 12 00 B7         [24]  704 	lcall	_uart_sendata
      0001C5 D0 07            [24]  705 	pop	ar7
      0001C7 80 0C            [24]  706 	sjmp	00104$
      0001C9                        707 00103$:
                                    708 ;	ps2_basic.c:143: uart_sendata(out[i] / 16 + 0x37);
      0001C9 74 37            [12]  709 	mov	a,#0x37
      0001CB 2D               [12]  710 	add	a,r5
      0001CC F5 82            [12]  711 	mov	dpl,a
      0001CE C0 07            [24]  712 	push	ar7
      0001D0 12 00 B7         [24]  713 	lcall	_uart_sendata
      0001D3 D0 07            [24]  714 	pop	ar7
      0001D5                        715 00104$:
                                    716 ;	ps2_basic.c:144: if (out[i] % 16 < 10)
      0001D5 EF               [12]  717 	mov	a,r7
      0001D6 24 11            [12]  718 	add	a,#_out
      0001D8 F9               [12]  719 	mov	r1,a
      0001D9 87 06            [24]  720 	mov	ar6,@r1
      0001DB 7D 00            [12]  721 	mov	r5,#0x00
      0001DD 75 1A 10         [24]  722 	mov	__modsint_PARM_2,#0x10
                                    723 ;	1-genFromRTrack replaced	mov	(__modsint_PARM_2 + 1),#0x00
      0001E0 8D 1B            [24]  724 	mov	(__modsint_PARM_2 + 1),r5
      0001E2 8E 82            [24]  725 	mov	dpl,r6
      0001E4 8D 83            [24]  726 	mov	dph,r5
      0001E6 C0 07            [24]  727 	push	ar7
      0001E8 12 03 36         [24]  728 	lcall	__modsint
      0001EB AD 82            [24]  729 	mov	r5,dpl
      0001ED AE 83            [24]  730 	mov	r6,dph
      0001EF D0 07            [24]  731 	pop	ar7
      0001F1 C3               [12]  732 	clr	c
      0001F2 ED               [12]  733 	mov	a,r5
      0001F3 94 0A            [12]  734 	subb	a,#0x0a
      0001F5 EE               [12]  735 	mov	a,r6
      0001F6 64 80            [12]  736 	xrl	a,#0x80
      0001F8 94 80            [12]  737 	subb	a,#0x80
      0001FA 50 10            [24]  738 	jnc	00106$
                                    739 ;	ps2_basic.c:145: uart_sendata(out[i] % 16 + 0x30);
      0001FC 8D 04            [24]  740 	mov	ar4,r5
      0001FE 74 30            [12]  741 	mov	a,#0x30
      000200 2C               [12]  742 	add	a,r4
      000201 F5 82            [12]  743 	mov	dpl,a
      000203 C0 07            [24]  744 	push	ar7
      000205 12 00 B7         [24]  745 	lcall	_uart_sendata
      000208 D0 07            [24]  746 	pop	ar7
      00020A 80 0C            [24]  747 	sjmp	00107$
      00020C                        748 00106$:
                                    749 ;	ps2_basic.c:147: uart_sendata(out[i] % 16 + 0x37);
      00020C 74 37            [12]  750 	mov	a,#0x37
      00020E 2D               [12]  751 	add	a,r5
      00020F F5 82            [12]  752 	mov	dpl,a
      000211 C0 07            [24]  753 	push	ar7
      000213 12 00 B7         [24]  754 	lcall	_uart_sendata
      000216 D0 07            [24]  755 	pop	ar7
      000218                        756 00107$:
                                    757 ;	ps2_basic.c:148: uart_sendata(' ');
      000218 75 82 20         [24]  758 	mov	dpl,#0x20
      00021B C0 07            [24]  759 	push	ar7
      00021D 12 00 B7         [24]  760 	lcall	_uart_sendata
                                    761 ;	ps2_basic.c:149: uart_sendata(' ');
      000220 75 82 20         [24]  762 	mov	dpl,#0x20
      000223 12 00 B7         [24]  763 	lcall	_uart_sendata
      000226 D0 07            [24]  764 	pop	ar7
                                    765 ;	ps2_basic.c:136: for (i = 0; i < 9; i++)
      000228 0F               [12]  766 	inc	r7
      000229 BF 09 00         [24]  767 	cjne	r7,#0x09,00145$
      00022C                        768 00145$:
      00022C 50 03            [24]  769 	jnc	00146$
      00022E 02 01 82         [24]  770 	ljmp	00111$
      000231                        771 00146$:
                                    772 ;	ps2_basic.c:151: uart_sendata(0x0d);
      000231 75 82 0D         [24]  773 	mov	dpl,#0x0d
      000234 12 00 B7         [24]  774 	lcall	_uart_sendata
                                    775 ;	ps2_basic.c:152: uart_sendata(0x0a);
      000237 75 82 0A         [24]  776 	mov	dpl,#0x0a
                                    777 ;	ps2_basic.c:153: }
      00023A 02 00 B7         [24]  778 	ljmp	_uart_sendata
                                    779 ;------------------------------------------------------------
                                    780 ;Allocation info for local variables in function 'main'
                                    781 ;------------------------------------------------------------
                                    782 ;	ps2_basic.c:154: void main()
                                    783 ;	-----------------------------------------
                                    784 ;	 function main
                                    785 ;	-----------------------------------------
      00023D                        786 _main:
                                    787 ;	ps2_basic.c:156: delayms(500);
      00023D 90 01 F4         [24]  788 	mov	dptr,#0x01f4
      000240 12 00 7D         [24]  789 	lcall	_delayms
                                    790 ;	ps2_basic.c:157: P0_0 = 0;
                                    791 ;	assignBit
      000243 C2 80            [12]  792 	clr	_P0_0
                                    793 ;	ps2_basic.c:158: uart_init();
      000245 12 00 A1         [24]  794 	lcall	_uart_init
                                    795 ;	ps2_basic.c:160: while (1)
      000248                        796 00114$:
                                    797 ;	ps2_basic.c:162: Read_PS2();
      000248 12 01 5C         [24]  798 	lcall	_Read_PS2
                                    799 ;	ps2_basic.c:164: switch (out[3])
      00024B AF 14            [24]  800 	mov	r7,(_out + 0x0003)
      00024D BF 7F 02         [24]  801 	cjne	r7,#0x7f,00154$
      000250 80 1F            [24]  802 	sjmp	00103$
      000252                        803 00154$:
      000252 BF BF 02         [24]  804 	cjne	r7,#0xbf,00155$
      000255 80 11            [24]  805 	sjmp	00102$
      000257                        806 00155$:
      000257 BF DF 02         [24]  807 	cjne	r7,#0xdf,00156$
      00025A 80 1E            [24]  808 	sjmp	00104$
      00025C                        809 00156$:
      00025C BF EF 22         [24]  810 	cjne	r7,#0xef,00106$
                                    811 ;	ps2_basic.c:167: P0_0 = !P0_0;//left top
      00025F B2 80            [12]  812 	cpl	_P0_0
                                    813 ;	ps2_basic.c:168: uart_sendata(out[3]);
      000261 8F 82            [24]  814 	mov	dpl,r7
      000263 12 00 B7         [24]  815 	lcall	_uart_sendata
                                    816 ;	ps2_basic.c:169: break;
                                    817 ;	ps2_basic.c:170: case 0xBF:
      000266 80 19            [24]  818 	sjmp	00106$
      000268                        819 00102$:
                                    820 ;	ps2_basic.c:171: P0_1 = !P0_1;//left down
      000268 B2 81            [12]  821 	cpl	_P0_1
                                    822 ;	ps2_basic.c:172: uart_sendata(out[3]);
      00026A 8F 82            [24]  823 	mov	dpl,r7
      00026C 12 00 B7         [24]  824 	lcall	_uart_sendata
                                    825 ;	ps2_basic.c:173: break;
                                    826 ;	ps2_basic.c:174: case 0x7F:
      00026F 80 10            [24]  827 	sjmp	00106$
      000271                        828 00103$:
                                    829 ;	ps2_basic.c:175: P0_2 = !P0_2;//left left
      000271 B2 82            [12]  830 	cpl	_P0_2
                                    831 ;	ps2_basic.c:176: uart_sendata(out[3]);
      000273 8F 82            [24]  832 	mov	dpl,r7
      000275 12 00 B7         [24]  833 	lcall	_uart_sendata
                                    834 ;	ps2_basic.c:177: break;
                                    835 ;	ps2_basic.c:178: case 0xDF:
      000278 80 07            [24]  836 	sjmp	00106$
      00027A                        837 00104$:
                                    838 ;	ps2_basic.c:179: P0_3 = !P0_3;//left right
      00027A B2 83            [12]  839 	cpl	_P0_3
                                    840 ;	ps2_basic.c:180: uart_sendata(out[3]);
      00027C 8F 82            [24]  841 	mov	dpl,r7
      00027E 12 00 B7         [24]  842 	lcall	_uart_sendata
                                    843 ;	ps2_basic.c:184: }
      000281                        844 00106$:
                                    845 ;	ps2_basic.c:186: switch (out[4])
      000281 AF 15            [24]  846 	mov	r7,(_out + 0x0004)
      000283 BF 7F 02         [24]  847 	cjne	r7,#0x7f,00159$
      000286 80 1F            [24]  848 	sjmp	00109$
      000288                        849 00159$:
      000288 BF BF 02         [24]  850 	cjne	r7,#0xbf,00160$
      00028B 80 11            [24]  851 	sjmp	00108$
      00028D                        852 00160$:
      00028D BF DF 02         [24]  853 	cjne	r7,#0xdf,00161$
      000290 80 1E            [24]  854 	sjmp	00110$
      000292                        855 00161$:
      000292 BF EF 22         [24]  856 	cjne	r7,#0xef,00112$
                                    857 ;	ps2_basic.c:190: P0_4 = !P0_4;//right top
      000295 B2 84            [12]  858 	cpl	_P0_4
                                    859 ;	ps2_basic.c:191: uart_sendata(out[4]);
      000297 8F 82            [24]  860 	mov	dpl,r7
      000299 12 00 B7         [24]  861 	lcall	_uart_sendata
                                    862 ;	ps2_basic.c:192: break;
                                    863 ;	ps2_basic.c:193: case 0xBF:
      00029C 80 19            [24]  864 	sjmp	00112$
      00029E                        865 00108$:
                                    866 ;	ps2_basic.c:194: P0_5 = !P0_5;//right down
      00029E B2 85            [12]  867 	cpl	_P0_5
                                    868 ;	ps2_basic.c:195: uart_sendata(out[4]);
      0002A0 8F 82            [24]  869 	mov	dpl,r7
      0002A2 12 00 B7         [24]  870 	lcall	_uart_sendata
                                    871 ;	ps2_basic.c:196: break;
                                    872 ;	ps2_basic.c:197: case 0x7F:
      0002A5 80 10            [24]  873 	sjmp	00112$
      0002A7                        874 00109$:
                                    875 ;	ps2_basic.c:198: P0_6 = !P0_6;//right left
      0002A7 B2 86            [12]  876 	cpl	_P0_6
                                    877 ;	ps2_basic.c:199: uart_sendata(out[4]);
      0002A9 8F 82            [24]  878 	mov	dpl,r7
      0002AB 12 00 B7         [24]  879 	lcall	_uart_sendata
                                    880 ;	ps2_basic.c:200: break;
                                    881 ;	ps2_basic.c:201: case 0xDF:
      0002AE 80 07            [24]  882 	sjmp	00112$
      0002B0                        883 00110$:
                                    884 ;	ps2_basic.c:202: P0_7 = !P0_7;//right right
      0002B0 B2 87            [12]  885 	cpl	_P0_7
                                    886 ;	ps2_basic.c:203: uart_sendata(out[4]);
      0002B2 8F 82            [24]  887 	mov	dpl,r7
      0002B4 12 00 B7         [24]  888 	lcall	_uart_sendata
                                    889 ;	ps2_basic.c:208: }
      0002B7                        890 00112$:
                                    891 ;	ps2_basic.c:210: delayms(200);
      0002B7 90 00 C8         [24]  892 	mov	dptr,#0x00c8
      0002BA 12 00 7D         [24]  893 	lcall	_delayms
                                    894 ;	ps2_basic.c:212: }
      0002BD 02 02 48         [24]  895 	ljmp	00114$
                                    896 	.area CSEG    (CODE)
                                    897 	.area CONST   (CODE)
                                    898 	.area XINIT   (CODE)
                                    899 	.area CABS    (ABS,CODE)
