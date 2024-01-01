                                      1 ;--------------------------------------------------------
                                      2 ; File Created by SDCC : free open source ANSI-C Compiler
                                      3 ; Version 3.8.0 #10562 (Linux)
                                      4 ;--------------------------------------------------------
                                      5 	.module ps2_cmd_convert
                                      6 	.optsdcc -mmcs51 --model-small
                                      7 	
                                      8 ;--------------------------------------------------------
                                      9 ; Public variables in this module
                                     10 ;--------------------------------------------------------
                                     11 	.globl _command_map
                                     12 	.globl _main
                                     13 	.globl _delay
                                     14 	.globl _convert_commands
                                     15 	.globl _CY
                                     16 	.globl _AC
                                     17 	.globl _F0
                                     18 	.globl _RS1
                                     19 	.globl _RS0
                                     20 	.globl _OV
                                     21 	.globl _F1
                                     22 	.globl _P
                                     23 	.globl _PS
                                     24 	.globl _PT1
                                     25 	.globl _PX1
                                     26 	.globl _PT0
                                     27 	.globl _PX0
                                     28 	.globl _RD
                                     29 	.globl _WR
                                     30 	.globl _T1
                                     31 	.globl _T0
                                     32 	.globl _INT1
                                     33 	.globl _INT0
                                     34 	.globl _TXD
                                     35 	.globl _RXD
                                     36 	.globl _P3_7
                                     37 	.globl _P3_6
                                     38 	.globl _P3_5
                                     39 	.globl _P3_4
                                     40 	.globl _P3_3
                                     41 	.globl _P3_2
                                     42 	.globl _P3_1
                                     43 	.globl _P3_0
                                     44 	.globl _EA
                                     45 	.globl _ES
                                     46 	.globl _ET1
                                     47 	.globl _EX1
                                     48 	.globl _ET0
                                     49 	.globl _EX0
                                     50 	.globl _P2_7
                                     51 	.globl _P2_6
                                     52 	.globl _P2_5
                                     53 	.globl _P2_4
                                     54 	.globl _P2_3
                                     55 	.globl _P2_2
                                     56 	.globl _P2_1
                                     57 	.globl _P2_0
                                     58 	.globl _SM0
                                     59 	.globl _SM1
                                     60 	.globl _SM2
                                     61 	.globl _REN
                                     62 	.globl _TB8
                                     63 	.globl _RB8
                                     64 	.globl _TI
                                     65 	.globl _RI
                                     66 	.globl _P1_7
                                     67 	.globl _P1_6
                                     68 	.globl _P1_5
                                     69 	.globl _P1_4
                                     70 	.globl _P1_3
                                     71 	.globl _P1_2
                                     72 	.globl _P1_1
                                     73 	.globl _P1_0
                                     74 	.globl _TF1
                                     75 	.globl _TR1
                                     76 	.globl _TF0
                                     77 	.globl _TR0
                                     78 	.globl _IE1
                                     79 	.globl _IT1
                                     80 	.globl _IE0
                                     81 	.globl _IT0
                                     82 	.globl _P0_7
                                     83 	.globl _P0_6
                                     84 	.globl _P0_5
                                     85 	.globl _P0_4
                                     86 	.globl _P0_3
                                     87 	.globl _P0_2
                                     88 	.globl _P0_1
                                     89 	.globl _P0_0
                                     90 	.globl _B
                                     91 	.globl _ACC
                                     92 	.globl _PSW
                                     93 	.globl _IP
                                     94 	.globl _P3
                                     95 	.globl _IE
                                     96 	.globl _P2
                                     97 	.globl _SBUF
                                     98 	.globl _SCON
                                     99 	.globl _P1
                                    100 	.globl _TH1
                                    101 	.globl _TH0
                                    102 	.globl _TL1
                                    103 	.globl _TL0
                                    104 	.globl _TMOD
                                    105 	.globl _TCON
                                    106 	.globl _PCON
                                    107 	.globl _DPH
                                    108 	.globl _DPL
                                    109 	.globl _SP
                                    110 	.globl _P0
                                    111 	.globl _out
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
      000008                        227 _out::
      000008                        228 	.ds 9
                                    229 ;--------------------------------------------------------
                                    230 ; overlayable items in internal ram 
                                    231 ;--------------------------------------------------------
                                    232 	.area	OSEG    (OVR,DATA)
      000011                        233 _convert_commands_commands_65536_1:
      000011                        234 	.ds 16
      000021                        235 _convert_commands_i_131072_2:
      000021                        236 	.ds 1
                                    237 	.area	OSEG    (OVR,DATA)
                                    238 ;--------------------------------------------------------
                                    239 ; Stack segment in internal ram 
                                    240 ;--------------------------------------------------------
                                    241 	.area	SSEG
      000022                        242 __start__stack:
      000022                        243 	.ds	1
                                    244 
                                    245 ;--------------------------------------------------------
                                    246 ; indirectly addressable internal ram data
                                    247 ;--------------------------------------------------------
                                    248 	.area ISEG    (DATA)
                                    249 ;--------------------------------------------------------
                                    250 ; absolute internal ram data
                                    251 ;--------------------------------------------------------
                                    252 	.area IABS    (ABS,DATA)
                                    253 	.area IABS    (ABS,DATA)
                                    254 ;--------------------------------------------------------
                                    255 ; bit data
                                    256 ;--------------------------------------------------------
                                    257 	.area BSEG    (BIT)
                                    258 ;--------------------------------------------------------
                                    259 ; paged external ram data
                                    260 ;--------------------------------------------------------
                                    261 	.area PSEG    (PAG,XDATA)
                                    262 ;--------------------------------------------------------
                                    263 ; external ram data
                                    264 ;--------------------------------------------------------
                                    265 	.area XSEG    (XDATA)
                                    266 ;--------------------------------------------------------
                                    267 ; absolute external ram data
                                    268 ;--------------------------------------------------------
                                    269 	.area XABS    (ABS,XDATA)
                                    270 ;--------------------------------------------------------
                                    271 ; external initialized ram data
                                    272 ;--------------------------------------------------------
                                    273 	.area XISEG   (XDATA)
                                    274 	.area HOME    (CODE)
                                    275 	.area GSINIT0 (CODE)
                                    276 	.area GSINIT1 (CODE)
                                    277 	.area GSINIT2 (CODE)
                                    278 	.area GSINIT3 (CODE)
                                    279 	.area GSINIT4 (CODE)
                                    280 	.area GSINIT5 (CODE)
                                    281 	.area GSINIT  (CODE)
                                    282 	.area GSFINAL (CODE)
                                    283 	.area CSEG    (CODE)
                                    284 ;--------------------------------------------------------
                                    285 ; interrupt vector 
                                    286 ;--------------------------------------------------------
                                    287 	.area HOME    (CODE)
      000000                        288 __interrupt_vect:
      000000 02 00 06         [24]  289 	ljmp	__sdcc_gsinit_startup
                                    290 ;--------------------------------------------------------
                                    291 ; global & static initialisations
                                    292 ;--------------------------------------------------------
                                    293 	.area HOME    (CODE)
                                    294 	.area GSINIT  (CODE)
                                    295 	.area GSFINAL (CODE)
                                    296 	.area GSINIT  (CODE)
                                    297 	.globl __sdcc_gsinit_startup
                                    298 	.globl __sdcc_program_startup
                                    299 	.globl __start__stack
                                    300 	.globl __mcs51_genXINIT
                                    301 	.globl __mcs51_genXRAMCLEAR
                                    302 	.globl __mcs51_genRAMCLEAR
                                    303 ;	ps2_cmd_convert.c:51: uchar out[9] = {0, 0, 0, 0xDF, 0xDF, 0, 0, 0, 0};
      00005F 75 08 00         [24]  304 	mov	_out,#0x00
      000062 75 09 00         [24]  305 	mov	(_out + 0x0001),#0x00
      000065 75 0A 00         [24]  306 	mov	(_out + 0x0002),#0x00
      000068 75 0B DF         [24]  307 	mov	(_out + 0x0003),#0xdf
      00006B 75 0C DF         [24]  308 	mov	(_out + 0x0004),#0xdf
      00006E 75 0D 00         [24]  309 	mov	(_out + 0x0005),#0x00
      000071 75 0E 00         [24]  310 	mov	(_out + 0x0006),#0x00
      000074 75 0F 00         [24]  311 	mov	(_out + 0x0007),#0x00
      000077 75 10 00         [24]  312 	mov	(_out + 0x0008),#0x00
                                    313 	.area GSFINAL (CODE)
      00007A 02 00 03         [24]  314 	ljmp	__sdcc_program_startup
                                    315 ;--------------------------------------------------------
                                    316 ; Home
                                    317 ;--------------------------------------------------------
                                    318 	.area HOME    (CODE)
                                    319 	.area HOME    (CODE)
      000003                        320 __sdcc_program_startup:
      000003 02 01 13         [24]  321 	ljmp	_main
                                    322 ;	return from main will return to caller
                                    323 ;--------------------------------------------------------
                                    324 ; code
                                    325 ;--------------------------------------------------------
                                    326 	.area CSEG    (CODE)
                                    327 ;------------------------------------------------------------
                                    328 ;Allocation info for local variables in function 'convert_commands'
                                    329 ;------------------------------------------------------------
                                    330 ;commands                  Allocated with name '_convert_commands_commands_65536_1'
                                    331 ;i                         Allocated with name '_convert_commands_i_131072_2'
                                    332 ;------------------------------------------------------------
                                    333 ;	ps2_cmd_convert.c:53: uint *convert_commands()
                                    334 ;	-----------------------------------------
                                    335 ;	 function convert_commands
                                    336 ;	-----------------------------------------
      00007D                        337 _convert_commands:
                           000007   338 	ar7 = 0x07
                           000006   339 	ar6 = 0x06
                           000005   340 	ar5 = 0x05
                           000004   341 	ar4 = 0x04
                           000003   342 	ar3 = 0x03
                           000002   343 	ar2 = 0x02
                           000001   344 	ar1 = 0x01
                           000000   345 	ar0 = 0x00
                                    346 ;	ps2_cmd_convert.c:56: for (char i = 0; i < COMMANDS_LENGTH; i++)
      00007D 75 21 00         [24]  347 	mov	_convert_commands_i_131072_2,#0x00
      000080                        348 00105$:
      000080 74 F8            [12]  349 	mov	a,#0x100 - 0x08
      000082 25 21            [12]  350 	add	a,_convert_commands_i_131072_2
      000084 40 5F            [24]  351 	jc	00103$
                                    352 ;	ps2_cmd_convert.c:58: if (out[command_map[i][0]] == command_map[i][1])
      000086 E5 21            [12]  353 	mov	a,_convert_commands_i_131072_2
      000088 75 F0 06         [24]  354 	mov	b,#0x06
      00008B A4               [48]  355 	mul	ab
      00008C FD               [12]  356 	mov	r5,a
      00008D AE F0            [24]  357 	mov	r6,b
      00008F 24 B1            [12]  358 	add	a,#_command_map
      000091 FB               [12]  359 	mov	r3,a
      000092 EE               [12]  360 	mov	a,r6
      000093 34 01            [12]  361 	addc	a,#(_command_map >> 8)
      000095 FC               [12]  362 	mov	r4,a
      000096 8B 82            [24]  363 	mov	dpl,r3
      000098 8C 83            [24]  364 	mov	dph,r4
      00009A E4               [12]  365 	clr	a
      00009B 93               [24]  366 	movc	a,@a+dptr
      00009C FA               [12]  367 	mov	r2,a
      00009D A3               [24]  368 	inc	dptr
      00009E E4               [12]  369 	clr	a
      00009F 93               [24]  370 	movc	a,@a+dptr
      0000A0 EA               [12]  371 	mov	a,r2
      0000A1 24 08            [12]  372 	add	a,#_out
      0000A3 F9               [12]  373 	mov	r1,a
      0000A4 87 07            [24]  374 	mov	ar7,@r1
      0000A6 8B 82            [24]  375 	mov	dpl,r3
      0000A8 8C 83            [24]  376 	mov	dph,r4
      0000AA A3               [24]  377 	inc	dptr
      0000AB A3               [24]  378 	inc	dptr
      0000AC E4               [12]  379 	clr	a
      0000AD 93               [24]  380 	movc	a,@a+dptr
      0000AE FB               [12]  381 	mov	r3,a
      0000AF A3               [24]  382 	inc	dptr
      0000B0 E4               [12]  383 	clr	a
      0000B1 93               [24]  384 	movc	a,@a+dptr
      0000B2 FC               [12]  385 	mov	r4,a
      0000B3 7A 00            [12]  386 	mov	r2,#0x00
      0000B5 EF               [12]  387 	mov	a,r7
      0000B6 B5 03 28         [24]  388 	cjne	a,ar3,00106$
      0000B9 EA               [12]  389 	mov	a,r2
      0000BA B5 04 24         [24]  390 	cjne	a,ar4,00106$
                                    391 ;	ps2_cmd_convert.c:60: commands[i] = command_map[i][2];
      0000BD E5 21            [12]  392 	mov	a,_convert_commands_i_131072_2
      0000BF 25 21            [12]  393 	add	a,_convert_commands_i_131072_2
      0000C1 24 11            [12]  394 	add	a,#_convert_commands_commands_65536_1
      0000C3 F9               [12]  395 	mov	r1,a
      0000C4 ED               [12]  396 	mov	a,r5
      0000C5 24 B1            [12]  397 	add	a,#_command_map
      0000C7 FD               [12]  398 	mov	r5,a
      0000C8 EE               [12]  399 	mov	a,r6
      0000C9 34 01            [12]  400 	addc	a,#(_command_map >> 8)
      0000CB FE               [12]  401 	mov	r6,a
      0000CC 8D 82            [24]  402 	mov	dpl,r5
      0000CE 8E 83            [24]  403 	mov	dph,r6
      0000D0 A3               [24]  404 	inc	dptr
      0000D1 A3               [24]  405 	inc	dptr
      0000D2 A3               [24]  406 	inc	dptr
      0000D3 A3               [24]  407 	inc	dptr
      0000D4 E4               [12]  408 	clr	a
      0000D5 93               [24]  409 	movc	a,@a+dptr
      0000D6 FE               [12]  410 	mov	r6,a
      0000D7 A3               [24]  411 	inc	dptr
      0000D8 E4               [12]  412 	clr	a
      0000D9 93               [24]  413 	movc	a,@a+dptr
      0000DA FF               [12]  414 	mov	r7,a
      0000DB A7 06            [24]  415 	mov	@r1,ar6
      0000DD 09               [12]  416 	inc	r1
      0000DE A7 07            [24]  417 	mov	@r1,ar7
      0000E0 19               [12]  418 	dec	r1
      0000E1                        419 00106$:
                                    420 ;	ps2_cmd_convert.c:56: for (char i = 0; i < COMMANDS_LENGTH; i++)
      0000E1 05 21            [12]  421 	inc	_convert_commands_i_131072_2
      0000E3 80 9B            [24]  422 	sjmp	00105$
      0000E5                        423 00103$:
                                    424 ;	ps2_cmd_convert.c:63: return commands;
      0000E5 90 00 11         [24]  425 	mov	dptr,#_convert_commands_commands_65536_1
      0000E8 75 F0 40         [24]  426 	mov	b,#0x40
                                    427 ;	ps2_cmd_convert.c:64: }
      0000EB 22               [24]  428 	ret
                                    429 ;------------------------------------------------------------
                                    430 ;Allocation info for local variables in function 'delay'
                                    431 ;------------------------------------------------------------
                                    432 ;i                         Allocated to registers r6 r7 
                                    433 ;j                         Allocated to registers r4 r5 
                                    434 ;------------------------------------------------------------
                                    435 ;	ps2_cmd_convert.c:66: void delay() {
                                    436 ;	-----------------------------------------
                                    437 ;	 function delay
                                    438 ;	-----------------------------------------
      0000EC                        439 _delay:
                                    440 ;	ps2_cmd_convert.c:68: for (i=0; i < 100; i++){
      0000EC 7E 00            [12]  441 	mov	r6,#0x00
      0000EE 7F 00            [12]  442 	mov	r7,#0x00
      0000F0                        443 00106$:
                                    444 ;	ps2_cmd_convert.c:69: for (j=0; j < 100; j++) {
      0000F0 7C 64            [12]  445 	mov	r4,#0x64
      0000F2 7D 00            [12]  446 	mov	r5,#0x00
      0000F4                        447 00105$:
      0000F4 EC               [12]  448 	mov	a,r4
      0000F5 24 FF            [12]  449 	add	a,#0xff
      0000F7 FA               [12]  450 	mov	r2,a
      0000F8 ED               [12]  451 	mov	a,r5
      0000F9 34 FF            [12]  452 	addc	a,#0xff
      0000FB FB               [12]  453 	mov	r3,a
      0000FC 8A 04            [24]  454 	mov	ar4,r2
      0000FE 8B 05            [24]  455 	mov	ar5,r3
      000100 EA               [12]  456 	mov	a,r2
      000101 4B               [12]  457 	orl	a,r3
      000102 70 F0            [24]  458 	jnz	00105$
                                    459 ;	ps2_cmd_convert.c:68: for (i=0; i < 100; i++){
      000104 0E               [12]  460 	inc	r6
      000105 BE 00 01         [24]  461 	cjne	r6,#0x00,00124$
      000108 0F               [12]  462 	inc	r7
      000109                        463 00124$:
      000109 C3               [12]  464 	clr	c
      00010A EE               [12]  465 	mov	a,r6
      00010B 94 64            [12]  466 	subb	a,#0x64
      00010D EF               [12]  467 	mov	a,r7
      00010E 94 00            [12]  468 	subb	a,#0x00
      000110 40 DE            [24]  469 	jc	00106$
                                    470 ;	ps2_cmd_convert.c:73: }
      000112 22               [24]  471 	ret
                                    472 ;------------------------------------------------------------
                                    473 ;Allocation info for local variables in function 'main'
                                    474 ;------------------------------------------------------------
                                    475 ;commands                  Allocated to registers r5 r6 r7 
                                    476 ;i                         Allocated to registers r4 
                                    477 ;------------------------------------------------------------
                                    478 ;	ps2_cmd_convert.c:75: void main(){
                                    479 ;	-----------------------------------------
                                    480 ;	 function main
                                    481 ;	-----------------------------------------
      000113                        482 _main:
                                    483 ;	ps2_cmd_convert.c:76: uint *commands = convert_commands();
      000113 12 00 7D         [24]  484 	lcall	_convert_commands
      000116 AD 82            [24]  485 	mov	r5,dpl
      000118 AE 83            [24]  486 	mov	r6,dph
      00011A AF F0            [24]  487 	mov	r7,b
                                    488 ;	ps2_cmd_convert.c:77: for (char i = 0; i < COMMANDS_LENGTH; i++) 
      00011C 7C 00            [12]  489 	mov	r4,#0x00
      00011E                        490 00115$:
      00011E BC 08 00         [24]  491 	cjne	r4,#0x08,00136$
      000121                        492 00136$:
      000121 40 03            [24]  493 	jc	00137$
      000123 02 01 8F         [24]  494 	ljmp	00112$
      000126                        495 00137$:
                                    496 ;	ps2_cmd_convert.c:79: switch (commands[i])
      000126 EC               [12]  497 	mov	a,r4
      000127 75 F0 02         [24]  498 	mov	b,#0x02
      00012A A4               [48]  499 	mul	ab
      00012B 2D               [12]  500 	add	a,r5
      00012C FA               [12]  501 	mov	r2,a
      00012D EE               [12]  502 	mov	a,r6
      00012E 35 F0            [12]  503 	addc	a,b
      000130 F9               [12]  504 	mov	r1,a
      000131 8F 03            [24]  505 	mov	ar3,r7
      000133 8A 82            [24]  506 	mov	dpl,r2
      000135 89 83            [24]  507 	mov	dph,r1
      000137 8B F0            [24]  508 	mov	b,r3
      000139 12 01 91         [24]  509 	lcall	__gptrget
      00013C FA               [12]  510 	mov	r2,a
      00013D A3               [24]  511 	inc	dptr
      00013E 12 01 91         [24]  512 	lcall	__gptrget
      000141 FB               [12]  513 	mov	r3,a
      000142 C3               [12]  514 	clr	c
      000143 74 08            [12]  515 	mov	a,#0x08
      000145 9A               [12]  516 	subb	a,r2
      000146 E4               [12]  517 	clr	a
      000147 9B               [12]  518 	subb	a,r3
      000148 50 03            [24]  519 	jnc	00138$
      00014A 02 01 8B         [24]  520 	ljmp	00116$
      00014D                        521 00138$:
      00014D EA               [12]  522 	mov	a,r2
      00014E 24 0A            [12]  523 	add	a,#(00139$-3-.)
      000150 83               [24]  524 	movc	a,@a+pc
      000151 F5 82            [12]  525 	mov	dpl,a
      000153 EA               [12]  526 	mov	a,r2
      000154 24 0D            [12]  527 	add	a,#(00140$-3-.)
      000156 83               [24]  528 	movc	a,@a+pc
      000157 F5 83            [12]  529 	mov	dph,a
      000159 E4               [12]  530 	clr	a
      00015A 73               [24]  531 	jmp	@a+dptr
      00015B                        532 00139$:
      00015B 8B                     533 	.db	00116$
      00015C 6D                     534 	.db	00101$
      00015D 71                     535 	.db	00102$
      00015E 75                     536 	.db	00103$
      00015F 79                     537 	.db	00104$
      000160 7D                     538 	.db	00105$
      000161 81                     539 	.db	00106$
      000162 85                     540 	.db	00107$
      000163 89                     541 	.db	00108$
      000164                        542 00140$:
      000164 01                     543 	.db	00116$>>8
      000165 01                     544 	.db	00101$>>8
      000166 01                     545 	.db	00102$>>8
      000167 01                     546 	.db	00103$>>8
      000168 01                     547 	.db	00104$>>8
      000169 01                     548 	.db	00105$>>8
      00016A 01                     549 	.db	00106$>>8
      00016B 01                     550 	.db	00107$>>8
      00016C 01                     551 	.db	00108$>>8
                                    552 ;	ps2_cmd_convert.c:81: case COMMAND_LEFT_TOP:
      00016D                        553 00101$:
                                    554 ;	ps2_cmd_convert.c:82: P0_0 = 0;
                                    555 ;	assignBit
      00016D C2 80            [12]  556 	clr	_P0_0
                                    557 ;	ps2_cmd_convert.c:83: break;
                                    558 ;	ps2_cmd_convert.c:84: case COMMAND_LEFT_DOWN:
      00016F 80 1A            [24]  559 	sjmp	00116$
      000171                        560 00102$:
                                    561 ;	ps2_cmd_convert.c:85: P0_1 = 0;
                                    562 ;	assignBit
      000171 C2 81            [12]  563 	clr	_P0_1
                                    564 ;	ps2_cmd_convert.c:86: break;
                                    565 ;	ps2_cmd_convert.c:87: case COMMAND_LEFT_LEFT:
      000173 80 16            [24]  566 	sjmp	00116$
      000175                        567 00103$:
                                    568 ;	ps2_cmd_convert.c:88: P0_2 = 0;
                                    569 ;	assignBit
      000175 C2 82            [12]  570 	clr	_P0_2
                                    571 ;	ps2_cmd_convert.c:89: break;
                                    572 ;	ps2_cmd_convert.c:90: case COMMAND_LEFT_RIGHT:
      000177 80 12            [24]  573 	sjmp	00116$
      000179                        574 00104$:
                                    575 ;	ps2_cmd_convert.c:91: P0_3 = 0;
                                    576 ;	assignBit
      000179 C2 83            [12]  577 	clr	_P0_3
                                    578 ;	ps2_cmd_convert.c:92: break;
                                    579 ;	ps2_cmd_convert.c:93: case COMMAND_RIGHT_TOP:
      00017B 80 0E            [24]  580 	sjmp	00116$
      00017D                        581 00105$:
                                    582 ;	ps2_cmd_convert.c:94: P0_4 = 0;
                                    583 ;	assignBit
      00017D C2 84            [12]  584 	clr	_P0_4
                                    585 ;	ps2_cmd_convert.c:95: break;
                                    586 ;	ps2_cmd_convert.c:96: case COMMAND_RIGHT_RIGHT:
      00017F 80 0A            [24]  587 	sjmp	00116$
      000181                        588 00106$:
                                    589 ;	ps2_cmd_convert.c:97: P0_5 = 0;
                                    590 ;	assignBit
      000181 C2 85            [12]  591 	clr	_P0_5
                                    592 ;	ps2_cmd_convert.c:98: break;
                                    593 ;	ps2_cmd_convert.c:99: case COMMAND_RIGHT_LEFT:
      000183 80 06            [24]  594 	sjmp	00116$
      000185                        595 00107$:
                                    596 ;	ps2_cmd_convert.c:100: P0_6 = 0;
                                    597 ;	assignBit
      000185 C2 86            [12]  598 	clr	_P0_6
                                    599 ;	ps2_cmd_convert.c:101: break;
                                    600 ;	ps2_cmd_convert.c:102: case COMMAND_RIGHT_DOWN:
      000187 80 02            [24]  601 	sjmp	00116$
      000189                        602 00108$:
                                    603 ;	ps2_cmd_convert.c:103: P0_7 = 0;
                                    604 ;	assignBit
      000189 C2 87            [12]  605 	clr	_P0_7
                                    606 ;	ps2_cmd_convert.c:107: }
      00018B                        607 00116$:
                                    608 ;	ps2_cmd_convert.c:77: for (char i = 0; i < COMMANDS_LENGTH; i++) 
      00018B 0C               [12]  609 	inc	r4
      00018C 02 01 1E         [24]  610 	ljmp	00115$
                                    611 ;	ps2_cmd_convert.c:111: while(1) {
      00018F                        612 00112$:
                                    613 ;	ps2_cmd_convert.c:114: }
      00018F 80 FE            [24]  614 	sjmp	00112$
                                    615 	.area CSEG    (CODE)
                                    616 	.area CONST   (CODE)
      0001B1                        617 _command_map:
      0001B1 03 00                  618 	.byte #0x03,#0x00	; 3
      0001B3 EF 00                  619 	.byte #0xef,#0x00	; 239
      0001B5 01 00                  620 	.byte #0x01,#0x00	; 1
      0001B7 03 00                  621 	.byte #0x03,#0x00	; 3
      0001B9 BF 00                  622 	.byte #0xbf,#0x00	; 191
      0001BB 02 00                  623 	.byte #0x02,#0x00	; 2
      0001BD 03 00                  624 	.byte #0x03,#0x00	; 3
      0001BF 7F 00                  625 	.byte #0x7f,#0x00	; 127
      0001C1 03 00                  626 	.byte #0x03,#0x00	; 3
      0001C3 03 00                  627 	.byte #0x03,#0x00	; 3
      0001C5 DF 00                  628 	.byte #0xdf,#0x00	; 223
      0001C7 04 00                  629 	.byte #0x04,#0x00	; 4
      0001C9 04 00                  630 	.byte #0x04,#0x00	; 4
      0001CB EF 00                  631 	.byte #0xef,#0x00	; 239
      0001CD 05 00                  632 	.byte #0x05,#0x00	; 5
      0001CF 04 00                  633 	.byte #0x04,#0x00	; 4
      0001D1 BF 00                  634 	.byte #0xbf,#0x00	; 191
      0001D3 08 00                  635 	.byte #0x08,#0x00	; 8
      0001D5 04 00                  636 	.byte #0x04,#0x00	; 4
      0001D7 7F 00                  637 	.byte #0x7f,#0x00	; 127
      0001D9 07 00                  638 	.byte #0x07,#0x00	; 7
      0001DB 04 00                  639 	.byte #0x04,#0x00	; 4
      0001DD DF 00                  640 	.byte #0xdf,#0x00	; 223
      0001DF 06 00                  641 	.byte #0x06,#0x00	; 6
                                    642 	.area XINIT   (CODE)
                                    643 	.area CABS    (ABS,CODE)
