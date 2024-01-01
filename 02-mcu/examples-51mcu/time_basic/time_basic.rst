                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module time_basic
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _time0_exe
                                     12 	.globl _main
                                     13 	.globl _time0_init
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
                                    110 	.globl _loop_times
                                    111 ;--------------------------------------------------------
                                    112 ; special function registers
                                    113 ;--------------------------------------------------------
                                    114 	.area RSEG    (ABS,DATA)
      000000                        115 	.org 0x0000
                           000080   116 _P0	=	0x0080
                           000081   117 _SP	=	0x0081
                           000082   118 _DPL	=	0x0082
                           000083   119 _DPH	=	0x0083
                           000087   120 _PCON	=	0x0087
                           000088   121 _TCON	=	0x0088
                           000089   122 _TMOD	=	0x0089
                           00008A   123 _TL0	=	0x008a
                           00008B   124 _TL1	=	0x008b
                           00008C   125 _TH0	=	0x008c
                           00008D   126 _TH1	=	0x008d
                           000090   127 _P1	=	0x0090
                           000098   128 _SCON	=	0x0098
                           000099   129 _SBUF	=	0x0099
                           0000A0   130 _P2	=	0x00a0
                           0000A8   131 _IE	=	0x00a8
                           0000B0   132 _P3	=	0x00b0
                           0000B8   133 _IP	=	0x00b8
                           0000D0   134 _PSW	=	0x00d0
                           0000E0   135 _ACC	=	0x00e0
                           0000F0   136 _B	=	0x00f0
                                    137 ;--------------------------------------------------------
                                    138 ; special function bits
                                    139 ;--------------------------------------------------------
                                    140 	.area RSEG    (ABS,DATA)
      000000                        141 	.org 0x0000
                           000080   142 _P0_0	=	0x0080
                           000081   143 _P0_1	=	0x0081
                           000082   144 _P0_2	=	0x0082
                           000083   145 _P0_3	=	0x0083
                           000084   146 _P0_4	=	0x0084
                           000085   147 _P0_5	=	0x0085
                           000086   148 _P0_6	=	0x0086
                           000087   149 _P0_7	=	0x0087
                           000088   150 _IT0	=	0x0088
                           000089   151 _IE0	=	0x0089
                           00008A   152 _IT1	=	0x008a
                           00008B   153 _IE1	=	0x008b
                           00008C   154 _TR0	=	0x008c
                           00008D   155 _TF0	=	0x008d
                           00008E   156 _TR1	=	0x008e
                           00008F   157 _TF1	=	0x008f
                           000090   158 _P1_0	=	0x0090
                           000091   159 _P1_1	=	0x0091
                           000092   160 _P1_2	=	0x0092
                           000093   161 _P1_3	=	0x0093
                           000094   162 _P1_4	=	0x0094
                           000095   163 _P1_5	=	0x0095
                           000096   164 _P1_6	=	0x0096
                           000097   165 _P1_7	=	0x0097
                           000098   166 _RI	=	0x0098
                           000099   167 _TI	=	0x0099
                           00009A   168 _RB8	=	0x009a
                           00009B   169 _TB8	=	0x009b
                           00009C   170 _REN	=	0x009c
                           00009D   171 _SM2	=	0x009d
                           00009E   172 _SM1	=	0x009e
                           00009F   173 _SM0	=	0x009f
                           0000A0   174 _P2_0	=	0x00a0
                           0000A1   175 _P2_1	=	0x00a1
                           0000A2   176 _P2_2	=	0x00a2
                           0000A3   177 _P2_3	=	0x00a3
                           0000A4   178 _P2_4	=	0x00a4
                           0000A5   179 _P2_5	=	0x00a5
                           0000A6   180 _P2_6	=	0x00a6
                           0000A7   181 _P2_7	=	0x00a7
                           0000A8   182 _EX0	=	0x00a8
                           0000A9   183 _ET0	=	0x00a9
                           0000AA   184 _EX1	=	0x00aa
                           0000AB   185 _ET1	=	0x00ab
                           0000AC   186 _ES	=	0x00ac
                           0000AF   187 _EA	=	0x00af
                           0000B0   188 _P3_0	=	0x00b0
                           0000B1   189 _P3_1	=	0x00b1
                           0000B2   190 _P3_2	=	0x00b2
                           0000B3   191 _P3_3	=	0x00b3
                           0000B4   192 _P3_4	=	0x00b4
                           0000B5   193 _P3_5	=	0x00b5
                           0000B6   194 _P3_6	=	0x00b6
                           0000B7   195 _P3_7	=	0x00b7
                           0000B0   196 _RXD	=	0x00b0
                           0000B1   197 _TXD	=	0x00b1
                           0000B2   198 _INT0	=	0x00b2
                           0000B3   199 _INT1	=	0x00b3
                           0000B4   200 _T0	=	0x00b4
                           0000B5   201 _T1	=	0x00b5
                           0000B6   202 _WR	=	0x00b6
                           0000B7   203 _RD	=	0x00b7
                           0000B8   204 _PX0	=	0x00b8
                           0000B9   205 _PT0	=	0x00b9
                           0000BA   206 _PX1	=	0x00ba
                           0000BB   207 _PT1	=	0x00bb
                           0000BC   208 _PS	=	0x00bc
                           0000D0   209 _P	=	0x00d0
                           0000D1   210 _F1	=	0x00d1
                           0000D2   211 _OV	=	0x00d2
                           0000D3   212 _RS0	=	0x00d3
                           0000D4   213 _RS1	=	0x00d4
                           0000D5   214 _F0	=	0x00d5
                           0000D6   215 _AC	=	0x00d6
                           0000D7   216 _CY	=	0x00d7
                                    217 ;--------------------------------------------------------
                                    218 ; overlayable register banks
                                    219 ;--------------------------------------------------------
                                    220 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        221 	.ds 8
                                    222 ;--------------------------------------------------------
                                    223 ; internal ram data
                                    224 ;--------------------------------------------------------
                                    225 	.area DSEG    (DATA)
      000008                        226 _loop_times::
      000008                        227 	.ds 2
                                    228 ;--------------------------------------------------------
                                    229 ; overlayable items in internal ram 
                                    230 ;--------------------------------------------------------
                                    231 ;--------------------------------------------------------
                                    232 ; Stack segment in internal ram 
                                    233 ;--------------------------------------------------------
                                    234 	.area	SSEG
      00000A                        235 __start__stack:
      00000A                        236 	.ds	1
                                    237 
                                    238 ;--------------------------------------------------------
                                    239 ; indirectly addressable internal ram data
                                    240 ;--------------------------------------------------------
                                    241 	.area ISEG    (DATA)
                                    242 ;--------------------------------------------------------
                                    243 ; absolute internal ram data
                                    244 ;--------------------------------------------------------
                                    245 	.area IABS    (ABS,DATA)
                                    246 	.area IABS    (ABS,DATA)
                                    247 ;--------------------------------------------------------
                                    248 ; bit data
                                    249 ;--------------------------------------------------------
                                    250 	.area BSEG    (BIT)
                                    251 ;--------------------------------------------------------
                                    252 ; paged external ram data
                                    253 ;--------------------------------------------------------
                                    254 	.area PSEG    (PAG,XDATA)
                                    255 ;--------------------------------------------------------
                                    256 ; external ram data
                                    257 ;--------------------------------------------------------
                                    258 	.area XSEG    (XDATA)
                                    259 ;--------------------------------------------------------
                                    260 ; absolute external ram data
                                    261 ;--------------------------------------------------------
                                    262 	.area XABS    (ABS,XDATA)
                                    263 ;--------------------------------------------------------
                                    264 ; external initialized ram data
                                    265 ;--------------------------------------------------------
                                    266 	.area XISEG   (XDATA)
                                    267 	.area HOME    (CODE)
                                    268 	.area GSINIT0 (CODE)
                                    269 	.area GSINIT1 (CODE)
                                    270 	.area GSINIT2 (CODE)
                                    271 	.area GSINIT3 (CODE)
                                    272 	.area GSINIT4 (CODE)
                                    273 	.area GSINIT5 (CODE)
                                    274 	.area GSINIT  (CODE)
                                    275 	.area GSFINAL (CODE)
                                    276 	.area CSEG    (CODE)
                                    277 ;--------------------------------------------------------
                                    278 ; interrupt vector 
                                    279 ;--------------------------------------------------------
                                    280 	.area HOME    (CODE)
      000000                        281 __interrupt_vect:
      000000 02 00 11         [24]  282 	ljmp	__sdcc_gsinit_startup
      000003 32               [24]  283 	reti
      000004                        284 	.ds	7
      00000B 02 00 8B         [24]  285 	ljmp	_time0_exe
                                    286 ;--------------------------------------------------------
                                    287 ; global & static initialisations
                                    288 ;--------------------------------------------------------
                                    289 	.area HOME    (CODE)
                                    290 	.area GSINIT  (CODE)
                                    291 	.area GSFINAL (CODE)
                                    292 	.area GSINIT  (CODE)
                                    293 	.globl __sdcc_gsinit_startup
                                    294 	.globl __sdcc_program_startup
                                    295 	.globl __start__stack
                                    296 	.globl __mcs51_genXINIT
                                    297 	.globl __mcs51_genXRAMCLEAR
                                    298 	.globl __mcs51_genRAMCLEAR
                                    299 ;	time_basic.c:8: unsigned int loop_times = 0;
      00006A E4               [12]  300 	clr	a
      00006B F5 08            [12]  301 	mov	_loop_times,a
      00006D F5 09            [12]  302 	mov	(_loop_times + 1),a
                                    303 	.area GSFINAL (CODE)
      00006F 02 00 0E         [24]  304 	ljmp	__sdcc_program_startup
                                    305 ;--------------------------------------------------------
                                    306 ; Home
                                    307 ;--------------------------------------------------------
                                    308 	.area HOME    (CODE)
                                    309 	.area HOME    (CODE)
      00000E                        310 __sdcc_program_startup:
      00000E 02 00 86         [24]  311 	ljmp	_main
                                    312 ;	return from main will return to caller
                                    313 ;--------------------------------------------------------
                                    314 ; code
                                    315 ;--------------------------------------------------------
                                    316 	.area CSEG    (CODE)
                                    317 ;------------------------------------------------------------
                                    318 ;Allocation info for local variables in function 'time0_init'
                                    319 ;------------------------------------------------------------
                                    320 ;	time_basic.c:10: void time0_init(void) {
                                    321 ;	-----------------------------------------
                                    322 ;	 function time0_init
                                    323 ;	-----------------------------------------
      000072                        324 _time0_init:
                           000007   325 	ar7 = 0x07
                           000006   326 	ar6 = 0x06
                           000005   327 	ar5 = 0x05
                           000004   328 	ar4 = 0x04
                           000003   329 	ar3 = 0x03
                           000002   330 	ar2 = 0x02
                           000001   331 	ar1 = 0x01
                           000000   332 	ar0 = 0x00
                                    333 ;	time_basic.c:11: EA = 1;
                                    334 ;	assignBit
      000072 D2 AF            [12]  335 	setb	_EA
                                    336 ;	time_basic.c:12: ET0 = 1;
                                    337 ;	assignBit
      000074 D2 A9            [12]  338 	setb	_ET0
                                    339 ;	time_basic.c:13: TR0 = 1;
                                    340 ;	assignBit
      000076 D2 8C            [12]  341 	setb	_TR0
                                    342 ;	time_basic.c:14: TMOD |= 0X01;
      000078 AE 89            [24]  343 	mov	r6,_TMOD
      00007A 43 06 01         [24]  344 	orl	ar6,#0x01
      00007D 8E 89            [24]  345 	mov	_TMOD,r6
                                    346 ;	time_basic.c:15: TH0 = 0X4B;
      00007F 75 8C 4B         [24]  347 	mov	_TH0,#0x4b
                                    348 ;	time_basic.c:16: TL0 = 0XEC; // 11.0592晶振 FC66是 1ms值。  12m晶振是0xfc18
      000082 75 8A EC         [24]  349 	mov	_TL0,#0xec
                                    350 ;	time_basic.c:17: }
      000085 22               [24]  351 	ret
                                    352 ;------------------------------------------------------------
                                    353 ;Allocation info for local variables in function 'main'
                                    354 ;------------------------------------------------------------
                                    355 ;	time_basic.c:19: void main(void)
                                    356 ;	-----------------------------------------
                                    357 ;	 function main
                                    358 ;	-----------------------------------------
      000086                        359 _main:
                                    360 ;	time_basic.c:21: time0_init();
      000086 12 00 72         [24]  361 	lcall	_time0_init
                                    362 ;	time_basic.c:23: while(1)
      000089                        363 00102$:
                                    364 ;	time_basic.c:30: }
      000089 80 FE            [24]  365 	sjmp	00102$
                                    366 ;------------------------------------------------------------
                                    367 ;Allocation info for local variables in function 'time0_exe'
                                    368 ;------------------------------------------------------------
                                    369 ;	time_basic.c:34: void time0_exe(void) __interrupt 1 //R0 R1 =  0/1   0/1
                                    370 ;	-----------------------------------------
                                    371 ;	 function time0_exe
                                    372 ;	-----------------------------------------
      00008B                        373 _time0_exe:
      00008B C0 E0            [24]  374 	push	acc
      00008D C0 D0            [24]  375 	push	psw
                                    376 ;	time_basic.c:36: loop_times++;
      00008F 05 08            [12]  377 	inc	_loop_times
      000091 E4               [12]  378 	clr	a
      000092 B5 08 02         [24]  379 	cjne	a,_loop_times,00109$
      000095 05 09            [12]  380 	inc	(_loop_times + 1)
      000097                        381 00109$:
                                    382 ;	time_basic.c:37: TH0 = 0X4B;
      000097 75 8C 4B         [24]  383 	mov	_TH0,#0x4b
                                    384 ;	time_basic.c:38: TL0 = 0XEC;
      00009A 75 8A EC         [24]  385 	mov	_TL0,#0xec
                                    386 ;	time_basic.c:40: if (loop_times >= 20) {
      00009D C3               [12]  387 	clr	c
      00009E E5 08            [12]  388 	mov	a,_loop_times
      0000A0 94 14            [12]  389 	subb	a,#0x14
      0000A2 E5 09            [12]  390 	mov	a,(_loop_times + 1)
      0000A4 94 00            [12]  391 	subb	a,#0x00
      0000A6 40 07            [24]  392 	jc	00103$
                                    393 ;	time_basic.c:41: LED = !LED;
      0000A8 B2 80            [12]  394 	cpl	_P0_0
                                    395 ;	time_basic.c:42: loop_times = 0;
      0000AA E4               [12]  396 	clr	a
      0000AB F5 08            [12]  397 	mov	_loop_times,a
      0000AD F5 09            [12]  398 	mov	(_loop_times + 1),a
      0000AF                        399 00103$:
                                    400 ;	time_basic.c:44: }
      0000AF D0 D0            [24]  401 	pop	psw
      0000B1 D0 E0            [24]  402 	pop	acc
      0000B3 32               [24]  403 	reti
                                    404 ;	eliminated unneeded mov psw,# (no regs used in bank)
                                    405 ;	eliminated unneeded push/pop dpl
                                    406 ;	eliminated unneeded push/pop dph
                                    407 ;	eliminated unneeded push/pop b
                                    408 	.area CSEG    (CODE)
                                    409 	.area CONST   (CODE)
                                    410 	.area XINIT   (CODE)
                                    411 	.area CABS    (ABS,CODE)
