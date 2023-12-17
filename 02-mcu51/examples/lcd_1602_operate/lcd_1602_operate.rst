                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module lcd_1602_operate
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _main
                                     12 	.globl _SetCurPosition
                                     13 	.globl _InitLcd
                                     14 	.globl _ShowString
                                     15 	.globl _ShowChar
                                     16 	.globl _WriteData
                                     17 	.globl _WriteCommand
                                     18 	.globl _DelayMs
                                     19 	.globl _DelayUs
                                     20 	.globl _sprintf
                                     21 	.globl _CY
                                     22 	.globl _AC
                                     23 	.globl _F0
                                     24 	.globl _RS1
                                     25 	.globl _RS0
                                     26 	.globl _OV
                                     27 	.globl _F1
                                     28 	.globl _P
                                     29 	.globl _PS
                                     30 	.globl _PT1
                                     31 	.globl _PX1
                                     32 	.globl _PT0
                                     33 	.globl _PX0
                                     34 	.globl _RD
                                     35 	.globl _WR
                                     36 	.globl _T1
                                     37 	.globl _T0
                                     38 	.globl _INT1
                                     39 	.globl _INT0
                                     40 	.globl _TXD
                                     41 	.globl _RXD
                                     42 	.globl _P3_7
                                     43 	.globl _P3_6
                                     44 	.globl _P3_5
                                     45 	.globl _P3_4
                                     46 	.globl _P3_3
                                     47 	.globl _P3_2
                                     48 	.globl _P3_1
                                     49 	.globl _P3_0
                                     50 	.globl _EA
                                     51 	.globl _ES
                                     52 	.globl _ET1
                                     53 	.globl _EX1
                                     54 	.globl _ET0
                                     55 	.globl _EX0
                                     56 	.globl _P2_7
                                     57 	.globl _P2_6
                                     58 	.globl _P2_5
                                     59 	.globl _P2_4
                                     60 	.globl _P2_3
                                     61 	.globl _P2_2
                                     62 	.globl _P2_1
                                     63 	.globl _P2_0
                                     64 	.globl _SM0
                                     65 	.globl _SM1
                                     66 	.globl _SM2
                                     67 	.globl _REN
                                     68 	.globl _TB8
                                     69 	.globl _RB8
                                     70 	.globl _TI
                                     71 	.globl _RI
                                     72 	.globl _P1_7
                                     73 	.globl _P1_6
                                     74 	.globl _P1_5
                                     75 	.globl _P1_4
                                     76 	.globl _P1_3
                                     77 	.globl _P1_2
                                     78 	.globl _P1_1
                                     79 	.globl _P1_0
                                     80 	.globl _TF1
                                     81 	.globl _TR1
                                     82 	.globl _TF0
                                     83 	.globl _TR0
                                     84 	.globl _IE1
                                     85 	.globl _IT1
                                     86 	.globl _IE0
                                     87 	.globl _IT0
                                     88 	.globl _P0_7
                                     89 	.globl _P0_6
                                     90 	.globl _P0_5
                                     91 	.globl _P0_4
                                     92 	.globl _P0_3
                                     93 	.globl _P0_2
                                     94 	.globl _P0_1
                                     95 	.globl _P0_0
                                     96 	.globl _B
                                     97 	.globl _ACC
                                     98 	.globl _PSW
                                     99 	.globl _IP
                                    100 	.globl _P3
                                    101 	.globl _IE
                                    102 	.globl _P2
                                    103 	.globl _SBUF
                                    104 	.globl _SCON
                                    105 	.globl _P1
                                    106 	.globl _TH1
                                    107 	.globl _TH0
                                    108 	.globl _TL1
                                    109 	.globl _TL0
                                    110 	.globl _TMOD
                                    111 	.globl _TCON
                                    112 	.globl _PCON
                                    113 	.globl _DPH
                                    114 	.globl _DPL
                                    115 	.globl _SP
                                    116 	.globl _P0
                                    117 	.globl _SetCurPosition_PARM_2
                                    118 	.globl _ShowString_PARM_2
                                    119 	.globl _ShowChar_PARM_2
                                    120 	.globl _Test1
                                    121 	.globl _TimeNum
                                    122 	.globl _count
                                    123 	.globl _second
                                    124 	.globl _minute
                                    125 	.globl _hour
                                    126 ;--------------------------------------------------------
                                    127 ; special function registers
                                    128 ;--------------------------------------------------------
                                    129 	.area RSEG    (ABS,DATA)
      000000                        130 	.org 0x0000
                           000080   131 _P0	=	0x0080
                           000081   132 _SP	=	0x0081
                           000082   133 _DPL	=	0x0082
                           000083   134 _DPH	=	0x0083
                           000087   135 _PCON	=	0x0087
                           000088   136 _TCON	=	0x0088
                           000089   137 _TMOD	=	0x0089
                           00008A   138 _TL0	=	0x008a
                           00008B   139 _TL1	=	0x008b
                           00008C   140 _TH0	=	0x008c
                           00008D   141 _TH1	=	0x008d
                           000090   142 _P1	=	0x0090
                           000098   143 _SCON	=	0x0098
                           000099   144 _SBUF	=	0x0099
                           0000A0   145 _P2	=	0x00a0
                           0000A8   146 _IE	=	0x00a8
                           0000B0   147 _P3	=	0x00b0
                           0000B8   148 _IP	=	0x00b8
                           0000D0   149 _PSW	=	0x00d0
                           0000E0   150 _ACC	=	0x00e0
                           0000F0   151 _B	=	0x00f0
                                    152 ;--------------------------------------------------------
                                    153 ; special function bits
                                    154 ;--------------------------------------------------------
                                    155 	.area RSEG    (ABS,DATA)
      000000                        156 	.org 0x0000
                           000080   157 _P0_0	=	0x0080
                           000081   158 _P0_1	=	0x0081
                           000082   159 _P0_2	=	0x0082
                           000083   160 _P0_3	=	0x0083
                           000084   161 _P0_4	=	0x0084
                           000085   162 _P0_5	=	0x0085
                           000086   163 _P0_6	=	0x0086
                           000087   164 _P0_7	=	0x0087
                           000088   165 _IT0	=	0x0088
                           000089   166 _IE0	=	0x0089
                           00008A   167 _IT1	=	0x008a
                           00008B   168 _IE1	=	0x008b
                           00008C   169 _TR0	=	0x008c
                           00008D   170 _TF0	=	0x008d
                           00008E   171 _TR1	=	0x008e
                           00008F   172 _TF1	=	0x008f
                           000090   173 _P1_0	=	0x0090
                           000091   174 _P1_1	=	0x0091
                           000092   175 _P1_2	=	0x0092
                           000093   176 _P1_3	=	0x0093
                           000094   177 _P1_4	=	0x0094
                           000095   178 _P1_5	=	0x0095
                           000096   179 _P1_6	=	0x0096
                           000097   180 _P1_7	=	0x0097
                           000098   181 _RI	=	0x0098
                           000099   182 _TI	=	0x0099
                           00009A   183 _RB8	=	0x009a
                           00009B   184 _TB8	=	0x009b
                           00009C   185 _REN	=	0x009c
                           00009D   186 _SM2	=	0x009d
                           00009E   187 _SM1	=	0x009e
                           00009F   188 _SM0	=	0x009f
                           0000A0   189 _P2_0	=	0x00a0
                           0000A1   190 _P2_1	=	0x00a1
                           0000A2   191 _P2_2	=	0x00a2
                           0000A3   192 _P2_3	=	0x00a3
                           0000A4   193 _P2_4	=	0x00a4
                           0000A5   194 _P2_5	=	0x00a5
                           0000A6   195 _P2_6	=	0x00a6
                           0000A7   196 _P2_7	=	0x00a7
                           0000A8   197 _EX0	=	0x00a8
                           0000A9   198 _ET0	=	0x00a9
                           0000AA   199 _EX1	=	0x00aa
                           0000AB   200 _ET1	=	0x00ab
                           0000AC   201 _ES	=	0x00ac
                           0000AF   202 _EA	=	0x00af
                           0000B0   203 _P3_0	=	0x00b0
                           0000B1   204 _P3_1	=	0x00b1
                           0000B2   205 _P3_2	=	0x00b2
                           0000B3   206 _P3_3	=	0x00b3
                           0000B4   207 _P3_4	=	0x00b4
                           0000B5   208 _P3_5	=	0x00b5
                           0000B6   209 _P3_6	=	0x00b6
                           0000B7   210 _P3_7	=	0x00b7
                           0000B0   211 _RXD	=	0x00b0
                           0000B1   212 _TXD	=	0x00b1
                           0000B2   213 _INT0	=	0x00b2
                           0000B3   214 _INT1	=	0x00b3
                           0000B4   215 _T0	=	0x00b4
                           0000B5   216 _T1	=	0x00b5
                           0000B6   217 _WR	=	0x00b6
                           0000B7   218 _RD	=	0x00b7
                           0000B8   219 _PX0	=	0x00b8
                           0000B9   220 _PT0	=	0x00b9
                           0000BA   221 _PX1	=	0x00ba
                           0000BB   222 _PT1	=	0x00bb
                           0000BC   223 _PS	=	0x00bc
                           0000D0   224 _P	=	0x00d0
                           0000D1   225 _F1	=	0x00d1
                           0000D2   226 _OV	=	0x00d2
                           0000D3   227 _RS0	=	0x00d3
                           0000D4   228 _RS1	=	0x00d4
                           0000D5   229 _F0	=	0x00d5
                           0000D6   230 _AC	=	0x00d6
                           0000D7   231 _CY	=	0x00d7
                                    232 ;--------------------------------------------------------
                                    233 ; overlayable register banks
                                    234 ;--------------------------------------------------------
                                    235 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        236 	.ds 8
                                    237 ;--------------------------------------------------------
                                    238 ; internal ram data
                                    239 ;--------------------------------------------------------
                                    240 	.area DSEG    (DATA)
      000008                        241 _hour::
      000008                        242 	.ds 2
      00000A                        243 _minute::
      00000A                        244 	.ds 2
      00000C                        245 _second::
      00000C                        246 	.ds 2
      00000E                        247 _count::
      00000E                        248 	.ds 2
      000010                        249 _TimeNum::
      000010                        250 	.ds 17
      000021                        251 _Test1::
      000021                        252 	.ds 19
      000034                        253 _ShowChar_PARM_2:
      000034                        254 	.ds 1
      000035                        255 _ShowString_PARM_2:
      000035                        256 	.ds 3
      000038                        257 _SetCurPosition_PARM_2:
      000038                        258 	.ds 1
                                    259 ;--------------------------------------------------------
                                    260 ; overlayable items in internal ram 
                                    261 ;--------------------------------------------------------
                                    262 	.area	OSEG    (OVR,DATA)
                                    263 ;--------------------------------------------------------
                                    264 ; Stack segment in internal ram 
                                    265 ;--------------------------------------------------------
                                    266 	.area	SSEG
      000072                        267 __start__stack:
      000072                        268 	.ds	1
                                    269 
                                    270 ;--------------------------------------------------------
                                    271 ; indirectly addressable internal ram data
                                    272 ;--------------------------------------------------------
                                    273 	.area ISEG    (DATA)
                                    274 ;--------------------------------------------------------
                                    275 ; absolute internal ram data
                                    276 ;--------------------------------------------------------
                                    277 	.area IABS    (ABS,DATA)
                                    278 	.area IABS    (ABS,DATA)
                                    279 ;--------------------------------------------------------
                                    280 ; bit data
                                    281 ;--------------------------------------------------------
                                    282 	.area BSEG    (BIT)
                                    283 ;--------------------------------------------------------
                                    284 ; paged external ram data
                                    285 ;--------------------------------------------------------
                                    286 	.area PSEG    (PAG,XDATA)
                                    287 ;--------------------------------------------------------
                                    288 ; external ram data
                                    289 ;--------------------------------------------------------
                                    290 	.area XSEG    (XDATA)
                                    291 ;--------------------------------------------------------
                                    292 ; absolute external ram data
                                    293 ;--------------------------------------------------------
                                    294 	.area XABS    (ABS,XDATA)
                                    295 ;--------------------------------------------------------
                                    296 ; external initialized ram data
                                    297 ;--------------------------------------------------------
                                    298 	.area XISEG   (XDATA)
                                    299 	.area HOME    (CODE)
                                    300 	.area GSINIT0 (CODE)
                                    301 	.area GSINIT1 (CODE)
                                    302 	.area GSINIT2 (CODE)
                                    303 	.area GSINIT3 (CODE)
                                    304 	.area GSINIT4 (CODE)
                                    305 	.area GSINIT5 (CODE)
                                    306 	.area GSINIT  (CODE)
                                    307 	.area GSFINAL (CODE)
                                    308 	.area CSEG    (CODE)
                                    309 ;--------------------------------------------------------
                                    310 ; interrupt vector 
                                    311 ;--------------------------------------------------------
                                    312 	.area HOME    (CODE)
      000000                        313 __interrupt_vect:
      000000 02 00 06         [24]  314 	ljmp	__sdcc_gsinit_startup
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
                                    328 ;	lcd_1602_operate.c:26: char __data TimeNum[] = "                ";
      00005F 75 10 20         [24]  329 	mov	_TimeNum,#0x20
      000062 75 11 20         [24]  330 	mov	(_TimeNum + 0x0001),#0x20
      000065 75 12 20         [24]  331 	mov	(_TimeNum + 0x0002),#0x20
      000068 75 13 20         [24]  332 	mov	(_TimeNum + 0x0003),#0x20
      00006B 75 14 20         [24]  333 	mov	(_TimeNum + 0x0004),#0x20
      00006E 75 15 20         [24]  334 	mov	(_TimeNum + 0x0005),#0x20
      000071 75 16 20         [24]  335 	mov	(_TimeNum + 0x0006),#0x20
      000074 75 17 20         [24]  336 	mov	(_TimeNum + 0x0007),#0x20
      000077 75 18 20         [24]  337 	mov	(_TimeNum + 0x0008),#0x20
      00007A 75 19 20         [24]  338 	mov	(_TimeNum + 0x0009),#0x20
      00007D 75 1A 20         [24]  339 	mov	(_TimeNum + 0x000a),#0x20
      000080 75 1B 20         [24]  340 	mov	(_TimeNum + 0x000b),#0x20
      000083 75 1C 20         [24]  341 	mov	(_TimeNum + 0x000c),#0x20
      000086 75 1D 20         [24]  342 	mov	(_TimeNum + 0x000d),#0x20
      000089 75 1E 20         [24]  343 	mov	(_TimeNum + 0x000e),#0x20
      00008C 75 1F 20         [24]  344 	mov	(_TimeNum + 0x000f),#0x20
      00008F 75 20 00         [24]  345 	mov	(_TimeNum + 0x0010),#0x00
                                    346 ;	lcd_1602_operate.c:27: char __data Test1[] = "                  ";
      000092 75 21 20         [24]  347 	mov	_Test1,#0x20
      000095 75 22 20         [24]  348 	mov	(_Test1 + 0x0001),#0x20
      000098 75 23 20         [24]  349 	mov	(_Test1 + 0x0002),#0x20
      00009B 75 24 20         [24]  350 	mov	(_Test1 + 0x0003),#0x20
      00009E 75 25 20         [24]  351 	mov	(_Test1 + 0x0004),#0x20
      0000A1 75 26 20         [24]  352 	mov	(_Test1 + 0x0005),#0x20
      0000A4 75 27 20         [24]  353 	mov	(_Test1 + 0x0006),#0x20
      0000A7 75 28 20         [24]  354 	mov	(_Test1 + 0x0007),#0x20
      0000AA 75 29 20         [24]  355 	mov	(_Test1 + 0x0008),#0x20
      0000AD 75 2A 20         [24]  356 	mov	(_Test1 + 0x0009),#0x20
      0000B0 75 2B 20         [24]  357 	mov	(_Test1 + 0x000a),#0x20
      0000B3 75 2C 20         [24]  358 	mov	(_Test1 + 0x000b),#0x20
      0000B6 75 2D 20         [24]  359 	mov	(_Test1 + 0x000c),#0x20
      0000B9 75 2E 20         [24]  360 	mov	(_Test1 + 0x000d),#0x20
      0000BC 75 2F 20         [24]  361 	mov	(_Test1 + 0x000e),#0x20
      0000BF 75 30 20         [24]  362 	mov	(_Test1 + 0x000f),#0x20
      0000C2 75 31 20         [24]  363 	mov	(_Test1 + 0x0010),#0x20
      0000C5 75 32 20         [24]  364 	mov	(_Test1 + 0x0011),#0x20
      0000C8 75 33 00         [24]  365 	mov	(_Test1 + 0x0012),#0x00
                                    366 	.area GSFINAL (CODE)
      0000CB 02 00 03         [24]  367 	ljmp	__sdcc_program_startup
                                    368 ;--------------------------------------------------------
                                    369 ; Home
                                    370 ;--------------------------------------------------------
                                    371 	.area HOME    (CODE)
                                    372 	.area HOME    (CODE)
      000003                        373 __sdcc_program_startup:
      000003 02 01 E0         [24]  374 	ljmp	_main
                                    375 ;	return from main will return to caller
                                    376 ;--------------------------------------------------------
                                    377 ; code
                                    378 ;--------------------------------------------------------
                                    379 	.area CSEG    (CODE)
                                    380 ;------------------------------------------------------------
                                    381 ;Allocation info for local variables in function 'DelayUs'
                                    382 ;------------------------------------------------------------
                                    383 ;us                        Allocated to registers r7 
                                    384 ;uscnt                     Allocated to registers r7 
                                    385 ;------------------------------------------------------------
                                    386 ;	lcd_1602_operate.c:31: void DelayUs(unsigned char us)                                       //delay us
                                    387 ;	-----------------------------------------
                                    388 ;	 function DelayUs
                                    389 ;	-----------------------------------------
      0000CE                        390 _DelayUs:
                           000007   391 	ar7 = 0x07
                           000006   392 	ar6 = 0x06
                           000005   393 	ar5 = 0x05
                           000004   394 	ar4 = 0x04
                           000003   395 	ar3 = 0x03
                           000002   396 	ar2 = 0x02
                           000001   397 	ar1 = 0x01
                           000000   398 	ar0 = 0x00
                                    399 ;	lcd_1602_operate.c:34: uscnt = us >> 1; /* Crystal frequency in 12MHz*/
      0000CE E5 82            [12]  400 	mov	a,dpl
      0000D0 C3               [12]  401 	clr	c
      0000D1 13               [12]  402 	rrc	a
      0000D2 FF               [12]  403 	mov	r7,a
                                    404 ;	lcd_1602_operate.c:35: while (--uscnt)
      0000D3                        405 00101$:
      0000D3 EF               [12]  406 	mov	a,r7
      0000D4 14               [12]  407 	dec	a
      0000D5 FF               [12]  408 	mov	r7,a
      0000D6 70 FB            [24]  409 	jnz	00101$
                                    410 ;	lcd_1602_operate.c:37: }
      0000D8 22               [24]  411 	ret
                                    412 ;------------------------------------------------------------
                                    413 ;Allocation info for local variables in function 'DelayMs'
                                    414 ;------------------------------------------------------------
                                    415 ;ms                        Allocated to registers r7 
                                    416 ;------------------------------------------------------------
                                    417 ;	lcd_1602_operate.c:39: void DelayMs(unsigned char ms) //delay Ms
                                    418 ;	-----------------------------------------
                                    419 ;	 function DelayMs
                                    420 ;	-----------------------------------------
      0000D9                        421 _DelayMs:
      0000D9 AF 82            [24]  422 	mov	r7,dpl
                                    423 ;	lcd_1602_operate.c:41: while (--ms)
      0000DB                        424 00101$:
      0000DB EF               [12]  425 	mov	a,r7
      0000DC 14               [12]  426 	dec	a
      0000DD FF               [12]  427 	mov	r7,a
      0000DE 60 1E            [24]  428 	jz	00104$
                                    429 ;	lcd_1602_operate.c:43: DelayUs(250);
      0000E0 75 82 FA         [24]  430 	mov	dpl,#0xfa
      0000E3 C0 07            [24]  431 	push	ar7
      0000E5 12 00 CE         [24]  432 	lcall	_DelayUs
                                    433 ;	lcd_1602_operate.c:44: DelayUs(250);
      0000E8 75 82 FA         [24]  434 	mov	dpl,#0xfa
      0000EB 12 00 CE         [24]  435 	lcall	_DelayUs
                                    436 ;	lcd_1602_operate.c:45: DelayUs(250);
      0000EE 75 82 FA         [24]  437 	mov	dpl,#0xfa
      0000F1 12 00 CE         [24]  438 	lcall	_DelayUs
                                    439 ;	lcd_1602_operate.c:46: DelayUs(250);
      0000F4 75 82 FA         [24]  440 	mov	dpl,#0xfa
      0000F7 12 00 CE         [24]  441 	lcall	_DelayUs
      0000FA D0 07            [24]  442 	pop	ar7
      0000FC 80 DD            [24]  443 	sjmp	00101$
      0000FE                        444 00104$:
                                    445 ;	lcd_1602_operate.c:48: }
      0000FE 22               [24]  446 	ret
                                    447 ;------------------------------------------------------------
                                    448 ;Allocation info for local variables in function 'WriteCommand'
                                    449 ;------------------------------------------------------------
                                    450 ;c                         Allocated to registers r7 
                                    451 ;------------------------------------------------------------
                                    452 ;	lcd_1602_operate.c:49: void WriteCommand(unsigned char c)
                                    453 ;	-----------------------------------------
                                    454 ;	 function WriteCommand
                                    455 ;	-----------------------------------------
      0000FF                        456 _WriteCommand:
      0000FF AF 82            [24]  457 	mov	r7,dpl
                                    458 ;	lcd_1602_operate.c:51: DelayMs(5); //short delay before operation
      000101 75 82 05         [24]  459 	mov	dpl,#0x05
      000104 C0 07            [24]  460 	push	ar7
      000106 12 00 D9         [24]  461 	lcall	_DelayMs
      000109 D0 07            [24]  462 	pop	ar7
                                    463 ;	lcd_1602_operate.c:52: E = 0;
                                    464 ;	assignBit
      00010B C2 90            [12]  465 	clr	_P1_0
                                    466 ;	lcd_1602_operate.c:53: RS = 0;
                                    467 ;	assignBit
      00010D C2 92            [12]  468 	clr	_P1_2
                                    469 ;	lcd_1602_operate.c:54: RW = 0;
                                    470 ;	assignBit
      00010F C2 91            [12]  471 	clr	_P1_1
                                    472 ;	lcd_1602_operate.c:55: _nop_();
      000111 00               [12]  473 	NOP	
                                    474 ;	lcd_1602_operate.c:56: E = 1;
                                    475 ;	assignBit
      000112 D2 90            [12]  476 	setb	_P1_0
                                    477 ;	lcd_1602_operate.c:57: Data = c;
      000114 8F A0            [24]  478 	mov	_P2,r7
                                    479 ;	lcd_1602_operate.c:58: E = 0;
                                    480 ;	assignBit
      000116 C2 90            [12]  481 	clr	_P1_0
                                    482 ;	lcd_1602_operate.c:59: }
      000118 22               [24]  483 	ret
                                    484 ;------------------------------------------------------------
                                    485 ;Allocation info for local variables in function 'WriteData'
                                    486 ;------------------------------------------------------------
                                    487 ;c                         Allocated to registers r7 
                                    488 ;------------------------------------------------------------
                                    489 ;	lcd_1602_operate.c:61: void WriteData(unsigned char c)
                                    490 ;	-----------------------------------------
                                    491 ;	 function WriteData
                                    492 ;	-----------------------------------------
      000119                        493 _WriteData:
      000119 AF 82            [24]  494 	mov	r7,dpl
                                    495 ;	lcd_1602_operate.c:63: DelayMs(5); //short delay before operation
      00011B 75 82 05         [24]  496 	mov	dpl,#0x05
      00011E C0 07            [24]  497 	push	ar7
      000120 12 00 D9         [24]  498 	lcall	_DelayMs
      000123 D0 07            [24]  499 	pop	ar7
                                    500 ;	lcd_1602_operate.c:64: E = 0;
                                    501 ;	assignBit
      000125 C2 90            [12]  502 	clr	_P1_0
                                    503 ;	lcd_1602_operate.c:65: RS = 1;
                                    504 ;	assignBit
      000127 D2 92            [12]  505 	setb	_P1_2
                                    506 ;	lcd_1602_operate.c:66: RW = 0;
                                    507 ;	assignBit
      000129 C2 91            [12]  508 	clr	_P1_1
                                    509 ;	lcd_1602_operate.c:67: _nop_();
      00012B 00               [12]  510 	NOP	
                                    511 ;	lcd_1602_operate.c:68: E = 1;
                                    512 ;	assignBit
      00012C D2 90            [12]  513 	setb	_P1_0
                                    514 ;	lcd_1602_operate.c:69: Data = c;
      00012E 8F A0            [24]  515 	mov	_P2,r7
                                    516 ;	lcd_1602_operate.c:70: E = 0;
                                    517 ;	assignBit
      000130 C2 90            [12]  518 	clr	_P1_0
                                    519 ;	lcd_1602_operate.c:71: RS = 0;
                                    520 ;	assignBit
      000132 C2 92            [12]  521 	clr	_P1_2
                                    522 ;	lcd_1602_operate.c:72: }
      000134 22               [24]  523 	ret
                                    524 ;------------------------------------------------------------
                                    525 ;Allocation info for local variables in function 'ShowChar'
                                    526 ;------------------------------------------------------------
                                    527 ;c                         Allocated with name '_ShowChar_PARM_2'
                                    528 ;pos                       Allocated to registers r7 
                                    529 ;p                         Allocated to registers r6 
                                    530 ;------------------------------------------------------------
                                    531 ;	lcd_1602_operate.c:74: void ShowChar(unsigned char pos, unsigned char c)
                                    532 ;	-----------------------------------------
                                    533 ;	 function ShowChar
                                    534 ;	-----------------------------------------
      000135                        535 _ShowChar:
      000135 AF 82            [24]  536 	mov	r7,dpl
                                    537 ;	lcd_1602_operate.c:77: if (pos >= 0x10)
      000137 BF 10 00         [24]  538 	cjne	r7,#0x10,00110$
      00013A                        539 00110$:
      00013A 40 08            [24]  540 	jc	00102$
                                    541 ;	lcd_1602_operate.c:78: p = pos + 0xb0; //是第二行则命令代码高4位为0xc
      00013C 8F 06            [24]  542 	mov	ar6,r7
      00013E 74 B0            [12]  543 	mov	a,#0xb0
      000140 2E               [12]  544 	add	a,r6
      000141 FE               [12]  545 	mov	r6,a
      000142 80 04            [24]  546 	sjmp	00103$
      000144                        547 00102$:
                                    548 ;	lcd_1602_operate.c:80: p = pos + 0x80; //是第二行则命令代码高4位为0x8
      000144 74 80            [12]  549 	mov	a,#0x80
      000146 2F               [12]  550 	add	a,r7
      000147 FE               [12]  551 	mov	r6,a
      000148                        552 00103$:
                                    553 ;	lcd_1602_operate.c:81: WriteCommand(p);    //write command
      000148 8E 82            [24]  554 	mov	dpl,r6
      00014A 12 00 FF         [24]  555 	lcall	_WriteCommand
                                    556 ;	lcd_1602_operate.c:82: WriteData(c);       //write data
      00014D 85 34 82         [24]  557 	mov	dpl,_ShowChar_PARM_2
                                    558 ;	lcd_1602_operate.c:83: }
      000150 02 01 19         [24]  559 	ljmp	_WriteData
                                    560 ;------------------------------------------------------------
                                    561 ;Allocation info for local variables in function 'ShowString'
                                    562 ;------------------------------------------------------------
                                    563 ;ptr                       Allocated with name '_ShowString_PARM_2'
                                    564 ;line                      Allocated to registers r7 
                                    565 ;l                         Allocated to registers 
                                    566 ;i                         Allocated to registers r6 
                                    567 ;------------------------------------------------------------
                                    568 ;	lcd_1602_operate.c:85: void ShowString(unsigned char line, char *ptr)
                                    569 ;	-----------------------------------------
                                    570 ;	 function ShowString
                                    571 ;	-----------------------------------------
      000153                        572 _ShowString:
                                    573 ;	lcd_1602_operate.c:88: l = line << 4;
      000153 E5 82            [12]  574 	mov	a,dpl
      000155 C4               [12]  575 	swap	a
      000156 54 F0            [12]  576 	anl	a,#0xf0
      000158 FF               [12]  577 	mov	r7,a
                                    578 ;	lcd_1602_operate.c:89: for (i = 0; i < 16; i++)
      000159 7E 00            [12]  579 	mov	r6,#0x00
      00015B                        580 00102$:
                                    581 ;	lcd_1602_operate.c:90: ShowChar(l++, *(ptr + i)); //循环显示16个字符
      00015B 8F 05            [24]  582 	mov	ar5,r7
      00015D 0F               [12]  583 	inc	r7
      00015E EE               [12]  584 	mov	a,r6
      00015F 25 35            [12]  585 	add	a,_ShowString_PARM_2
      000161 FA               [12]  586 	mov	r2,a
      000162 E4               [12]  587 	clr	a
      000163 35 36            [12]  588 	addc	a,(_ShowString_PARM_2 + 1)
      000165 FB               [12]  589 	mov	r3,a
      000166 AC 37            [24]  590 	mov	r4,(_ShowString_PARM_2 + 2)
      000168 8A 82            [24]  591 	mov	dpl,r2
      00016A 8B 83            [24]  592 	mov	dph,r3
      00016C 8C F0            [24]  593 	mov	b,r4
      00016E 12 0A C8         [24]  594 	lcall	__gptrget
      000171 F5 34            [12]  595 	mov	_ShowChar_PARM_2,a
      000173 8D 82            [24]  596 	mov	dpl,r5
      000175 C0 07            [24]  597 	push	ar7
      000177 C0 06            [24]  598 	push	ar6
      000179 12 01 35         [24]  599 	lcall	_ShowChar
      00017C D0 06            [24]  600 	pop	ar6
      00017E D0 07            [24]  601 	pop	ar7
                                    602 ;	lcd_1602_operate.c:89: for (i = 0; i < 16; i++)
      000180 0E               [12]  603 	inc	r6
      000181 BE 10 00         [24]  604 	cjne	r6,#0x10,00111$
      000184                        605 00111$:
      000184 40 D5            [24]  606 	jc	00102$
                                    607 ;	lcd_1602_operate.c:91: }
      000186 22               [24]  608 	ret
                                    609 ;------------------------------------------------------------
                                    610 ;Allocation info for local variables in function 'InitLcd'
                                    611 ;------------------------------------------------------------
                                    612 ;	lcd_1602_operate.c:93: void InitLcd()
                                    613 ;	-----------------------------------------
                                    614 ;	 function InitLcd
                                    615 ;	-----------------------------------------
      000187                        616 _InitLcd:
                                    617 ;	lcd_1602_operate.c:95: DelayMs(15);
      000187 75 82 0F         [24]  618 	mov	dpl,#0x0f
      00018A 12 00 D9         [24]  619 	lcall	_DelayMs
                                    620 ;	lcd_1602_operate.c:96: WriteCommand(0x38); //display mode
      00018D 75 82 38         [24]  621 	mov	dpl,#0x38
      000190 12 00 FF         [24]  622 	lcall	_WriteCommand
                                    623 ;	lcd_1602_operate.c:97: WriteCommand(0x38); //display mode
      000193 75 82 38         [24]  624 	mov	dpl,#0x38
      000196 12 00 FF         [24]  625 	lcall	_WriteCommand
                                    626 ;	lcd_1602_operate.c:98: WriteCommand(0x38); //display mode
      000199 75 82 38         [24]  627 	mov	dpl,#0x38
      00019C 12 00 FF         [24]  628 	lcall	_WriteCommand
                                    629 ;	lcd_1602_operate.c:99: WriteCommand(0x08); //显示光标移动位置
      00019F 75 82 08         [24]  630 	mov	dpl,#0x08
      0001A2 12 00 FF         [24]  631 	lcall	_WriteCommand
                                    632 ;	lcd_1602_operate.c:100: WriteCommand(0x0c); //显示开及光标设置
      0001A5 75 82 0C         [24]  633 	mov	dpl,#0x0c
      0001A8 12 00 FF         [24]  634 	lcall	_WriteCommand
                                    635 ;	lcd_1602_operate.c:101: WriteCommand(0x01); //显示清屏
      0001AB 75 82 01         [24]  636 	mov	dpl,#0x01
      0001AE 12 00 FF         [24]  637 	lcall	_WriteCommand
                                    638 ;	lcd_1602_operate.c:102: WriteCommand(0x04); //
      0001B1 75 82 04         [24]  639 	mov	dpl,#0x04
      0001B4 12 00 FF         [24]  640 	lcall	_WriteCommand
                                    641 ;	lcd_1602_operate.c:103: WriteCommand(0x0c); //
      0001B7 75 82 0C         [24]  642 	mov	dpl,#0x0c
                                    643 ;	lcd_1602_operate.c:104: }
      0001BA 02 00 FF         [24]  644 	ljmp	_WriteCommand
                                    645 ;------------------------------------------------------------
                                    646 ;Allocation info for local variables in function 'SetCurPosition'
                                    647 ;------------------------------------------------------------
                                    648 ;Y                         Allocated with name '_SetCurPosition_PARM_2'
                                    649 ;X                         Allocated to registers r7 
                                    650 ;------------------------------------------------------------
                                    651 ;	lcd_1602_operate.c:105: void SetCurPosition(unsigned char X, unsigned char Y)
                                    652 ;	-----------------------------------------
                                    653 ;	 function SetCurPosition
                                    654 ;	-----------------------------------------
      0001BD                        655 _SetCurPosition:
      0001BD AF 82            [24]  656 	mov	r7,dpl
                                    657 ;	lcd_1602_operate.c:107: Y &= 0x01;
      0001BF 53 38 01         [24]  658 	anl	_SetCurPosition_PARM_2,#0x01
                                    659 ;	lcd_1602_operate.c:108: X &= 0x0F; // 限制X不能大于15，Y不能大于1
      0001C2 53 07 0F         [24]  660 	anl	ar7,#0x0f
                                    661 ;	lcd_1602_operate.c:109: if (Y)
      0001C5 E5 38            [12]  662 	mov	a,_SetCurPosition_PARM_2
      0001C7 60 09            [24]  663 	jz	00102$
                                    664 ;	lcd_1602_operate.c:111: X |= 0xc0; // 当要显示第二行时地址码:0xc0	 0x80 + 0x40
      0001C9 8F 05            [24]  665 	mov	ar5,r7
      0001CB 7E 00            [12]  666 	mov	r6,#0x00
      0001CD 43 05 C0         [24]  667 	orl	ar5,#0xc0
      0001D0 8D 07            [24]  668 	mov	ar7,r5
      0001D2                        669 00102$:
                                    670 ;	lcd_1602_operate.c:113: X |= 0x80;          // 第一行的地址码:0x80
      0001D2 43 07 80         [24]  671 	orl	ar7,#0x80
      0001D5 8F 82            [24]  672 	mov	dpl,r7
                                    673 ;	lcd_1602_operate.c:114: WriteCommand(X);    //
      0001D7 12 00 FF         [24]  674 	lcall	_WriteCommand
                                    675 ;	lcd_1602_operate.c:115: WriteCommand(0X0f); //开光标，闪烁
      0001DA 75 82 0F         [24]  676 	mov	dpl,#0x0f
                                    677 ;	lcd_1602_operate.c:116: }
      0001DD 02 00 FF         [24]  678 	ljmp	_WriteCommand
                                    679 ;------------------------------------------------------------
                                    680 ;Allocation info for local variables in function 'main'
                                    681 ;------------------------------------------------------------
                                    682 ;	lcd_1602_operate.c:119: void main(void)
                                    683 ;	-----------------------------------------
                                    684 ;	 function main
                                    685 ;	-----------------------------------------
      0001E0                        686 _main:
                                    687 ;	lcd_1602_operate.c:122: InitLcd(); //
      0001E0 12 01 87         [24]  688 	lcall	_InitLcd
                                    689 ;	lcd_1602_operate.c:123: DelayMs(15);
      0001E3 75 82 0F         [24]  690 	mov	dpl,#0x0f
      0001E6 12 00 D9         [24]  691 	lcall	_DelayMs
                                    692 ;	lcd_1602_operate.c:124: sprintf(Test1, " gogo yoyo      "); //the first line
      0001E9 74 03            [12]  693 	mov	a,#___str_0
      0001EB C0 E0            [24]  694 	push	acc
      0001ED 74 0B            [12]  695 	mov	a,#(___str_0 >> 8)
      0001EF C0 E0            [24]  696 	push	acc
      0001F1 74 80            [12]  697 	mov	a,#0x80
      0001F3 C0 E0            [24]  698 	push	acc
      0001F5 74 21            [12]  699 	mov	a,#_Test1
      0001F7 C0 E0            [24]  700 	push	acc
      0001F9 74 00            [12]  701 	mov	a,#(_Test1 >> 8)
      0001FB C0 E0            [24]  702 	push	acc
      0001FD 74 40            [12]  703 	mov	a,#0x40
      0001FF C0 E0            [24]  704 	push	acc
      000201 12 03 01         [24]  705 	lcall	_sprintf
      000204 E5 81            [12]  706 	mov	a,sp
      000206 24 FA            [12]  707 	add	a,#0xfa
      000208 F5 81            [12]  708 	mov	sp,a
                                    709 ;	lcd_1602_operate.c:125: ShowString(0, Test1);
      00020A 75 35 21         [24]  710 	mov	_ShowString_PARM_2,#_Test1
      00020D 75 36 00         [24]  711 	mov	(_ShowString_PARM_2 + 1),#0x00
      000210 75 37 40         [24]  712 	mov	(_ShowString_PARM_2 + 2),#0x40
      000213 75 82 00         [24]  713 	mov	dpl,#0x00
      000216 12 01 53         [24]  714 	lcall	_ShowString
                                    715 ;	lcd_1602_operate.c:126: sprintf(TimeNum, "zhangsai is good boy"); //the first line
      000219 74 14            [12]  716 	mov	a,#___str_1
      00021B C0 E0            [24]  717 	push	acc
      00021D 74 0B            [12]  718 	mov	a,#(___str_1 >> 8)
      00021F C0 E0            [24]  719 	push	acc
      000221 74 80            [12]  720 	mov	a,#0x80
      000223 C0 E0            [24]  721 	push	acc
      000225 74 10            [12]  722 	mov	a,#_TimeNum
      000227 C0 E0            [24]  723 	push	acc
      000229 74 00            [12]  724 	mov	a,#(_TimeNum >> 8)
      00022B C0 E0            [24]  725 	push	acc
      00022D 74 40            [12]  726 	mov	a,#0x40
      00022F C0 E0            [24]  727 	push	acc
      000231 12 03 01         [24]  728 	lcall	_sprintf
      000234 E5 81            [12]  729 	mov	a,sp
      000236 24 FA            [12]  730 	add	a,#0xfa
      000238 F5 81            [12]  731 	mov	sp,a
                                    732 ;	lcd_1602_operate.c:127: ShowString(1, TimeNum);
      00023A 75 35 10         [24]  733 	mov	_ShowString_PARM_2,#_TimeNum
      00023D 75 36 00         [24]  734 	mov	(_ShowString_PARM_2 + 1),#0x00
      000240 75 37 40         [24]  735 	mov	(_ShowString_PARM_2 + 2),#0x40
      000243 75 82 01         [24]  736 	mov	dpl,#0x01
      000246 12 01 53         [24]  737 	lcall	_ShowString
                                    738 ;	lcd_1602_operate.c:128: SetCurPosition(0, 0); //第一行第一位置
      000249 75 38 00         [24]  739 	mov	_SetCurPosition_PARM_2,#0x00
      00024C 75 82 00         [24]  740 	mov	dpl,#0x00
      00024F 12 01 BD         [24]  741 	lcall	_SetCurPosition
                                    742 ;	lcd_1602_operate.c:130: while (1)
      000252                        743 00102$:
                                    744 ;	lcd_1602_operate.c:132: }
      000252 80 FE            [24]  745 	sjmp	00102$
                                    746 	.area CSEG    (CODE)
                                    747 	.area CONST   (CODE)
      000B03                        748 ___str_0:
      000B03 20 67 6F 67 6F 20 79   749 	.ascii " gogo yoyo      "
             6F 79 6F 20 20 20 20
             20 20
      000B13 00                     750 	.db 0x00
      000B14                        751 ___str_1:
      000B14 7A 68 61 6E 67 73 61   752 	.ascii "zhangsai is good boy"
             69 20 69 73 20 67 6F
             6F 64 20 62 6F 79
      000B28 00                     753 	.db 0x00
                                    754 	.area XINIT   (CODE)
                                    755 	.area CABS    (ABS,CODE)
