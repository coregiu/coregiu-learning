                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module interrupt_basic
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _Key_INT
                                     12 	.globl _main
                                     13 	.globl _delay_10us
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
                                    110 ;--------------------------------------------------------
                                    111 ; special function registers
                                    112 ;--------------------------------------------------------
                                    113 	.area RSEG    (ABS,DATA)
      000000                        114 	.org 0x0000
                           000080   115 _P0	=	0x0080
                           000081   116 _SP	=	0x0081
                           000082   117 _DPL	=	0x0082
                           000083   118 _DPH	=	0x0083
                           000087   119 _PCON	=	0x0087
                           000088   120 _TCON	=	0x0088
                           000089   121 _TMOD	=	0x0089
                           00008A   122 _TL0	=	0x008a
                           00008B   123 _TL1	=	0x008b
                           00008C   124 _TH0	=	0x008c
                           00008D   125 _TH1	=	0x008d
                           000090   126 _P1	=	0x0090
                           000098   127 _SCON	=	0x0098
                           000099   128 _SBUF	=	0x0099
                           0000A0   129 _P2	=	0x00a0
                           0000A8   130 _IE	=	0x00a8
                           0000B0   131 _P3	=	0x00b0
                           0000B8   132 _IP	=	0x00b8
                           0000D0   133 _PSW	=	0x00d0
                           0000E0   134 _ACC	=	0x00e0
                           0000F0   135 _B	=	0x00f0
                                    136 ;--------------------------------------------------------
                                    137 ; special function bits
                                    138 ;--------------------------------------------------------
                                    139 	.area RSEG    (ABS,DATA)
      000000                        140 	.org 0x0000
                           000080   141 _P0_0	=	0x0080
                           000081   142 _P0_1	=	0x0081
                           000082   143 _P0_2	=	0x0082
                           000083   144 _P0_3	=	0x0083
                           000084   145 _P0_4	=	0x0084
                           000085   146 _P0_5	=	0x0085
                           000086   147 _P0_6	=	0x0086
                           000087   148 _P0_7	=	0x0087
                           000088   149 _IT0	=	0x0088
                           000089   150 _IE0	=	0x0089
                           00008A   151 _IT1	=	0x008a
                           00008B   152 _IE1	=	0x008b
                           00008C   153 _TR0	=	0x008c
                           00008D   154 _TF0	=	0x008d
                           00008E   155 _TR1	=	0x008e
                           00008F   156 _TF1	=	0x008f
                           000090   157 _P1_0	=	0x0090
                           000091   158 _P1_1	=	0x0091
                           000092   159 _P1_2	=	0x0092
                           000093   160 _P1_3	=	0x0093
                           000094   161 _P1_4	=	0x0094
                           000095   162 _P1_5	=	0x0095
                           000096   163 _P1_6	=	0x0096
                           000097   164 _P1_7	=	0x0097
                           000098   165 _RI	=	0x0098
                           000099   166 _TI	=	0x0099
                           00009A   167 _RB8	=	0x009a
                           00009B   168 _TB8	=	0x009b
                           00009C   169 _REN	=	0x009c
                           00009D   170 _SM2	=	0x009d
                           00009E   171 _SM1	=	0x009e
                           00009F   172 _SM0	=	0x009f
                           0000A0   173 _P2_0	=	0x00a0
                           0000A1   174 _P2_1	=	0x00a1
                           0000A2   175 _P2_2	=	0x00a2
                           0000A3   176 _P2_3	=	0x00a3
                           0000A4   177 _P2_4	=	0x00a4
                           0000A5   178 _P2_5	=	0x00a5
                           0000A6   179 _P2_6	=	0x00a6
                           0000A7   180 _P2_7	=	0x00a7
                           0000A8   181 _EX0	=	0x00a8
                           0000A9   182 _ET0	=	0x00a9
                           0000AA   183 _EX1	=	0x00aa
                           0000AB   184 _ET1	=	0x00ab
                           0000AC   185 _ES	=	0x00ac
                           0000AF   186 _EA	=	0x00af
                           0000B0   187 _P3_0	=	0x00b0
                           0000B1   188 _P3_1	=	0x00b1
                           0000B2   189 _P3_2	=	0x00b2
                           0000B3   190 _P3_3	=	0x00b3
                           0000B4   191 _P3_4	=	0x00b4
                           0000B5   192 _P3_5	=	0x00b5
                           0000B6   193 _P3_6	=	0x00b6
                           0000B7   194 _P3_7	=	0x00b7
                           0000B0   195 _RXD	=	0x00b0
                           0000B1   196 _TXD	=	0x00b1
                           0000B2   197 _INT0	=	0x00b2
                           0000B3   198 _INT1	=	0x00b3
                           0000B4   199 _T0	=	0x00b4
                           0000B5   200 _T1	=	0x00b5
                           0000B6   201 _WR	=	0x00b6
                           0000B7   202 _RD	=	0x00b7
                           0000B8   203 _PX0	=	0x00b8
                           0000B9   204 _PT0	=	0x00b9
                           0000BA   205 _PX1	=	0x00ba
                           0000BB   206 _PT1	=	0x00bb
                           0000BC   207 _PS	=	0x00bc
                           0000D0   208 _P	=	0x00d0
                           0000D1   209 _F1	=	0x00d1
                           0000D2   210 _OV	=	0x00d2
                           0000D3   211 _RS0	=	0x00d3
                           0000D4   212 _RS1	=	0x00d4
                           0000D5   213 _F0	=	0x00d5
                           0000D6   214 _AC	=	0x00d6
                           0000D7   215 _CY	=	0x00d7
                                    216 ;--------------------------------------------------------
                                    217 ; overlayable register banks
                                    218 ;--------------------------------------------------------
                                    219 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        220 	.ds 8
                                    221 ;--------------------------------------------------------
                                    222 ; overlayable bit register bank
                                    223 ;--------------------------------------------------------
                                    224 	.area BIT_BANK	(REL,OVR,DATA)
      000020                        225 bits:
      000020                        226 	.ds 1
                           008000   227 	b0 = bits[0]
                           008100   228 	b1 = bits[1]
                           008200   229 	b2 = bits[2]
                           008300   230 	b3 = bits[3]
                           008400   231 	b4 = bits[4]
                           008500   232 	b5 = bits[5]
                           008600   233 	b6 = bits[6]
                           008700   234 	b7 = bits[7]
                                    235 ;--------------------------------------------------------
                                    236 ; internal ram data
                                    237 ;--------------------------------------------------------
                                    238 	.area DSEG    (DATA)
                                    239 ;--------------------------------------------------------
                                    240 ; overlayable items in internal ram 
                                    241 ;--------------------------------------------------------
                                    242 	.area	OSEG    (OVR,DATA)
                                    243 ;--------------------------------------------------------
                                    244 ; Stack segment in internal ram 
                                    245 ;--------------------------------------------------------
                                    246 	.area	SSEG
      000021                        247 __start__stack:
      000021                        248 	.ds	1
                                    249 
                                    250 ;--------------------------------------------------------
                                    251 ; indirectly addressable internal ram data
                                    252 ;--------------------------------------------------------
                                    253 	.area ISEG    (DATA)
                                    254 ;--------------------------------------------------------
                                    255 ; absolute internal ram data
                                    256 ;--------------------------------------------------------
                                    257 	.area IABS    (ABS,DATA)
                                    258 	.area IABS    (ABS,DATA)
                                    259 ;--------------------------------------------------------
                                    260 ; bit data
                                    261 ;--------------------------------------------------------
                                    262 	.area BSEG    (BIT)
                                    263 ;--------------------------------------------------------
                                    264 ; paged external ram data
                                    265 ;--------------------------------------------------------
                                    266 	.area PSEG    (PAG,XDATA)
                                    267 ;--------------------------------------------------------
                                    268 ; external ram data
                                    269 ;--------------------------------------------------------
                                    270 	.area XSEG    (XDATA)
                                    271 ;--------------------------------------------------------
                                    272 ; absolute external ram data
                                    273 ;--------------------------------------------------------
                                    274 	.area XABS    (ABS,XDATA)
                                    275 ;--------------------------------------------------------
                                    276 ; external initialized ram data
                                    277 ;--------------------------------------------------------
                                    278 	.area XISEG   (XDATA)
                                    279 	.area HOME    (CODE)
                                    280 	.area GSINIT0 (CODE)
                                    281 	.area GSINIT1 (CODE)
                                    282 	.area GSINIT2 (CODE)
                                    283 	.area GSINIT3 (CODE)
                                    284 	.area GSINIT4 (CODE)
                                    285 	.area GSINIT5 (CODE)
                                    286 	.area GSINIT  (CODE)
                                    287 	.area GSFINAL (CODE)
                                    288 	.area CSEG    (CODE)
                                    289 ;--------------------------------------------------------
                                    290 ; interrupt vector 
                                    291 ;--------------------------------------------------------
                                    292 	.area HOME    (CODE)
      000000                        293 __interrupt_vect:
      000000 02 00 09         [24]  294 	ljmp	__sdcc_gsinit_startup
      000003 02 00 7F         [24]  295 	ljmp	_Key_INT
                                    296 ;--------------------------------------------------------
                                    297 ; global & static initialisations
                                    298 ;--------------------------------------------------------
                                    299 	.area HOME    (CODE)
                                    300 	.area GSINIT  (CODE)
                                    301 	.area GSFINAL (CODE)
                                    302 	.area GSINIT  (CODE)
                                    303 	.globl __sdcc_gsinit_startup
                                    304 	.globl __sdcc_program_startup
                                    305 	.globl __start__stack
                                    306 	.globl __mcs51_genXINIT
                                    307 	.globl __mcs51_genXRAMCLEAR
                                    308 	.globl __mcs51_genRAMCLEAR
                                    309 	.area GSFINAL (CODE)
      000062 02 00 06         [24]  310 	ljmp	__sdcc_program_startup
                                    311 ;--------------------------------------------------------
                                    312 ; Home
                                    313 ;--------------------------------------------------------
                                    314 	.area HOME    (CODE)
                                    315 	.area HOME    (CODE)
      000006                        316 __sdcc_program_startup:
      000006 02 00 75         [24]  317 	ljmp	_main
                                    318 ;	return from main will return to caller
                                    319 ;--------------------------------------------------------
                                    320 ; code
                                    321 ;--------------------------------------------------------
                                    322 	.area CSEG    (CODE)
                                    323 ;------------------------------------------------------------
                                    324 ;Allocation info for local variables in function 'delay_10us'
                                    325 ;------------------------------------------------------------
                                    326 ;num                       Allocated to registers 
                                    327 ;------------------------------------------------------------
                                    328 ;	interrupt_basic.c:8: void delay_10us(unsigned int num) {
                                    329 ;	-----------------------------------------
                                    330 ;	 function delay_10us
                                    331 ;	-----------------------------------------
      000065                        332 _delay_10us:
                           000007   333 	ar7 = 0x07
                           000006   334 	ar6 = 0x06
                           000005   335 	ar5 = 0x05
                           000004   336 	ar4 = 0x04
                           000003   337 	ar3 = 0x03
                           000002   338 	ar2 = 0x02
                           000001   339 	ar1 = 0x01
                           000000   340 	ar0 = 0x00
      000065 AE 82            [24]  341 	mov	r6,dpl
      000067 AF 83            [24]  342 	mov	r7,dph
                                    343 ;	interrupt_basic.c:9: while (num > 0) {
      000069                        344 00101$:
      000069 EE               [12]  345 	mov	a,r6
      00006A 4F               [12]  346 	orl	a,r7
      00006B 60 07            [24]  347 	jz	00104$
                                    348 ;	interrupt_basic.c:10: num--;
      00006D 1E               [12]  349 	dec	r6
      00006E BE FF 01         [24]  350 	cjne	r6,#0xff,00116$
      000071 1F               [12]  351 	dec	r7
      000072                        352 00116$:
      000072 80 F5            [24]  353 	sjmp	00101$
      000074                        354 00104$:
                                    355 ;	interrupt_basic.c:12: }
      000074 22               [24]  356 	ret
                                    357 ;------------------------------------------------------------
                                    358 ;Allocation info for local variables in function 'main'
                                    359 ;------------------------------------------------------------
                                    360 ;	interrupt_basic.c:14: void main(void)
                                    361 ;	-----------------------------------------
                                    362 ;	 function main
                                    363 ;	-----------------------------------------
      000075                        364 _main:
                                    365 ;	interrupt_basic.c:16: LED = 0xff;       //P1口初始值，关闭led灯
                                    366 ;	assignBit
      000075 D2 80            [12]  367 	setb	_P0_0
                                    368 ;	interrupt_basic.c:17: EA=1;          //全局中断开
                                    369 ;	assignBit
      000077 D2 AF            [12]  370 	setb	_EA
                                    371 ;	interrupt_basic.c:18: EX0=1;         //外部中断0开
                                    372 ;	assignBit
      000079 D2 A8            [12]  373 	setb	_EX0
                                    374 ;	interrupt_basic.c:19: IT0=0;         //电平触发
                                    375 ;	assignBit
      00007B C2 88            [12]  376 	clr	_IT0
                                    377 ;	interrupt_basic.c:20: while(1)
      00007D                        378 00102$:
                                    379 ;	interrupt_basic.c:27: }
      00007D 80 FE            [24]  380 	sjmp	00102$
                                    381 ;------------------------------------------------------------
                                    382 ;Allocation info for local variables in function 'Key_INT'
                                    383 ;------------------------------------------------------------
                                    384 ;	interrupt_basic.c:31: void Key_INT(void) __interrupt 0 //R0 R1 =  0/1   0/1
                                    385 ;	-----------------------------------------
                                    386 ;	 function Key_INT
                                    387 ;	-----------------------------------------
      00007F                        388 _Key_INT:
      00007F C0 20            [24]  389 	push	bits
      000081 C0 E0            [24]  390 	push	acc
      000083 C0 F0            [24]  391 	push	b
      000085 C0 82            [24]  392 	push	dpl
      000087 C0 83            [24]  393 	push	dph
      000089 C0 07            [24]  394 	push	(0+7)
      00008B C0 06            [24]  395 	push	(0+6)
      00008D C0 05            [24]  396 	push	(0+5)
      00008F C0 04            [24]  397 	push	(0+4)
      000091 C0 03            [24]  398 	push	(0+3)
      000093 C0 02            [24]  399 	push	(0+2)
      000095 C0 01            [24]  400 	push	(0+1)
      000097 C0 00            [24]  401 	push	(0+0)
      000099 C0 D0            [24]  402 	push	psw
      00009B 75 D0 00         [24]  403 	mov	psw,#0x00
                                    404 ;	interrupt_basic.c:33: delay_10us(1000);
      00009E 90 03 E8         [24]  405 	mov	dptr,#0x03e8
      0000A1 12 00 65         [24]  406 	lcall	_delay_10us
                                    407 ;	interrupt_basic.c:34: if (P3_2 == 0) {
      0000A4 20 B2 02         [24]  408 	jb	_P3_2,00103$
                                    409 ;	interrupt_basic.c:35: LED = !LED;
      0000A7 B2 80            [12]  410 	cpl	_P0_0
      0000A9                        411 00103$:
                                    412 ;	interrupt_basic.c:37: }
      0000A9 D0 D0            [24]  413 	pop	psw
      0000AB D0 00            [24]  414 	pop	(0+0)
      0000AD D0 01            [24]  415 	pop	(0+1)
      0000AF D0 02            [24]  416 	pop	(0+2)
      0000B1 D0 03            [24]  417 	pop	(0+3)
      0000B3 D0 04            [24]  418 	pop	(0+4)
      0000B5 D0 05            [24]  419 	pop	(0+5)
      0000B7 D0 06            [24]  420 	pop	(0+6)
      0000B9 D0 07            [24]  421 	pop	(0+7)
      0000BB D0 83            [24]  422 	pop	dph
      0000BD D0 82            [24]  423 	pop	dpl
      0000BF D0 F0            [24]  424 	pop	b
      0000C1 D0 E0            [24]  425 	pop	acc
      0000C3 D0 20            [24]  426 	pop	bits
      0000C5 32               [24]  427 	reti
                                    428 	.area CSEG    (CODE)
                                    429 	.area CONST   (CODE)
                                    430 	.area XINIT   (CODE)
                                    431 	.area CABS    (ABS,CODE)
