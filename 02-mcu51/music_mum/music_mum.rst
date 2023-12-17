                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module music_mum
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _FREQL
                                     12 	.globl _FREQH
                                     13 	.globl _sszymmh
                                     14 	.globl _t0int
                                     15 	.globl _main
                                     16 	.globl _song
                                     17 	.globl _delay
                                     18 	.globl _change_led
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
                                    115 	.globl _flag
                                    116 	.globl _led_times
                                    117 	.globl _time
                                    118 	.globl _timer0l
                                    119 	.globl _timer0h
                                    120 ;--------------------------------------------------------
                                    121 ; special function registers
                                    122 ;--------------------------------------------------------
                                    123 	.area RSEG    (ABS,DATA)
      000000                        124 	.org 0x0000
                           000080   125 _P0	=	0x0080
                           000081   126 _SP	=	0x0081
                           000082   127 _DPL	=	0x0082
                           000083   128 _DPH	=	0x0083
                           000087   129 _PCON	=	0x0087
                           000088   130 _TCON	=	0x0088
                           000089   131 _TMOD	=	0x0089
                           00008A   132 _TL0	=	0x008a
                           00008B   133 _TL1	=	0x008b
                           00008C   134 _TH0	=	0x008c
                           00008D   135 _TH1	=	0x008d
                           000090   136 _P1	=	0x0090
                           000098   137 _SCON	=	0x0098
                           000099   138 _SBUF	=	0x0099
                           0000A0   139 _P2	=	0x00a0
                           0000A8   140 _IE	=	0x00a8
                           0000B0   141 _P3	=	0x00b0
                           0000B8   142 _IP	=	0x00b8
                           0000D0   143 _PSW	=	0x00d0
                           0000E0   144 _ACC	=	0x00e0
                           0000F0   145 _B	=	0x00f0
                                    146 ;--------------------------------------------------------
                                    147 ; special function bits
                                    148 ;--------------------------------------------------------
                                    149 	.area RSEG    (ABS,DATA)
      000000                        150 	.org 0x0000
                           000080   151 _P0_0	=	0x0080
                           000081   152 _P0_1	=	0x0081
                           000082   153 _P0_2	=	0x0082
                           000083   154 _P0_3	=	0x0083
                           000084   155 _P0_4	=	0x0084
                           000085   156 _P0_5	=	0x0085
                           000086   157 _P0_6	=	0x0086
                           000087   158 _P0_7	=	0x0087
                           000088   159 _IT0	=	0x0088
                           000089   160 _IE0	=	0x0089
                           00008A   161 _IT1	=	0x008a
                           00008B   162 _IE1	=	0x008b
                           00008C   163 _TR0	=	0x008c
                           00008D   164 _TF0	=	0x008d
                           00008E   165 _TR1	=	0x008e
                           00008F   166 _TF1	=	0x008f
                           000090   167 _P1_0	=	0x0090
                           000091   168 _P1_1	=	0x0091
                           000092   169 _P1_2	=	0x0092
                           000093   170 _P1_3	=	0x0093
                           000094   171 _P1_4	=	0x0094
                           000095   172 _P1_5	=	0x0095
                           000096   173 _P1_6	=	0x0096
                           000097   174 _P1_7	=	0x0097
                           000098   175 _RI	=	0x0098
                           000099   176 _TI	=	0x0099
                           00009A   177 _RB8	=	0x009a
                           00009B   178 _TB8	=	0x009b
                           00009C   179 _REN	=	0x009c
                           00009D   180 _SM2	=	0x009d
                           00009E   181 _SM1	=	0x009e
                           00009F   182 _SM0	=	0x009f
                           0000A0   183 _P2_0	=	0x00a0
                           0000A1   184 _P2_1	=	0x00a1
                           0000A2   185 _P2_2	=	0x00a2
                           0000A3   186 _P2_3	=	0x00a3
                           0000A4   187 _P2_4	=	0x00a4
                           0000A5   188 _P2_5	=	0x00a5
                           0000A6   189 _P2_6	=	0x00a6
                           0000A7   190 _P2_7	=	0x00a7
                           0000A8   191 _EX0	=	0x00a8
                           0000A9   192 _ET0	=	0x00a9
                           0000AA   193 _EX1	=	0x00aa
                           0000AB   194 _ET1	=	0x00ab
                           0000AC   195 _ES	=	0x00ac
                           0000AF   196 _EA	=	0x00af
                           0000B0   197 _P3_0	=	0x00b0
                           0000B1   198 _P3_1	=	0x00b1
                           0000B2   199 _P3_2	=	0x00b2
                           0000B3   200 _P3_3	=	0x00b3
                           0000B4   201 _P3_4	=	0x00b4
                           0000B5   202 _P3_5	=	0x00b5
                           0000B6   203 _P3_6	=	0x00b6
                           0000B7   204 _P3_7	=	0x00b7
                           0000B0   205 _RXD	=	0x00b0
                           0000B1   206 _TXD	=	0x00b1
                           0000B2   207 _INT0	=	0x00b2
                           0000B3   208 _INT1	=	0x00b3
                           0000B4   209 _T0	=	0x00b4
                           0000B5   210 _T1	=	0x00b5
                           0000B6   211 _WR	=	0x00b6
                           0000B7   212 _RD	=	0x00b7
                           0000B8   213 _PX0	=	0x00b8
                           0000B9   214 _PT0	=	0x00b9
                           0000BA   215 _PX1	=	0x00ba
                           0000BB   216 _PT1	=	0x00bb
                           0000BC   217 _PS	=	0x00bc
                           0000D0   218 _P	=	0x00d0
                           0000D1   219 _F1	=	0x00d1
                           0000D2   220 _OV	=	0x00d2
                           0000D3   221 _RS0	=	0x00d3
                           0000D4   222 _RS1	=	0x00d4
                           0000D5   223 _F0	=	0x00d5
                           0000D6   224 _AC	=	0x00d6
                           0000D7   225 _CY	=	0x00d7
                                    226 ;--------------------------------------------------------
                                    227 ; overlayable register banks
                                    228 ;--------------------------------------------------------
                                    229 	.area REG_BANK_0	(REL,OVR,DATA)
      000000                        230 	.ds 8
                                    231 ;--------------------------------------------------------
                                    232 ; internal ram data
                                    233 ;--------------------------------------------------------
                                    234 	.area DSEG    (DATA)
      000008                        235 _timer0h::
      000008                        236 	.ds 1
      000009                        237 _timer0l::
      000009                        238 	.ds 1
      00000A                        239 _time::
      00000A                        240 	.ds 1
      00000B                        241 _led_times::
      00000B                        242 	.ds 2
      00000D                        243 _flag::
      00000D                        244 	.ds 2
      00000F                        245 _delay_t_65536_3:
      00000F                        246 	.ds 1
      000010                        247 _delay_t2_65536_4:
      000010                        248 	.ds 4
                                    249 ;--------------------------------------------------------
                                    250 ; overlayable items in internal ram 
                                    251 ;--------------------------------------------------------
                                    252 ;--------------------------------------------------------
                                    253 ; Stack segment in internal ram 
                                    254 ;--------------------------------------------------------
                                    255 	.area	SSEG
      000014                        256 __start__stack:
      000014                        257 	.ds	1
                                    258 
                                    259 ;--------------------------------------------------------
                                    260 ; indirectly addressable internal ram data
                                    261 ;--------------------------------------------------------
                                    262 	.area ISEG    (DATA)
                                    263 ;--------------------------------------------------------
                                    264 ; absolute internal ram data
                                    265 ;--------------------------------------------------------
                                    266 	.area IABS    (ABS,DATA)
                                    267 	.area IABS    (ABS,DATA)
                                    268 ;--------------------------------------------------------
                                    269 ; bit data
                                    270 ;--------------------------------------------------------
                                    271 	.area BSEG    (BIT)
                                    272 ;--------------------------------------------------------
                                    273 ; paged external ram data
                                    274 ;--------------------------------------------------------
                                    275 	.area PSEG    (PAG,XDATA)
                                    276 ;--------------------------------------------------------
                                    277 ; external ram data
                                    278 ;--------------------------------------------------------
                                    279 	.area XSEG    (XDATA)
                                    280 ;--------------------------------------------------------
                                    281 ; absolute external ram data
                                    282 ;--------------------------------------------------------
                                    283 	.area XABS    (ABS,XDATA)
                                    284 ;--------------------------------------------------------
                                    285 ; external initialized ram data
                                    286 ;--------------------------------------------------------
                                    287 	.area XISEG   (XDATA)
                                    288 	.area HOME    (CODE)
                                    289 	.area GSINIT0 (CODE)
                                    290 	.area GSINIT1 (CODE)
                                    291 	.area GSINIT2 (CODE)
                                    292 	.area GSINIT3 (CODE)
                                    293 	.area GSINIT4 (CODE)
                                    294 	.area GSINIT5 (CODE)
                                    295 	.area GSINIT  (CODE)
                                    296 	.area GSFINAL (CODE)
                                    297 	.area CSEG    (CODE)
                                    298 ;--------------------------------------------------------
                                    299 ; interrupt vector 
                                    300 ;--------------------------------------------------------
                                    301 	.area HOME    (CODE)
      000000                        302 __interrupt_vect:
      000000 02 00 11         [24]  303 	ljmp	__sdcc_gsinit_startup
      000003 32               [24]  304 	reti
      000004                        305 	.ds	7
      00000B 02 01 66         [24]  306 	ljmp	_t0int
                                    307 ;--------------------------------------------------------
                                    308 ; global & static initialisations
                                    309 ;--------------------------------------------------------
                                    310 	.area HOME    (CODE)
                                    311 	.area GSINIT  (CODE)
                                    312 	.area GSFINAL (CODE)
                                    313 	.area GSINIT  (CODE)
                                    314 	.globl __sdcc_gsinit_startup
                                    315 	.globl __sdcc_program_startup
                                    316 	.globl __start__stack
                                    317 	.globl __mcs51_genXINIT
                                    318 	.globl __mcs51_genXRAMCLEAR
                                    319 	.globl __mcs51_genRAMCLEAR
                                    320 ;	music_mum.c:24: unsigned int led_times = 0;
      00006A E4               [12]  321 	clr	a
      00006B F5 0B            [12]  322 	mov	_led_times,a
      00006D F5 0C            [12]  323 	mov	(_led_times + 1),a
                                    324 ;	music_mum.c:25: unsigned int flag = 1;
      00006F 75 0D 01         [24]  325 	mov	_flag,#0x01
                                    326 ;	1-genFromRTrack replaced	mov	(_flag + 1),#0x00
      000072 F5 0E            [12]  327 	mov	(_flag + 1),a
                                    328 	.area GSFINAL (CODE)
      000074 02 00 0E         [24]  329 	ljmp	__sdcc_program_startup
                                    330 ;--------------------------------------------------------
                                    331 ; Home
                                    332 ;--------------------------------------------------------
                                    333 	.area HOME    (CODE)
                                    334 	.area HOME    (CODE)
      00000E                        335 __sdcc_program_startup:
      00000E 02 01 06         [24]  336 	ljmp	_main
                                    337 ;	return from main will return to caller
                                    338 ;--------------------------------------------------------
                                    339 ; code
                                    340 ;--------------------------------------------------------
                                    341 	.area CSEG    (CODE)
                                    342 ;------------------------------------------------------------
                                    343 ;Allocation info for local variables in function 'change_led'
                                    344 ;------------------------------------------------------------
                                    345 ;	music_mum.c:26: void change_led(){
                                    346 ;	-----------------------------------------
                                    347 ;	 function change_led
                                    348 ;	-----------------------------------------
      000077                        349 _change_led:
                           000007   350 	ar7 = 0x07
                           000006   351 	ar6 = 0x06
                           000005   352 	ar5 = 0x05
                           000004   353 	ar4 = 0x04
                           000003   354 	ar3 = 0x03
                           000002   355 	ar2 = 0x02
                           000001   356 	ar1 = 0x01
                           000000   357 	ar0 = 0x00
                                    358 ;	music_mum.c:27: if (led_times > 8) {
      000077 C3               [12]  359 	clr	c
      000078 74 08            [12]  360 	mov	a,#0x08
      00007A 95 0B            [12]  361 	subb	a,_led_times
      00007C E4               [12]  362 	clr	a
      00007D 95 0C            [12]  363 	subb	a,(_led_times + 1)
      00007F 50 19            [24]  364 	jnc	00102$
                                    365 ;	music_mum.c:28: flag = !flag;
      000081 E5 0D            [12]  366 	mov	a,_flag
      000083 45 0E            [12]  367 	orl	a,(_flag + 1)
      000085 B4 01 00         [24]  368 	cjne	a,#0x01,00116$
      000088                        369 00116$:
      000088 E4               [12]  370 	clr	a
      000089 33               [12]  371 	rlc	a
      00008A FF               [12]  372 	mov	r7,a
      00008B 8F 0D            [24]  373 	mov	_flag,r7
      00008D 33               [12]  374 	rlc	a
      00008E 95 E0            [12]  375 	subb	a,acc
      000090 F5 0E            [12]  376 	mov	(_flag + 1),a
                                    377 ;	music_mum.c:29: P0 = 0xff;
      000092 75 80 FF         [24]  378 	mov	_P0,#0xff
                                    379 ;	music_mum.c:30: led_times = 0;
      000095 E4               [12]  380 	clr	a
      000096 F5 0B            [12]  381 	mov	_led_times,a
      000098 F5 0C            [12]  382 	mov	(_led_times + 1),a
      00009A                        383 00102$:
                                    384 ;	music_mum.c:32: P0 = flag ? P0 << 1 : P0 >> 1;
      00009A E5 0D            [12]  385 	mov	a,_flag
      00009C 45 0E            [12]  386 	orl	a,(_flag + 1)
      00009E 60 07            [24]  387 	jz	00105$
      0000A0 E5 80            [12]  388 	mov	a,_P0
      0000A2 25 E0            [12]  389 	add	a,acc
      0000A4 FF               [12]  390 	mov	r7,a
      0000A5 80 05            [24]  391 	sjmp	00106$
      0000A7                        392 00105$:
      0000A7 E5 80            [12]  393 	mov	a,_P0
      0000A9 C3               [12]  394 	clr	c
      0000AA 13               [12]  395 	rrc	a
      0000AB FF               [12]  396 	mov	r7,a
      0000AC                        397 00106$:
      0000AC 8F 80            [24]  398 	mov	_P0,r7
                                    399 ;	music_mum.c:33: led_times++;
      0000AE 05 0B            [12]  400 	inc	_led_times
      0000B0 E4               [12]  401 	clr	a
      0000B1 B5 0B 02         [24]  402 	cjne	a,_led_times,00118$
      0000B4 05 0C            [12]  403 	inc	(_led_times + 1)
      0000B6                        404 00118$:
                                    405 ;	music_mum.c:34: }
      0000B6 22               [24]  406 	ret
                                    407 ;------------------------------------------------------------
                                    408 ;Allocation info for local variables in function 'delay'
                                    409 ;------------------------------------------------------------
                                    410 ;t                         Allocated with name '_delay_t_65536_3'
                                    411 ;t1                        Allocated to registers r6 
                                    412 ;t2                        Allocated with name '_delay_t2_65536_4'
                                    413 ;------------------------------------------------------------
                                    414 ;	music_mum.c:36: void delay(uchar t)		  // 延时函数 
                                    415 ;	-----------------------------------------
                                    416 ;	 function delay
                                    417 ;	-----------------------------------------
      0000B7                        418 _delay:
      0000B7 85 82 0F         [24]  419 	mov	_delay_t_65536_3,dpl
                                    420 ;	music_mum.c:40: for(t1=0;t1<t;t1++)
      0000BA 7E 00            [12]  421 	mov	r6,#0x00
      0000BC                        422 00107$:
      0000BC C3               [12]  423 	clr	c
      0000BD EE               [12]  424 	mov	a,r6
      0000BE 95 0F            [12]  425 	subb	a,_delay_t_65536_3
      0000C0 50 30            [24]  426 	jnc	00102$
                                    427 ;	music_mum.c:42: for(t2=0;t2<8000;t2++);
      0000C2 75 10 40         [24]  428 	mov	_delay_t2_65536_4,#0x40
      0000C5 75 11 1F         [24]  429 	mov	(_delay_t2_65536_4 + 1),#0x1f
      0000C8 E4               [12]  430 	clr	a
      0000C9 F5 12            [12]  431 	mov	(_delay_t2_65536_4 + 2),a
      0000CB F5 13            [12]  432 	mov	(_delay_t2_65536_4 + 3),a
      0000CD                        433 00105$:
      0000CD E5 10            [12]  434 	mov	a,_delay_t2_65536_4
      0000CF 24 FF            [12]  435 	add	a,#0xff
      0000D1 F8               [12]  436 	mov	r0,a
      0000D2 E5 11            [12]  437 	mov	a,(_delay_t2_65536_4 + 1)
      0000D4 34 FF            [12]  438 	addc	a,#0xff
      0000D6 F9               [12]  439 	mov	r1,a
      0000D7 E5 12            [12]  440 	mov	a,(_delay_t2_65536_4 + 2)
      0000D9 34 FF            [12]  441 	addc	a,#0xff
      0000DB FF               [12]  442 	mov	r7,a
      0000DC E5 13            [12]  443 	mov	a,(_delay_t2_65536_4 + 3)
      0000DE 34 FF            [12]  444 	addc	a,#0xff
      0000E0 FD               [12]  445 	mov	r5,a
      0000E1 88 10            [24]  446 	mov	_delay_t2_65536_4,r0
      0000E3 89 11            [24]  447 	mov	(_delay_t2_65536_4 + 1),r1
      0000E5 8F 12            [24]  448 	mov	(_delay_t2_65536_4 + 2),r7
      0000E7 8D 13            [24]  449 	mov	(_delay_t2_65536_4 + 3),r5
      0000E9 E8               [12]  450 	mov	a,r0
      0000EA 49               [12]  451 	orl	a,r1
      0000EB 4F               [12]  452 	orl	a,r7
      0000EC 4D               [12]  453 	orl	a,r5
      0000ED 70 DE            [24]  454 	jnz	00105$
                                    455 ;	music_mum.c:40: for(t1=0;t1<t;t1++)
      0000EF 0E               [12]  456 	inc	r6
      0000F0 80 CA            [24]  457 	sjmp	00107$
      0000F2                        458 00102$:
                                    459 ;	music_mum.c:44: change_led();
      0000F2 12 00 77         [24]  460 	lcall	_change_led
                                    461 ;	music_mum.c:45: TR0=0;
                                    462 ;	assignBit
      0000F5 C2 8C            [12]  463 	clr	_TR0
                                    464 ;	music_mum.c:46: }
      0000F7 22               [24]  465 	ret
                                    466 ;------------------------------------------------------------
                                    467 ;Allocation info for local variables in function 'song'
                                    468 ;------------------------------------------------------------
                                    469 ;	music_mum.c:47: void song()				 //  音乐处理函数
                                    470 ;	-----------------------------------------
                                    471 ;	 function song
                                    472 ;	-----------------------------------------
      0000F8                        473 _song:
                                    474 ;	music_mum.c:49: TH0=timer0h;
      0000F8 85 08 8C         [24]  475 	mov	_TH0,_timer0h
                                    476 ;	music_mum.c:50: TL0=timer0l;
      0000FB 85 09 8A         [24]  477 	mov	_TL0,_timer0l
                                    478 ;	music_mum.c:51: TR0=1;
                                    479 ;	assignBit
      0000FE D2 8C            [12]  480 	setb	_TR0
                                    481 ;	music_mum.c:52: delay(time);                       
      000100 85 0A 82         [24]  482 	mov	dpl,_time
                                    483 ;	music_mum.c:53: }
      000103 02 00 B7         [24]  484 	ljmp	_delay
                                    485 ;------------------------------------------------------------
                                    486 ;Allocation info for local variables in function 'main'
                                    487 ;------------------------------------------------------------
                                    488 ;k                         Allocated to registers r6 
                                    489 ;i                         Allocated to registers r7 
                                    490 ;------------------------------------------------------------
                                    491 ;	music_mum.c:57: void main()
                                    492 ;	-----------------------------------------
                                    493 ;	 function main
                                    494 ;	-----------------------------------------
      000106                        495 _main:
                                    496 ;	music_mum.c:60: TMOD=1; 			//置CT0定时工作方式1
      000106 75 89 01         [24]  497 	mov	_TMOD,#0x01
                                    498 ;	music_mum.c:61: EA=1;
                                    499 ;	assignBit
      000109 D2 AF            [12]  500 	setb	_EA
                                    501 ;	music_mum.c:62: ET0=1;				//IE=0x82 //CPU开中断,CT0开中断 
                                    502 ;	assignBit
      00010B D2 A9            [12]  503 	setb	_ET0
                                    504 ;	music_mum.c:63: while(1)
      00010D                        505 00105$:
                                    506 ;	music_mum.c:65: i=0;  
      00010D 7F 00            [12]  507 	mov	r7,#0x00
                                    508 ;	music_mum.c:66: while(i<100)				  //音乐数组长度 ，唱完从头再来  
      00010F                        509 00101$:
      00010F BF 64 00         [24]  510 	cjne	r7,#0x64,00121$
      000112                        511 00121$:
      000112 50 F9            [24]  512 	jnc	00105$
                                    513 ;	music_mum.c:68: k=sszymmh[i]+7*sszymmh[i+1]-1;
      000114 EF               [12]  514 	mov	a,r7
      000115 90 01 77         [24]  515 	mov	dptr,#_sszymmh
      000118 93               [24]  516 	movc	a,@a+dptr
      000119 FE               [12]  517 	mov	r6,a
      00011A 8F 05            [24]  518 	mov	ar5,r7
      00011C ED               [12]  519 	mov	a,r5
      00011D 04               [12]  520 	inc	a
      00011E FC               [12]  521 	mov	r4,a
      00011F 33               [12]  522 	rlc	a
      000120 95 E0            [12]  523 	subb	a,acc
      000122 FB               [12]  524 	mov	r3,a
      000123 EC               [12]  525 	mov	a,r4
      000124 24 77            [12]  526 	add	a,#_sszymmh
      000126 F5 82            [12]  527 	mov	dpl,a
      000128 EB               [12]  528 	mov	a,r3
      000129 34 01            [12]  529 	addc	a,#(_sszymmh >> 8)
      00012B F5 83            [12]  530 	mov	dph,a
      00012D E4               [12]  531 	clr	a
      00012E 93               [24]  532 	movc	a,@a+dptr
      00012F 75 F0 07         [24]  533 	mov	b,#0x07
      000132 A4               [48]  534 	mul	ab
      000133 2E               [12]  535 	add	a,r6
      000134 14               [12]  536 	dec	a
                                    537 ;	music_mum.c:69: timer0h=FREQH[k];
      000135 FE               [12]  538 	mov	r6,a
      000136 90 01 DD         [24]  539 	mov	dptr,#_FREQH
      000139 93               [24]  540 	movc	a,@a+dptr
      00013A F5 08            [12]  541 	mov	_timer0h,a
                                    542 ;	music_mum.c:70: timer0l=FREQL[k];
      00013C EE               [12]  543 	mov	a,r6
      00013D 90 01 F9         [24]  544 	mov	dptr,#_FREQL
      000140 93               [24]  545 	movc	a,@a+dptr
      000141 F5 09            [12]  546 	mov	_timer0l,a
                                    547 ;	music_mum.c:71: time=sszymmh[i+2];
      000143 74 02            [12]  548 	mov	a,#0x02
      000145 2D               [12]  549 	add	a,r5
      000146 FE               [12]  550 	mov	r6,a
      000147 33               [12]  551 	rlc	a
      000148 95 E0            [12]  552 	subb	a,acc
      00014A FC               [12]  553 	mov	r4,a
      00014B EE               [12]  554 	mov	a,r6
      00014C 24 77            [12]  555 	add	a,#_sszymmh
      00014E F5 82            [12]  556 	mov	dpl,a
      000150 EC               [12]  557 	mov	a,r4
      000151 34 01            [12]  558 	addc	a,#(_sszymmh >> 8)
      000153 F5 83            [12]  559 	mov	dph,a
      000155 E4               [12]  560 	clr	a
      000156 93               [24]  561 	movc	a,@a+dptr
      000157 F5 0A            [12]  562 	mov	_time,a
                                    563 ;	music_mum.c:72: i=i+3;
      000159 74 03            [12]  564 	mov	a,#0x03
      00015B 2D               [12]  565 	add	a,r5
      00015C FF               [12]  566 	mov	r7,a
                                    567 ;	music_mum.c:73: song();
      00015D C0 07            [24]  568 	push	ar7
      00015F 12 00 F8         [24]  569 	lcall	_song
      000162 D0 07            [24]  570 	pop	ar7
                                    571 ;	music_mum.c:76: }
      000164 80 A9            [24]  572 	sjmp	00101$
                                    573 ;------------------------------------------------------------
                                    574 ;Allocation info for local variables in function 't0int'
                                    575 ;------------------------------------------------------------
                                    576 ;	music_mum.c:78: void t0int() __interrupt 1		//定时器中断函数
                                    577 ;	-----------------------------------------
                                    578 ;	 function t0int
                                    579 ;	-----------------------------------------
      000166                        580 _t0int:
                                    581 ;	music_mum.c:80: TR0=0;
                                    582 ;	assignBit
      000166 C2 8C            [12]  583 	clr	_TR0
                                    584 ;	music_mum.c:81: beep = !beep;//beep =~beep;
      000168 B2 B7            [12]  585 	cpl	_P3_7
                                    586 ;	music_mum.c:82: TH0=timer0h;
      00016A 85 08 8C         [24]  587 	mov	_TH0,_timer0h
                                    588 ;	music_mum.c:83: TL0=timer0l;
      00016D 85 09 8A         [24]  589 	mov	_TL0,_timer0l
                                    590 ;	music_mum.c:84: TR0=1;
                                    591 ;	assignBit
      000170 D2 8C            [12]  592 	setb	_TR0
                                    593 ;	music_mum.c:85: }
      000172 32               [24]  594 	reti
                                    595 ;	eliminated unneeded mov psw,# (no regs used in bank)
                                    596 ;	eliminated unneeded push/pop psw
                                    597 ;	eliminated unneeded push/pop dpl
                                    598 ;	eliminated unneeded push/pop dph
                                    599 ;	eliminated unneeded push/pop b
                                    600 ;	eliminated unneeded push/pop acc
                                    601 	.area CSEG    (CODE)
                                    602 	.area CONST   (CODE)
      000177                        603 _sszymmh:
      000177 06                     604 	.db #0x06	; 6
      000178 02                     605 	.db #0x02	; 2
      000179 03                     606 	.db #0x03	; 3
      00017A 05                     607 	.db #0x05	; 5
      00017B 02                     608 	.db #0x02	; 2
      00017C 01                     609 	.db #0x01	; 1
      00017D 03                     610 	.db #0x03	; 3
      00017E 02                     611 	.db #0x02	; 2
      00017F 02                     612 	.db #0x02	; 2
      000180 05                     613 	.db #0x05	; 5
      000181 02                     614 	.db #0x02	; 2
      000182 02                     615 	.db #0x02	; 2
      000183 01                     616 	.db #0x01	; 1
      000184 03                     617 	.db #0x03	; 3
      000185 02                     618 	.db #0x02	; 2
      000186 06                     619 	.db #0x06	; 6
      000187 02                     620 	.db #0x02	; 2
      000188 01                     621 	.db #0x01	; 1
      000189 05                     622 	.db #0x05	; 5
      00018A 02                     623 	.db #0x02	; 2
      00018B 01                     624 	.db #0x01	; 1
      00018C 06                     625 	.db #0x06	; 6
      00018D 02                     626 	.db #0x02	; 2
      00018E 04                     627 	.db #0x04	; 4
      00018F 03                     628 	.db #0x03	; 3
      000190 02                     629 	.db #0x02	; 2
      000191 02                     630 	.db #0x02	; 2
      000192 05                     631 	.db #0x05	; 5
      000193 02                     632 	.db #0x02	; 2
      000194 01                     633 	.db #0x01	; 1
      000195 06                     634 	.db #0x06	; 6
      000196 02                     635 	.db #0x02	; 2
      000197 01                     636 	.db #0x01	; 1
      000198 05                     637 	.db #0x05	; 5
      000199 02                     638 	.db #0x02	; 2
      00019A 02                     639 	.db #0x02	; 2
      00019B 03                     640 	.db #0x03	; 3
      00019C 02                     641 	.db #0x02	; 2
      00019D 02                     642 	.db #0x02	; 2
      00019E 01                     643 	.db #0x01	; 1
      00019F 02                     644 	.db #0x02	; 2
      0001A0 01                     645 	.db #0x01	; 1
      0001A1 06                     646 	.db #0x06	; 6
      0001A2 01                     647 	.db #0x01	; 1
      0001A3 01                     648 	.db #0x01	; 1
      0001A4 05                     649 	.db #0x05	; 5
      0001A5 02                     650 	.db #0x02	; 2
      0001A6 01                     651 	.db #0x01	; 1
      0001A7 03                     652 	.db #0x03	; 3
      0001A8 02                     653 	.db #0x02	; 2
      0001A9 01                     654 	.db #0x01	; 1
      0001AA 02                     655 	.db #0x02	; 2
      0001AB 02                     656 	.db #0x02	; 2
      0001AC 04                     657 	.db #0x04	; 4
      0001AD 02                     658 	.db #0x02	; 2
      0001AE 02                     659 	.db #0x02	; 2
      0001AF 03                     660 	.db #0x03	; 3
      0001B0 03                     661 	.db #0x03	; 3
      0001B1 02                     662 	.db #0x02	; 2
      0001B2 01                     663 	.db #0x01	; 1
      0001B3 05                     664 	.db #0x05	; 5
      0001B4 02                     665 	.db #0x02	; 2
      0001B5 02                     666 	.db #0x02	; 2
      0001B6 05                     667 	.db #0x05	; 5
      0001B7 02                     668 	.db #0x02	; 2
      0001B8 01                     669 	.db #0x01	; 1
      0001B9 06                     670 	.db #0x06	; 6
      0001BA 02                     671 	.db #0x02	; 2
      0001BB 01                     672 	.db #0x01	; 1
      0001BC 03                     673 	.db #0x03	; 3
      0001BD 02                     674 	.db #0x02	; 2
      0001BE 02                     675 	.db #0x02	; 2
      0001BF 02                     676 	.db #0x02	; 2
      0001C0 02                     677 	.db #0x02	; 2
      0001C1 02                     678 	.db #0x02	; 2
      0001C2 01                     679 	.db #0x01	; 1
      0001C3 02                     680 	.db #0x02	; 2
      0001C4 04                     681 	.db #0x04	; 4
      0001C5 05                     682 	.db #0x05	; 5
      0001C6 02                     683 	.db #0x02	; 2
      0001C7 03                     684 	.db #0x03	; 3
      0001C8 03                     685 	.db #0x03	; 3
      0001C9 02                     686 	.db #0x02	; 2
      0001CA 01                     687 	.db #0x01	; 1
      0001CB 02                     688 	.db #0x02	; 2
      0001CC 02                     689 	.db #0x02	; 2
      0001CD 01                     690 	.db #0x01	; 1
      0001CE 01                     691 	.db #0x01	; 1
      0001CF 02                     692 	.db #0x02	; 2
      0001D0 01                     693 	.db #0x01	; 1
      0001D1 06                     694 	.db #0x06	; 6
      0001D2 01                     695 	.db #0x01	; 1
      0001D3 01                     696 	.db #0x01	; 1
      0001D4 01                     697 	.db #0x01	; 1
      0001D5 02                     698 	.db #0x02	; 2
      0001D6 01                     699 	.db #0x01	; 1
      0001D7 05                     700 	.db #0x05	; 5
      0001D8 01                     701 	.db #0x01	; 1
      0001D9 06                     702 	.db #0x06	; 6
      0001DA 00                     703 	.db #0x00	; 0
      0001DB 00                     704 	.db #0x00	; 0
      0001DC 00                     705 	.db #0x00	; 0
      0001DD                        706 _FREQH:
      0001DD F2                     707 	.db #0xf2	; 242
      0001DE F3                     708 	.db #0xf3	; 243
      0001DF F5                     709 	.db #0xf5	; 245
      0001E0 F5                     710 	.db #0xf5	; 245
      0001E1 F6                     711 	.db #0xf6	; 246
      0001E2 F7                     712 	.db #0xf7	; 247
      0001E3 F8                     713 	.db #0xf8	; 248
      0001E4 F9                     714 	.db #0xf9	; 249
      0001E5 F9                     715 	.db #0xf9	; 249
      0001E6 FA                     716 	.db #0xfa	; 250
      0001E7 FA                     717 	.db #0xfa	; 250
      0001E8 FB                     718 	.db #0xfb	; 251
      0001E9 FB                     719 	.db #0xfb	; 251
      0001EA FC                     720 	.db #0xfc	; 252
      0001EB FC                     721 	.db #0xfc	; 252
      0001EC FC                     722 	.db #0xfc	; 252
      0001ED FD                     723 	.db #0xfd	; 253
      0001EE FD                     724 	.db #0xfd	; 253
      0001EF FD                     725 	.db #0xfd	; 253
      0001F0 FD                     726 	.db #0xfd	; 253
      0001F1 FE                     727 	.db #0xfe	; 254
      0001F2 FE                     728 	.db #0xfe	; 254
      0001F3 FE                     729 	.db #0xfe	; 254
      0001F4 FE                     730 	.db #0xfe	; 254
      0001F5 FE                     731 	.db #0xfe	; 254
      0001F6 FE                     732 	.db #0xfe	; 254
      0001F7 FE                     733 	.db #0xfe	; 254
      0001F8 FF                     734 	.db #0xff	; 255
      0001F9                        735 _FREQL:
      0001F9 42                     736 	.db #0x42	; 66	'B'
      0001FA C1                     737 	.db #0xc1	; 193
      0001FB 17                     738 	.db #0x17	; 23
      0001FC B6                     739 	.db #0xb6	; 182
      0001FD D0                     740 	.db #0xd0	; 208
      0001FE D1                     741 	.db #0xd1	; 209
      0001FF B6                     742 	.db #0xb6	; 182
      000200 21                     743 	.db #0x21	; 33
      000201 E1                     744 	.db #0xe1	; 225
      000202 8C                     745 	.db #0x8c	; 140
      000203 D8                     746 	.db #0xd8	; 216
      000204 68                     747 	.db #0x68	; 104	'h'
      000205 E9                     748 	.db #0xe9	; 233
      000206 5B                     749 	.db #0x5b	; 91
      000207 8F                     750 	.db #0x8f	; 143
      000208 EE                     751 	.db #0xee	; 238
      000209 44                     752 	.db #0x44	; 68	'D'
      00020A 6B                     753 	.db #0x6b	; 107	'k'
      00020B B4                     754 	.db #0xb4	; 180
      00020C F4                     755 	.db #0xf4	; 244
      00020D 2D                     756 	.db #0x2d	; 45
      00020E 47                     757 	.db #0x47	; 71	'G'
      00020F 77                     758 	.db #0x77	; 119	'w'
      000210 A2                     759 	.db #0xa2	; 162
      000211 B6                     760 	.db #0xb6	; 182
      000212 DA                     761 	.db #0xda	; 218
      000213 FA                     762 	.db #0xfa	; 250
      000214 16                     763 	.db #0x16	; 22
                                    764 	.area XINIT   (CODE)
                                    765 	.area CABS    (ABS,CODE)
