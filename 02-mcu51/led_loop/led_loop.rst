                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module led_loop
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _main
                                     12 	.globl _delay
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
                                    109 ;--------------------------------------------------------
                                    110 ; special function registers
                                    111 ;--------------------------------------------------------
                                    112 	.area RSEG    (ABS,DATA)
      000000                        113 	.org 0x0000
                           000080   114 _P0	=	0x0080
                           000081   115 _SP	=	0x0081
                           000082   116 _DPL	=	0x0082
                           000083   117 _DPH	=	0x0083
                           000087   118 _PCON	=	0x0087
                           000088   119 _TCON	=	0x0088
                           000089   120 _TMOD	=	0x0089
                           00008A   121 _TL0	=	0x008a
                           00008B   122 _TL1	=	0x008b
                           00008C   123 _TH0	=	0x008c
                           00008D   124 _TH1	=	0x008d
                           000090   125 _P1	=	0x0090
                           000098   126 _SCON	=	0x0098
                           000099   127 _SBUF	=	0x0099
                           0000A0   128 _P2	=	0x00a0
                           0000A8   129 _IE	=	0x00a8
                           0000B0   130 _P3	=	0x00b0
                           0000B8   131 _IP	=	0x00b8
                           0000D0   132 _PSW	=	0x00d0
                           0000E0   133 _ACC	=	0x00e0
                           0000F0   134 _B	=	0x00f0
                                    135 ;--------------------------------------------------------
                                    136 ; special function bits
                                    137 ;--------------------------------------------------------
                                    138 	.area RSEG    (ABS,DATA)
      000000                        139 	.org 0x0000
                           000080   140 _P0_0	=	0x0080
                           000081   141 _P0_1	=	0x0081
                           000082   142 _P0_2	=	0x0082
                           000083   143 _P0_3	=	0x0083
                           000084   144 _P0_4	=	0x0084
                           000085   145 _P0_5	=	0x0085
                           000086   146 _P0_6	=	0x0086
                           000087   147 _P0_7	=	0x0087
                           000088   148 _IT0	=	0x0088
                           000089   149 _IE0	=	0x0089
                           00008A   150 _IT1	=	0x008a
                           00008B   151 _IE1	=	0x008b
                           00008C   152 _TR0	=	0x008c
                           00008D   153 _TF0	=	0x008d
                           00008E   154 _TR1	=	0x008e
                           00008F   155 _TF1	=	0x008f
                           000090   156 _P1_0	=	0x0090
                           000091   157 _P1_1	=	0x0091
                           000092   158 _P1_2	=	0x0092
                           000093   159 _P1_3	=	0x0093
                           000094   160 _P1_4	=	0x0094
                           000095   161 _P1_5	=	0x0095
                           000096   162 _P1_6	=	0x0096
                           000097   163 _P1_7	=	0x0097
                           000098   164 _RI	=	0x0098
                           000099   165 _TI	=	0x0099
                           00009A   166 _RB8	=	0x009a
                           00009B   167 _TB8	=	0x009b
                           00009C   168 _REN	=	0x009c
                           00009D   169 _SM2	=	0x009d
                           00009E   170 _SM1	=	0x009e
                           00009F   171 _SM0	=	0x009f
                           0000A0   172 _P2_0	=	0x00a0
                           0000A1   173 _P2_1	=	0x00a1
                           0000A2   174 _P2_2	=	0x00a2
                           0000A3   175 _P2_3	=	0x00a3
                           0000A4   176 _P2_4	=	0x00a4
                           0000A5   177 _P2_5	=	0x00a5
                           0000A6   178 _P2_6	=	0x00a6
                           0000A7   179 _P2_7	=	0x00a7
                           0000A8   180 _EX0	=	0x00a8
                           0000A9   181 _ET0	=	0x00a9
                           0000AA   182 _EX1	=	0x00aa
                           0000AB   183 _ET1	=	0x00ab
                           0000AC   184 _ES	=	0x00ac
                           0000AF   185 _EA	=	0x00af
                           0000B0   186 _P3_0	=	0x00b0
                           0000B1   187 _P3_1	=	0x00b1
                           0000B2   188 _P3_2	=	0x00b2
                           0000B3   189 _P3_3	=	0x00b3
                           0000B4   190 _P3_4	=	0x00b4
                           0000B5   191 _P3_5	=	0x00b5
                           0000B6   192 _P3_6	=	0x00b6
                           0000B7   193 _P3_7	=	0x00b7
                           0000B0   194 _RXD	=	0x00b0
                           0000B1   195 _TXD	=	0x00b1
                           0000B2   196 _INT0	=	0x00b2
                           0000B3   197 _INT1	=	0x00b3
                           0000B4   198 _T0	=	0x00b4
                           0000B5   199 _T1	=	0x00b5
                           0000B6   200 _WR	=	0x00b6
                           0000B7   201 _RD	=	0x00b7
                           0000B8   202 _PX0	=	0x00b8
                           0000B9   203 _PT0	=	0x00b9
                           0000BA   204 _PX1	=	0x00ba
                           0000BB   205 _PT1	=	0x00bb
                           0000BC   206 _PS	=	0x00bc
                           0000D0   207 _P	=	0x00d0
                           0000D1   208 _F1	=	0x00d1
                           0000D2   209 _OV	=	0x00d2
                           0000D3   210 _RS0	=	0x00d3
                           0000D4   211 _RS1	=	0x00d4
                           0000D5   212 _F0	=	0x00d5
                           0000D6   213 _AC	=	0x00d6
                           0000D7   214 _CY	=	0x00d7
                                    215 ;--------------------------------------------------------
                                    216 ; overlayable register banks
                                    217 ;--------------------------------------------------------
                                    218 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        219 	.ds 8
                                    220 ;--------------------------------------------------------
                                    221 ; internal ram data
                                    222 ;--------------------------------------------------------
                                    223 	.area DSEG    (DATA)
                                    224 ;--------------------------------------------------------
                                    225 ; overlayable items in internal ram 
                                    226 ;--------------------------------------------------------
                                    227 	.area	OSEG    (OVR,DATA)
                                    228 ;--------------------------------------------------------
                                    229 ; Stack segment in internal ram 
                                    230 ;--------------------------------------------------------
                                    231 	.area	SSEG
      000008                        232 __start__stack:
      000008                        233 	.ds	1
                                    234 
                                    235 ;--------------------------------------------------------
                                    236 ; indirectly addressable internal ram data
                                    237 ;--------------------------------------------------------
                                    238 	.area ISEG    (DATA)
                                    239 ;--------------------------------------------------------
                                    240 ; absolute internal ram data
                                    241 ;--------------------------------------------------------
                                    242 	.area IABS    (ABS,DATA)
                                    243 	.area IABS    (ABS,DATA)
                                    244 ;--------------------------------------------------------
                                    245 ; bit data
                                    246 ;--------------------------------------------------------
                                    247 	.area BSEG    (BIT)
                                    248 ;--------------------------------------------------------
                                    249 ; paged external ram data
                                    250 ;--------------------------------------------------------
                                    251 	.area PSEG    (PAG,XDATA)
                                    252 ;--------------------------------------------------------
                                    253 ; external ram data
                                    254 ;--------------------------------------------------------
                                    255 	.area XSEG    (XDATA)
                                    256 ;--------------------------------------------------------
                                    257 ; absolute external ram data
                                    258 ;--------------------------------------------------------
                                    259 	.area XABS    (ABS,XDATA)
                                    260 ;--------------------------------------------------------
                                    261 ; external initialized ram data
                                    262 ;--------------------------------------------------------
                                    263 	.area XISEG   (XDATA)
                                    264 	.area HOME    (CODE)
                                    265 	.area GSINIT0 (CODE)
                                    266 	.area GSINIT1 (CODE)
                                    267 	.area GSINIT2 (CODE)
                                    268 	.area GSINIT3 (CODE)
                                    269 	.area GSINIT4 (CODE)
                                    270 	.area GSINIT5 (CODE)
                                    271 	.area GSINIT  (CODE)
                                    272 	.area GSFINAL (CODE)
                                    273 	.area CSEG    (CODE)
                                    274 ;--------------------------------------------------------
                                    275 ; interrupt vector 
                                    276 ;--------------------------------------------------------
                                    277 	.area HOME    (CODE)
      000000                        278 __interrupt_vect:
      000000 02 00 06         [24]  279 	ljmp	__sdcc_gsinit_startup
                                    280 ;--------------------------------------------------------
                                    281 ; global & static initialisations
                                    282 ;--------------------------------------------------------
                                    283 	.area HOME    (CODE)
                                    284 	.area GSINIT  (CODE)
                                    285 	.area GSFINAL (CODE)
                                    286 	.area GSINIT  (CODE)
                                    287 	.globl __sdcc_gsinit_startup
                                    288 	.globl __sdcc_program_startup
                                    289 	.globl __start__stack
                                    290 	.globl __mcs51_genXINIT
                                    291 	.globl __mcs51_genXRAMCLEAR
                                    292 	.globl __mcs51_genRAMCLEAR
                                    293 	.area GSFINAL (CODE)
      00005F 02 00 03         [24]  294 	ljmp	__sdcc_program_startup
                                    295 ;--------------------------------------------------------
                                    296 ; Home
                                    297 ;--------------------------------------------------------
                                    298 	.area HOME    (CODE)
                                    299 	.area HOME    (CODE)
      000003                        300 __sdcc_program_startup:
      000003 02 00 89         [24]  301 	ljmp	_main
                                    302 ;	return from main will return to caller
                                    303 ;--------------------------------------------------------
                                    304 ; code
                                    305 ;--------------------------------------------------------
                                    306 	.area CSEG    (CODE)
                                    307 ;------------------------------------------------------------
                                    308 ;Allocation info for local variables in function 'delay'
                                    309 ;------------------------------------------------------------
                                    310 ;i                         Allocated to registers r6 r7 
                                    311 ;j                         Allocated to registers r4 r5 
                                    312 ;------------------------------------------------------------
                                    313 ;	led_loop.c:3: void delay() {
                                    314 ;	-----------------------------------------
                                    315 ;	 function delay
                                    316 ;	-----------------------------------------
      000062                        317 _delay:
                           000007   318 	ar7 = 0x07
                           000006   319 	ar6 = 0x06
                           000005   320 	ar5 = 0x05
                           000004   321 	ar4 = 0x04
                           000003   322 	ar3 = 0x03
                           000002   323 	ar2 = 0x02
                           000001   324 	ar1 = 0x01
                           000000   325 	ar0 = 0x00
                                    326 ;	led_loop.c:5: for (i=0; i < 100; i++){
      000062 7E 00            [12]  327 	mov	r6,#0x00
      000064 7F 00            [12]  328 	mov	r7,#0x00
      000066                        329 00106$:
                                    330 ;	led_loop.c:6: for (j=0; j < 100; j++) {
      000066 7C 64            [12]  331 	mov	r4,#0x64
      000068 7D 00            [12]  332 	mov	r5,#0x00
      00006A                        333 00105$:
      00006A EC               [12]  334 	mov	a,r4
      00006B 24 FF            [12]  335 	add	a,#0xff
      00006D FA               [12]  336 	mov	r2,a
      00006E ED               [12]  337 	mov	a,r5
      00006F 34 FF            [12]  338 	addc	a,#0xff
      000071 FB               [12]  339 	mov	r3,a
      000072 8A 04            [24]  340 	mov	ar4,r2
      000074 8B 05            [24]  341 	mov	ar5,r3
      000076 EA               [12]  342 	mov	a,r2
      000077 4B               [12]  343 	orl	a,r3
      000078 70 F0            [24]  344 	jnz	00105$
                                    345 ;	led_loop.c:5: for (i=0; i < 100; i++){
      00007A 0E               [12]  346 	inc	r6
      00007B BE 00 01         [24]  347 	cjne	r6,#0x00,00124$
      00007E 0F               [12]  348 	inc	r7
      00007F                        349 00124$:
      00007F C3               [12]  350 	clr	c
      000080 EE               [12]  351 	mov	a,r6
      000081 94 64            [12]  352 	subb	a,#0x64
      000083 EF               [12]  353 	mov	a,r7
      000084 94 00            [12]  354 	subb	a,#0x00
      000086 40 DE            [24]  355 	jc	00106$
                                    356 ;	led_loop.c:17: }
      000088 22               [24]  357 	ret
                                    358 ;------------------------------------------------------------
                                    359 ;Allocation info for local variables in function 'main'
                                    360 ;------------------------------------------------------------
                                    361 ;data1                     Allocated to registers r6 r7 
                                    362 ;data2                     Allocated to registers r4 r5 
                                    363 ;num                       Allocated to registers r2 r3 
                                    364 ;------------------------------------------------------------
                                    365 ;	led_loop.c:19: void main(){
                                    366 ;	-----------------------------------------
                                    367 ;	 function main
                                    368 ;	-----------------------------------------
      000089                        369 _main:
                                    370 ;	led_loop.c:21: while(1) {
      000089                        371 00104$:
                                    372 ;	led_loop.c:22: data1=0xfe;
      000089 7E FE            [12]  373 	mov	r6,#0xfe
      00008B 7F 00            [12]  374 	mov	r7,#0x00
                                    375 ;	led_loop.c:23: data2=0x7f;
      00008D 7C 7F            [12]  376 	mov	r4,#0x7f
      00008F 7D 00            [12]  377 	mov	r5,#0x00
                                    378 ;	led_loop.c:24: for(num=0; num<7; num++) {
      000091 7A 00            [12]  379 	mov	r2,#0x00
      000093 7B 00            [12]  380 	mov	r3,#0x00
      000095                        381 00106$:
                                    382 ;	led_loop.c:25: P0=data1;
                                    383 ;	led_loop.c:26: data1=data1<<1;
      000095 EE               [12]  384 	mov	a,r6
      000096 F5 80            [12]  385 	mov	_P0,a
      000098 25 E0            [12]  386 	add	a,acc
      00009A FE               [12]  387 	mov	r6,a
      00009B EF               [12]  388 	mov	a,r7
      00009C 33               [12]  389 	rlc	a
      00009D FF               [12]  390 	mov	r7,a
                                    391 ;	led_loop.c:27: delay();
      00009E C0 07            [24]  392 	push	ar7
      0000A0 C0 06            [24]  393 	push	ar6
      0000A2 C0 05            [24]  394 	push	ar5
      0000A4 C0 04            [24]  395 	push	ar4
      0000A6 C0 03            [24]  396 	push	ar3
      0000A8 C0 02            [24]  397 	push	ar2
      0000AA 12 00 62         [24]  398 	lcall	_delay
      0000AD D0 02            [24]  399 	pop	ar2
      0000AF D0 03            [24]  400 	pop	ar3
      0000B1 D0 04            [24]  401 	pop	ar4
      0000B3 D0 05            [24]  402 	pop	ar5
      0000B5 D0 06            [24]  403 	pop	ar6
      0000B7 D0 07            [24]  404 	pop	ar7
                                    405 ;	led_loop.c:24: for(num=0; num<7; num++) {
      0000B9 0A               [12]  406 	inc	r2
      0000BA BA 00 01         [24]  407 	cjne	r2,#0x00,00134$
      0000BD 0B               [12]  408 	inc	r3
      0000BE                        409 00134$:
      0000BE C3               [12]  410 	clr	c
      0000BF EA               [12]  411 	mov	a,r2
      0000C0 94 07            [12]  412 	subb	a,#0x07
      0000C2 EB               [12]  413 	mov	a,r3
      0000C3 94 00            [12]  414 	subb	a,#0x00
      0000C5 40 CE            [24]  415 	jc	00106$
                                    416 ;	led_loop.c:29: for(num=0; num<7; num++) {
      0000C7 7E 00            [12]  417 	mov	r6,#0x00
      0000C9 7F 00            [12]  418 	mov	r7,#0x00
      0000CB                        419 00108$:
                                    420 ;	led_loop.c:30: P0=data2;
      0000CB 8C 80            [24]  421 	mov	_P0,r4
                                    422 ;	led_loop.c:31: data2=data2>>1;
      0000CD ED               [12]  423 	mov	a,r5
      0000CE C3               [12]  424 	clr	c
      0000CF 13               [12]  425 	rrc	a
      0000D0 CC               [12]  426 	xch	a,r4
      0000D1 13               [12]  427 	rrc	a
      0000D2 CC               [12]  428 	xch	a,r4
      0000D3 FD               [12]  429 	mov	r5,a
                                    430 ;	led_loop.c:32: delay();
      0000D4 C0 07            [24]  431 	push	ar7
      0000D6 C0 06            [24]  432 	push	ar6
      0000D8 C0 05            [24]  433 	push	ar5
      0000DA C0 04            [24]  434 	push	ar4
      0000DC 12 00 62         [24]  435 	lcall	_delay
      0000DF D0 04            [24]  436 	pop	ar4
      0000E1 D0 05            [24]  437 	pop	ar5
      0000E3 D0 06            [24]  438 	pop	ar6
      0000E5 D0 07            [24]  439 	pop	ar7
                                    440 ;	led_loop.c:29: for(num=0; num<7; num++) {
      0000E7 0E               [12]  441 	inc	r6
      0000E8 BE 00 01         [24]  442 	cjne	r6,#0x00,00136$
      0000EB 0F               [12]  443 	inc	r7
      0000EC                        444 00136$:
      0000EC C3               [12]  445 	clr	c
      0000ED EE               [12]  446 	mov	a,r6
      0000EE 94 07            [12]  447 	subb	a,#0x07
      0000F0 EF               [12]  448 	mov	a,r7
      0000F1 94 00            [12]  449 	subb	a,#0x00
      0000F3 40 D6            [24]  450 	jc	00108$
                                    451 ;	led_loop.c:35: }
      0000F5 80 92            [24]  452 	sjmp	00104$
                                    453 	.area CSEG    (CODE)
                                    454 	.area CONST   (CODE)
                                    455 	.area XINIT   (CODE)
                                    456 	.area CABS    (ABS,CODE)
