pqc_kyber::reference::polyvec::polyvec_ntt:
 push    rbp
 push    rbx
 lea     rax, [rdi, +, 256]
 lea     rcx, [rdi, +, 128]
 lea     rdx, [rdi, +, 48]
 lea     rsi, [rdi, +, 16]
 xor     r8d, r8d
 movdqa  xmm6, xmmword, ptr, [rip, +, .LCPI15_0]
 movdqa  xmm7, xmmword, ptr, [rip, +, .LCPI15_1]
 movdqa  xmm8, xmmword, ptr, [rip, +, .LCPI15_2]
 movdqa  xmm9, xmmword, ptr, [rip, +, .LCPI15_3]
 movdqa  xmm10, xmmword, ptr, [rip, +, .LCPI15_4]
 movdqa  xmm11, xmmword, ptr, [rip, +, .LCPI15_5]
 lea     r9, [rip, +, .L__unnamed_32]
 movdqa  xmm12, xmmword, ptr, [rip, +, .LCPI15_14]
 movdqa  xmm13, xmmword, ptr, [rip, +, .LCPI15_15]
 mov     r10, rdi
.LBB15_1:
 mov     r11, r8
 shl     r11, 9
 add     r11, rdi
 mov     rbx, -256
.LBB15_2:
 movdqu  xmm0, xmmword, ptr, [rax, +, rbx]
 movdqu  xmm2, xmmword, ptr, [rax, +, rbx, +, 256]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [rax, +, rbx, +, 256], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [rax, +, rbx], xmm3
 add     rbx, 16
 jne     .LBB15_2
 mov     rbx, -128
.LBB15_4:
 movdqu  xmm0, xmmword, ptr, [rcx, +, rbx]
 movdqu  xmm2, xmmword, ptr, [rcx, +, rbx, +, 128]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm8
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm8
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm8
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm8
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm9
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm9
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [rcx, +, rbx, +, 128], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [rcx, +, rbx], xmm3
 add     rbx, 16
 jne     .LBB15_4
 xor     ebx, ebx
.LBB15_6:
 movdqu  xmm0, xmmword, ptr, [rax, +, 2*rbx]
 movdqu  xmm2, xmmword, ptr, [rax, +, 2*rbx, +, 128]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm10
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm10
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm10
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm10
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm11
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm11
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [rax, +, 2*rbx, +, 128], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [rax, +, 2*rbx], xmm3
 add     rbx, 8
 cmp     rbx, 64
 jne     .LBB15_6
 movdqu  xmm1, xmmword, ptr, [r11, +, 64]
 punpckhwd xmm0, xmm1
 psrad   xmm0, 16
 punpcklwd xmm1, xmm1
 psrad   xmm1, 16
 movdqa  xmm2, xmm1
 movdqa  xmm8, xmmword, ptr, [rip, +, .LCPI15_6]
 pmuludq xmm2, xmm8
 pshufd  xmm2, xmm2, 232
 pshufd  xmm3, xmm1, 245
 pmuludq xmm3, xmm8
 pshufd  xmm3, xmm3, 232
 punpckldq xmm2, xmm3
 movdqa  xmm3, xmm0
 pmuludq xmm3, xmm8
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm0, 245
 pmuludq xmm4, xmm8
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 psrld   xmm3, 16
 psrld   xmm2, 16
 pshuflw xmm2, xmm2, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm4, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm4, xmm1
 movdqa  xmm9, xmmword, ptr, [rip, +, .LCPI15_7]
 pmaddwd xmm4, xmm9
 pshuflw xmm1, xmm3, 232
 pshufhw xmm1, xmm1, 232
 pshufd  xmm1, xmm1, 232
 packssdw xmm0, xmm0
 punpcklwd xmm1, xmm0
 pmaddwd xmm1, xmm9
 psrad   xmm1, 16
 psrad   xmm4, 16
 packssdw xmm4, xmm1
 movdqu  xmm3, xmmword, ptr, [r11]
 movdqu  xmm2, xmmword, ptr, [r11, +, 16]
 movdqu  xmm0, xmmword, ptr, [r11, +, 32]
 movdqu  xmm1, xmmword, ptr, [r11, +, 48]
 movdqa  xmm5, xmm3
 psubw   xmm5, xmm4
 movdqu  xmmword, ptr, [r11, +, 64], xmm5
 paddw   xmm4, xmm3
 movdqu  xmmword, ptr, [r11], xmm4
 movdqu  xmm4, xmmword, ptr, [r11, +, 80]
 punpckhwd xmm3, xmm4
 psrad   xmm3, 16
 punpcklwd xmm4, xmm4
 psrad   xmm4, 16
 movdqa  xmm5, xmm4
 pmuludq xmm5, xmm8
 pshufd  xmm5, xmm5, 232
 pshufd  xmm6, xmm4, 245
 pmuludq xmm6, xmm8
 pshufd  xmm6, xmm6, 232
 punpckldq xmm5, xmm6
 movdqa  xmm6, xmm3
 pmuludq xmm6, xmm8
 pshufd  xmm6, xmm6, 232
 pshufd  xmm7, xmm3, 245
 pmuludq xmm7, xmm8
 pshufd  xmm7, xmm7, 232
 punpckldq xmm6, xmm7
 psrld   xmm6, 16
 psrld   xmm5, 16
 pshuflw xmm5, xmm5, 232
 pshufhw xmm5, xmm5, 232
 pshufd  xmm5, xmm5, 232
 packssdw xmm4, xmm4
 punpcklwd xmm5, xmm4
 pmaddwd xmm5, xmm9
 pshuflw xmm4, xmm6, 232
 pshufhw xmm4, xmm4, 232
 pshufd  xmm4, xmm4, 232
 packssdw xmm3, xmm3
 punpcklwd xmm4, xmm3
 pmaddwd xmm4, xmm9
 psrad   xmm4, 16
 psrad   xmm5, 16
 packssdw xmm5, xmm4
 movdqa  xmm3, xmm2
 psubw   xmm3, xmm5
 movdqu  xmmword, ptr, [r11, +, 80], xmm3
 paddw   xmm5, xmm2
 movdqu  xmmword, ptr, [r11, +, 16], xmm5
 movdqu  xmm3, xmmword, ptr, [r11, +, 96]
 punpckhwd xmm2, xmm3
 psrad   xmm2, 16
 punpcklwd xmm3, xmm3
 psrad   xmm3, 16
 movdqa  xmm4, xmm3
 pmuludq xmm4, xmm8
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm3, 245
 pmuludq xmm5, xmm8
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 movdqa  xmm5, xmm2
 pmuludq xmm5, xmm8
 pshufd  xmm5, xmm5, 232
 pshufd  xmm6, xmm2, 245
 pmuludq xmm6, xmm8
 pshufd  xmm6, xmm6, 232
 punpckldq xmm5, xmm6
 psrld   xmm5, 16
 psrld   xmm4, 16
 pshuflw xmm4, xmm4, 232
 pshufhw xmm4, xmm4, 232
 pshufd  xmm4, xmm4, 232
 packssdw xmm3, xmm3
 punpcklwd xmm4, xmm3
 pmaddwd xmm4, xmm9
 pshuflw xmm3, xmm5, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm9
 psrad   xmm3, 16
 psrad   xmm4, 16
 packssdw xmm4, xmm3
 movdqa  xmm2, xmm0
 psubw   xmm2, xmm4
 movdqu  xmmword, ptr, [r11, +, 96], xmm2
 paddw   xmm4, xmm0
 movdqu  xmmword, ptr, [r11, +, 32], xmm4
 movdqu  xmm2, xmmword, ptr, [r11, +, 112]
 punpckhwd xmm0, xmm2
 psrad   xmm0, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm8
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm8
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm0
 pmuludq xmm4, xmm8
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm0, 245
 pmuludq xmm5, xmm8
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm9
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm0, xmm0
 punpcklwd xmm2, xmm0
 pmaddwd xmm2, xmm9
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm0, xmm1
 psubw   xmm0, xmm3
 movdqu  xmmword, ptr, [r11, +, 112], xmm0
 paddw   xmm3, xmm1
 movdqu  xmmword, ptr, [r11, +, 48], xmm3
 movdqu  xmm1, xmmword, ptr, [r11, +, 192]
 punpckhwd xmm0, xmm1
 psrad   xmm0, 16
 punpcklwd xmm1, xmm1
 psrad   xmm1, 16
 movdqa  xmm2, xmm1
 movdqa  xmm6, xmmword, ptr, [rip, +, .LCPI15_8]
 pmuludq xmm2, xmm6
 pshufd  xmm2, xmm2, 232
 pshufd  xmm3, xmm1, 245
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 punpckldq xmm2, xmm3
 movdqa  xmm3, xmm0
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm0, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 psrld   xmm3, 16
 psrld   xmm2, 16
 pshuflw xmm2, xmm2, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 movdqa  xmm7, xmmword, ptr, [rip, +, .LCPI15_9]
 pmaddwd xmm2, xmm7
 pshuflw xmm1, xmm3, 232
 pshufhw xmm1, xmm1, 232
 pshufd  xmm1, xmm1, 232
 packssdw xmm0, xmm0
 punpcklwd xmm1, xmm0
 pmaddwd xmm1, xmm7
 psrad   xmm1, 16
 psrad   xmm2, 16
 packssdw xmm2, xmm1
 movdqu  xmm0, xmmword, ptr, [r11, +, 128]
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm2
 movdqu  xmmword, ptr, [r11, +, 192], xmm1
 paddw   xmm2, xmm0
 movdqu  xmmword, ptr, [r11, +, 128], xmm2
 movdqu  xmm0, xmmword, ptr, [r11, +, 144]
 movdqu  xmm2, xmmword, ptr, [r11, +, 208]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 208], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 144], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 160]
 movdqu  xmm2, xmmword, ptr, [r11, +, 224]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 224], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 160], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 176]
 movdqu  xmm2, xmmword, ptr, [r11, +, 240]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 240], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 176], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 256]
 movdqu  xmm2, xmmword, ptr, [r11, +, 320]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 movdqa  xmm6, xmmword, ptr, [rip, +, .LCPI15_10]
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 movdqa  xmm7, xmmword, ptr, [rip, +, .LCPI15_11]
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 320], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 256], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 272]
 movdqu  xmm2, xmmword, ptr, [r11, +, 336]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 336], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 272], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 288]
 movdqu  xmm2, xmmword, ptr, [r11, +, 352]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 352], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 288], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 304]
 movdqu  xmm2, xmmword, ptr, [r11, +, 368]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 368], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 304], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 384]
 movdqu  xmm2, xmmword, ptr, [r11, +, 448]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 movdqa  xmm6, xmmword, ptr, [rip, +, .LCPI15_12]
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 movdqa  xmm7, xmmword, ptr, [rip, +, .LCPI15_13]
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 448], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 384], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 400]
 movdqu  xmm2, xmmword, ptr, [r11, +, 464]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 464], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 400], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 416]
 movdqu  xmm2, xmmword, ptr, [r11, +, 480]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 480], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 416], xmm3
 movdqu  xmm0, xmmword, ptr, [r11, +, 432]
 movdqu  xmm2, xmmword, ptr, [r11, +, 496]
 punpckhwd xmm1, xmm2
 psrad   xmm1, 16
 punpcklwd xmm2, xmm2
 psrad   xmm2, 16
 movdqa  xmm3, xmm2
 pmuludq xmm3, xmm6
 pshufd  xmm3, xmm3, 232
 pshufd  xmm4, xmm2, 245
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 punpckldq xmm3, xmm4
 movdqa  xmm4, xmm1
 pmuludq xmm4, xmm6
 pshufd  xmm4, xmm4, 232
 pshufd  xmm5, xmm1, 245
 pmuludq xmm5, xmm6
 pshufd  xmm5, xmm5, 232
 punpckldq xmm4, xmm5
 psrld   xmm4, 16
 psrld   xmm3, 16
 pshuflw xmm3, xmm3, 232
 pshufhw xmm3, xmm3, 232
 pshufd  xmm3, xmm3, 232
 packssdw xmm2, xmm2
 punpcklwd xmm3, xmm2
 pmaddwd xmm3, xmm7
 pshuflw xmm2, xmm4, 232
 pshufhw xmm2, xmm2, 232
 pshufd  xmm2, xmm2, 232
 packssdw xmm1, xmm1
 punpcklwd xmm2, xmm1
 pmaddwd xmm2, xmm7
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11, +, 496], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, +, 432], xmm3
 mov     r11, rdx
 xor     ebx, ebx
.LBB15_8:
 movsx   ebp, word, ptr, [r9, +, 2*rbx, +, 16]
 movdqu  xmm3, xmmword, ptr, [r11, -, 48]
 movdqu  xmm0, xmmword, ptr, [r11, -, 32]
 movdqu  xmm4, xmmword, ptr, [r11, -, 16]
 movdqu  xmm2, xmmword, ptr, [r11]
 movd    xmm1, ebp
 pshufd  xmm1, xmm1, 0
 packssdw xmm1, xmm1
 movdqa  xmm5, xmm1
 pmulhw  xmm5, xmm4
 pmullw  xmm4, xmm1
 movdqa  xmm6, xmm4
 punpcklwd xmm6, xmm5
 punpckhwd xmm4, xmm5
 movdqa  xmm5, xmm4
 pmuludq xmm5, xmm12
 pshufd  xmm5, xmm5, 232
 pshufd  xmm7, xmm4, 245
 pmuludq xmm7, xmm12
 pshufd  xmm7, xmm7, 232
 punpckldq xmm5, xmm7
 movdqa  xmm7, xmm6
 pmuludq xmm7, xmm12
 pshufd  xmm7, xmm7, 232
 pshufd  xmm8, xmm6, 245
 pmuludq xmm8, xmm12
 pshufd  xmm8, xmm8, 232
 punpckldq xmm7, xmm8
 psrld   xmm7, 16
 psrld   xmm5, 16
 pmaddwd xmm5, xmm13
 pmaddwd xmm7, xmm13
 paddd   xmm7, xmm6
 paddd   xmm5, xmm4
 psrad   xmm5, 16
 psrad   xmm7, 16
 packssdw xmm7, xmm5
 movdqa  xmm4, xmm3
 psubw   xmm4, xmm7
 movdqu  xmmword, ptr, [r11, -, 16], xmm4
 paddw   xmm7, xmm3
 movdqu  xmmword, ptr, [r11, -, 48], xmm7
 movdqa  xmm3, xmm1
 pmulhw  xmm3, xmm2
 pmullw  xmm1, xmm2
 movdqa  xmm4, xmm1
 punpcklwd xmm4, xmm3
 punpckhwd xmm1, xmm3
 movdqa  xmm2, xmm1
 pmuludq xmm2, xmm12
 pshufd  xmm2, xmm2, 232
 pshufd  xmm3, xmm1, 245
 pmuludq xmm3, xmm12
 pshufd  xmm3, xmm3, 232
 punpckldq xmm2, xmm3
 movdqa  xmm3, xmm4
 pmuludq xmm3, xmm12
 pshufd  xmm3, xmm3, 232
 pshufd  xmm5, xmm4, 245
 pmuludq xmm5, xmm12
 pshufd  xmm5, xmm5, 232
 punpckldq xmm3, xmm5
 psrld   xmm3, 16
 psrld   xmm2, 16
 pmaddwd xmm2, xmm13
 pmaddwd xmm3, xmm13
 paddd   xmm3, xmm4
 paddd   xmm2, xmm1
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, -, 32], xmm3
 inc     rbx
 add     r11, 64
 cmp     rbx, 8
 jne     .LBB15_8
 mov     r11, rsi
 xor     ebx, ebx
.LBB15_10:
 movdqu  xmm0, xmmword, ptr, [r11, -, 16]
 movdqu  xmm2, xmmword, ptr, [r11]
 movzx   ebp, word, ptr, [r9, +, 2*rbx, +, 32]
 movd    xmm1, ebp
 pshuflw xmm1, xmm1, 0
 pshufd  xmm1, xmm1, 0
 movdqa  xmm3, xmm1
 pmulhw  xmm3, xmm2
 pmullw  xmm1, xmm2
 movdqa  xmm4, xmm1
 punpcklwd xmm4, xmm3
 punpckhwd xmm1, xmm3
 movdqa  xmm2, xmm1
 pmuludq xmm2, xmm12
 pshufd  xmm2, xmm2, 232
 pshufd  xmm3, xmm1, 245
 pmuludq xmm3, xmm12
 pshufd  xmm3, xmm3, 232
 punpckldq xmm2, xmm3
 movdqa  xmm3, xmm4
 pmuludq xmm3, xmm12
 pshufd  xmm3, xmm3, 232
 pshufd  xmm5, xmm4, 245
 pmuludq xmm5, xmm12
 pshufd  xmm5, xmm5, 232
 punpckldq xmm3, xmm5
 psrld   xmm3, 16
 psrld   xmm2, 16
 pmaddwd xmm2, xmm13
 pmaddwd xmm3, xmm13
 paddd   xmm3, xmm4
 paddd   xmm2, xmm1
 psrad   xmm2, 16
 psrad   xmm3, 16
 packssdw xmm3, xmm2
 movdqa  xmm1, xmm0
 psubw   xmm1, xmm3
 movdqu  xmmword, ptr, [r11], xmm1
 paddw   xmm3, xmm0
 movdqu  xmmword, ptr, [r11, -, 16], xmm3
 inc     rbx
 add     r11, 32
 cmp     rbx, 16
 jne     .LBB15_10
 xor     r11d, r11d
.LBB15_12:
 movdqu  xmm12, xmmword, ptr, [r10, +, 8*r11]
 movdqu  xmm4, xmmword, ptr, [r10, +, 8*r11, +, 16]
 movdqa  xmmword, ptr, [rsp, -, 24], xmm4
 movdqu  xmm14, xmmword, ptr, [r10, +, 8*r11, +, 32]
 movdqu  xmm3, xmmword, ptr, [r10, +, 8*r11, +, 48]
 movdqu  xmm8, xmmword, ptr, [r10, +, 8*r11, +, 80]
 movdqu  xmm9, xmmword, ptr, [r10, +, 8*r11, +, 64]
 movdqu  xmm2, xmmword, ptr, [r10, +, 8*r11, +, 112]
 movdqu  xmm15, xmmword, ptr, [r10, +, 8*r11, +, 96]
 movdqa  xmm13, xmm15
 punpcklwd xmm13, xmm2
 pshufd  xmm0, xmm13, 0
 movdqa  xmm5, xmm9
 punpcklwd xmm5, xmm8
 pshufd  xmm1, xmm5, 0
 punpckhdq xmm1, xmm0
 movdqa  xmm10, xmm14
 punpcklwd xmm10, xmm3
 movdqa  xmm7, xmm12
 punpcklwd xmm7, xmm4
 movdqa  xmm4, xmm7
 punpckldq xmm4, xmm10
 shufps  xmm4, xmm1, 228
 movdqa  xmm1, xmm5
 punpckldq xmm1, xmm13
 pshufd  xmm6, xmm10, 85
 pshufd  xmm11, xmm7, 85
 punpckldq xmm11, xmm6
 shufps  xmm11, xmm1, 228
 pshufd  xmm1, xmm13, 170
 pshufd  xmm0, xmm5, 170
 punpckhdq xmm0, xmm1
 pshufd  xmm6, xmm7, 255
 punpckhdq xmm7, xmm10
 shufps  xmm7, xmm0, 228
 movdqu  xmm1, xmmword, ptr, [r11, +, r9, +, 64]
 punpckhdq xmm5, xmm13
 pshufd  xmm0, xmm10, 255
 punpckldq xmm6, xmm0
 shufps  xmm6, xmm5, 228
 punpckhwd xmm15, xmm2
 pshufd  xmm0, xmm15, 0
 punpckhwd xmm9, xmm8
 pshufd  xmm2, xmm9, 0
 punpckhdq xmm2, xmm0
 punpckhwd xmm14, xmm3
 punpckhwd xmm12, xmmword, ptr, [rsp, -, 24]
 movdqa  xmm8, xmm12
 punpckldq xmm8, xmm14
 shufps  xmm8, xmm2, 228
 movdqa  xmm0, xmm9
 punpckldq xmm0, xmm15
 pshufd  xmm3, xmm14, 85
 pshufd  xmm2, xmm12, 85
 punpckldq xmm2, xmm3
 shufps  xmm2, xmm0, 228
 pshufd  xmm0, xmm15, 170
 pshufd  xmm5, xmm9, 170
 punpckhdq xmm5, xmm0
 pshufd  xmm3, xmm12, 255
 punpckhdq xmm12, xmm14
 shufps  xmm12, xmm5, 228
 punpckhdq xmm9, xmm15
 pshufd  xmm0, xmm14, 255
 punpckldq xmm3, xmm0
 shufps  xmm3, xmm9, 228
 movaps  xmm0, xmm8
 pmulhw  xmm0, xmm1
 pmullw  xmm8, xmm1
 movdqa  xmm9, xmm8
 punpcklwd xmm9, xmm0
 punpckhwd xmm8, xmm0
 movdqa  xmm0, xmm8
 movdqa  xmm14, xmmword, ptr, [rip, +, .LCPI15_14]
 pmuludq xmm0, xmm14
 pshufd  xmm5, xmm0, 232
 pshufd  xmm0, xmm8, 245
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 punpckldq xmm5, xmm0
 movdqa  xmm0, xmm9
 pmuludq xmm0, xmm14
 pshufd  xmm13, xmm0, 232
 pshufd  xmm0, xmm9, 245
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 punpckldq xmm13, xmm0
 psrld   xmm13, 16
 psrld   xmm5, 16
 movdqa  xmm15, xmmword, ptr, [rip, +, .LCPI15_15]
 pmaddwd xmm5, xmm15
 pmaddwd xmm13, xmm15
 paddd   xmm13, xmm9
 paddd   xmm5, xmm8
 psrad   xmm5, 16
 psrad   xmm13, 16
 packssdw xmm13, xmm5
 movaps  xmm10, xmm4
 psubw   xmm10, xmm13
 paddw   xmm13, xmm4
 movaps  xmm0, xmm2
 pmulhw  xmm0, xmm1
 pmullw  xmm2, xmm1
 movdqa  xmm8, xmm2
 punpcklwd xmm8, xmm0
 punpckhwd xmm2, xmm0
 movdqa  xmm0, xmm2
 pmuludq xmm0, xmm14
 pshufd  xmm4, xmm0, 232
 pshufd  xmm0, xmm2, 245
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 punpckldq xmm4, xmm0
 movdqa  xmm0, xmm8
 pmuludq xmm0, xmm14
 pshufd  xmm5, xmm0, 232
 pshufd  xmm0, xmm8, 245
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 punpckldq xmm5, xmm0
 psrld   xmm5, 16
 psrld   xmm4, 16
 pmaddwd xmm4, xmm15
 pmaddwd xmm5, xmm15
 paddd   xmm5, xmm8
 paddd   xmm4, xmm2
 psrad   xmm4, 16
 psrad   xmm5, 16
 packssdw xmm5, xmm4
 movaps  xmm4, xmm11
 psubw   xmm4, xmm5
 paddw   xmm5, xmm11
 movaps  xmm0, xmm12
 pmulhw  xmm0, xmm1
 pmullw  xmm12, xmm1
 movdqa  xmm8, xmm12
 punpcklwd xmm8, xmm0
 punpckhwd xmm12, xmm0
 movdqa  xmm0, xmm12
 pmuludq xmm0, xmm14
 pshufd  xmm2, xmm0, 232
 pshufd  xmm0, xmm12, 245
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 punpckldq xmm2, xmm0
 movdqa  xmm0, xmm8
 pmuludq xmm0, xmm14
 pshufd  xmm11, xmm0, 232
 pshufd  xmm0, xmm8, 245
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 punpckldq xmm11, xmm0
 psrld   xmm11, 16
 psrld   xmm2, 16
 pmaddwd xmm2, xmm15
 pmaddwd xmm11, xmm15
 paddd   xmm11, xmm8
 paddd   xmm2, xmm12
 psrad   xmm2, 16
 psrad   xmm11, 16
 packssdw xmm11, xmm2
 movaps  xmm8, xmm7
 psubw   xmm8, xmm11
 paddw   xmm11, xmm7
 movaps  xmm0, xmm3
 pmulhw  xmm0, xmm1
 pmullw  xmm3, xmm1
 movdqa  xmm2, xmm3
 punpcklwd xmm2, xmm0
 punpckhwd xmm3, xmm0
 movdqa  xmm0, xmm3
 pmuludq xmm0, xmm14
 pshufd  xmm0, xmm0, 232
 pshufd  xmm1, xmm3, 245
 pmuludq xmm1, xmm14
 pshufd  xmm1, xmm1, 232
 punpckldq xmm0, xmm1
 movdqa  xmm1, xmm2
 pmuludq xmm1, xmm14
 pshufd  xmm1, xmm1, 232
 pshufd  xmm7, xmm2, 245
 pmuludq xmm7, xmm14
 pshufd  xmm7, xmm7, 232
 punpckldq xmm1, xmm7
 psrld   xmm1, 16
 psrld   xmm0, 16
 pmaddwd xmm0, xmm15
 pmaddwd xmm1, xmm15
 paddd   xmm1, xmm2
 paddd   xmm0, xmm3
 psrad   xmm0, 16
 psrad   xmm1, 16
 packssdw xmm1, xmm0
 movaps  xmm12, xmm6
 psubw   xmm12, xmm1
 paddw   xmm1, xmm6
 movdqa  xmm7, xmm8
 punpcklwd xmm7, xmm12
 movdqa  xmm9, xmm10
 punpcklwd xmm9, xmm4
 movdqa  xmm0, xmm9
 punpckldq xmm0, xmm7
 movdqa  xmm14, xmm11
 punpcklwd xmm14, xmm1
 pshufd  xmm3, xmm14, 85
 movdqa  xmm15, xmm13
 punpcklwd xmm15, xmm5
 pshufd  xmm2, xmm15, 85
 punpckldq xmm2, xmm3
 shufps  xmm2, xmm0, 228
 movdqa  xmm0, xmm15
 punpckldq xmm0, xmm14
 pshufd  xmm6, xmm7, 0
 pshufd  xmm3, xmm9, 0
 punpckhdq xmm3, xmm6
 movsd   xmm3, xmm0
 pshufd  xmm6, xmm14, 255
 pshufd  xmm0, xmm15, 255
 punpckldq xmm0, xmm6
 pshufd  xmm6, xmm9, 170
 punpckhdq xmm9, xmm7
 shufps  xmm0, xmm9, 228
 punpckhdq xmm15, xmm14
 pshufd  xmm7, xmm7, 170
 punpckhdq xmm6, xmm7
 movsd   xmm6, xmm15
 punpckhwd xmm8, xmm12
 punpckhwd xmm10, xmm4
 movdqa  xmm4, xmm10
 punpckldq xmm4, xmm8
 punpckhwd xmm11, xmm1
 pshufd  xmm1, xmm11, 85
 punpckhwd xmm13, xmm5
 pshufd  xmm5, xmm13, 85
 punpckldq xmm5, xmm1
 shufps  xmm5, xmm4, 228
 movdqa  xmm1, xmm13
 punpckldq xmm1, xmm11
 pshufd  xmm4, xmm8, 0
 pshufd  xmm7, xmm10, 0
 punpckhdq xmm7, xmm4
 movsd   xmm7, xmm1
 pshufd  xmm1, xmm11, 255
 pshufd  xmm4, xmm13, 255
 punpckldq xmm4, xmm1
 pshufd  xmm1, xmm10, 170
 punpckhdq xmm10, xmm8
 shufps  xmm4, xmm10, 228
 punpckhdq xmm13, xmm11
 pshufd  xmm8, xmm8, 170
 punpckhdq xmm1, xmm8
 movsd   xmm1, xmm13
 movupd  xmmword, ptr, [r10, +, 8*r11, +, 96], xmm1
 movups  xmmword, ptr, [r10, +, 8*r11, +, 112], xmm4
 movupd  xmmword, ptr, [r10, +, 8*r11, +, 64], xmm7
 movups  xmmword, ptr, [r10, +, 8*r11, +, 80], xmm5
 movupd  xmmword, ptr, [r10, +, 8*r11, +, 32], xmm6
 movups  xmmword, ptr, [r10, +, 8*r11, +, 48], xmm0
 movupd  xmmword, ptr, [r10, +, 8*r11], xmm3
 movups  xmmword, ptr, [r10, +, 8*r11, +, 16], xmm2
 add     r11, 16
 cmp     r11, 64
 jne     .LBB15_12
 xor     r11d, r11d
 movdqa  xmm12, xmmword, ptr, [rip, +, .LCPI15_14]
 movdqa  xmm13, xmmword, ptr, [rip, +, .LCPI15_15]
.LBB15_14:
 movdqu  xmm1, xmmword, ptr, [r9, +, 2*r11, +, 128]
 movdqu  xmm3, xmmword, ptr, [r10, +, 8*r11]
 movdqu  xmm4, xmmword, ptr, [r10, +, 8*r11, +, 16]
 movdqu  xmm2, xmmword, ptr, [r10, +, 8*r11, +, 32]
 movdqu  xmm6, xmmword, ptr, [r10, +, 8*r11, +, 48]
 pshufd  xmm0, xmm4, 232
 pshuflw xmm5, xmm0, 232
 pshufd  xmm7, xmm3, 232
 pshuflw xmm8, xmm7, 232
 punpckldq xmm8, xmm5
 pshufd  xmm9, xmm6, 232
 pshuflw xmm10, xmm9, 132
 pshufd  xmm11, xmm2, 232
 pshuflw xmm5, xmm11, 132
 punpckldq xmm5, xmm10
 movsd   xmm5, xmm8
 pshuflw xmm0, xmm0, 237
 pshuflw xmm7, xmm7, 237
 punpckldq xmm7, xmm0
 pshuflw xmm8, xmm9, 212
 pshuflw xmm0, xmm11, 212
 punpckldq xmm0, xmm8
 movsd   xmm0, xmm7
 pshufd  xmm7, xmm4, 231
 pshuflw xmm4, xmm7, 226
 pshufd  xmm3, xmm3, 231
 pshuflw xmm8, xmm3, 226
 punpckldq xmm8, xmm4
 pshufd  xmm6, xmm6, 231
 pshuflw xmm9, xmm6, 36
 pshufd  xmm2, xmm2, 231
 pshuflw xmm4, xmm2, 36
 punpckldq xmm4, xmm9
 movsd   xmm4, xmm8
 pshuflw xmm7, xmm7, 231
 pshuflw xmm8, xmm3, 231
 punpckldq xmm8, xmm7
 pshuflw xmm6, xmm6, 116
 pshuflw xmm3, xmm2, 116
 punpckldq xmm3, xmm6
 movsd   xmm3, xmm8
 movapd  xmm2, xmm4
 pmulhw  xmm2, xmm1
 pmullw  xmm4, xmm1
 movdqa  xmm7, xmm4
 punpcklwd xmm7, xmm2
 punpckhwd xmm4, xmm2
 movdqa  xmm2, xmm4
 pmuludq xmm2, xmm12
 pshufd  xmm6, xmm2, 232
 pshufd  xmm2, xmm4, 245
 pmuludq xmm2, xmm12
 pshufd  xmm2, xmm2, 232
 punpckldq xmm6, xmm2
 movdqa  xmm2, xmm7
 pmuludq xmm2, xmm12
 pshufd  xmm2, xmm2, 232
 pshufd  xmm8, xmm7, 245
 pmuludq xmm8, xmm12
 pshufd  xmm8, xmm8, 232
 punpckldq xmm2, xmm8
 psrld   xmm2, 16
 psrld   xmm6, 16
 pmaddwd xmm6, xmm13
 pmaddwd xmm2, xmm13
 paddd   xmm2, xmm7
 paddd   xmm6, xmm4
 psrad   xmm6, 16
 psrad   xmm2, 16
 packssdw xmm2, xmm6
 movapd  xmm6, xmm5
 psubw   xmm6, xmm2
 paddw   xmm2, xmm5
 movapd  xmm4, xmm3
 pmulhw  xmm4, xmm1
 pmullw  xmm3, xmm1
 movdqa  xmm5, xmm3
 punpcklwd xmm5, xmm4
 punpckhwd xmm3, xmm4
 movdqa  xmm1, xmm3
 pmuludq xmm1, xmm12
 pshufd  xmm4, xmm1, 232
 pshufd  xmm1, xmm3, 245
 pmuludq xmm1, xmm12
 pshufd  xmm1, xmm1, 232
 punpckldq xmm4, xmm1
 movdqa  xmm1, xmm5
 pmuludq xmm1, xmm12
 pshufd  xmm1, xmm1, 232
 pshufd  xmm7, xmm5, 245
 pmuludq xmm7, xmm12
 pshufd  xmm7, xmm7, 232
 punpckldq xmm1, xmm7
 psrld   xmm1, 16
 psrld   xmm4, 16
 pmaddwd xmm4, xmm13
 pmaddwd xmm1, xmm13
 paddd   xmm1, xmm5
 paddd   xmm4, xmm3
 psrad   xmm4, 16
 psrad   xmm1, 16
 packssdw xmm1, xmm4
 movapd  xmm3, xmm0
 psubw   xmm3, xmm1
 paddw   xmm1, xmm0
 movdqa  xmm0, xmm6
 punpcklwd xmm0, xmm3
 movdqa  xmm4, xmm2
 punpcklwd xmm4, xmm1
 movdqa  xmm5, xmm4
 punpckhdq xmm5, xmm0
 punpckldq xmm4, xmm0
 punpckhwd xmm6, xmm3
 punpckhwd xmm2, xmm1
 movdqa  xmm0, xmm2
 punpckhdq xmm0, xmm6
 punpckldq xmm2, xmm6
 movdqu  xmmword, ptr, [r10, +, 8*r11, +, 32], xmm2
 movdqu  xmmword, ptr, [r10, +, 8*r11, +, 48], xmm0
 movdqu  xmmword, ptr, [r10, +, 8*r11], xmm4
 movdqu  xmmword, ptr, [r10, +, 8*r11, +, 16], xmm5
 add     r11, 8
 cmp     r11, 64
 jne     .LBB15_14
 xor     r11d, r11d
 movdqa  xmm4, xmmword, ptr, [rip, +, .LCPI15_16]
 movdqa  xmm5, xmmword, ptr, [rip, +, .LCPI15_17]
 movdqa  xmm6, xmmword, ptr, [rip, +, .LCPI15_18]
.LBB15_16:
 movdqu  xmm1, xmmword, ptr, [rsi, +, 2*r11, -, 16]
 movdqu  xmm0, xmmword, ptr, [rsi, +, 2*r11]
 movdqa  xmm2, xmm1
 punpcklwd xmm2, xmm2
 movdqa  xmm3, xmm1
 punpckhwd xmm3, xmm3
 pmaddwd xmm3, xmm4
 pmaddwd xmm2, xmm4
 paddd   xmm2, xmm5
 paddd   xmm3, xmm5
 psrad   xmm3, 26
 psrad   xmm2, 26
 packssdw xmm2, xmm3
 pmullw  xmm2, xmm6
 paddw   xmm2, xmm1
 movdqu  xmmword, ptr, [rsi, +, 2*r11, -, 16], xmm2
 movdqa  xmm1, xmm0
 punpcklwd xmm1, xmm1
 movdqa  xmm2, xmm0
 punpckhwd xmm2, xmm2
 pmaddwd xmm2, xmm4
 pmaddwd xmm1, xmm4
 paddd   xmm1, xmm5
 paddd   xmm2, xmm5
 psrad   xmm2, 26
 psrad   xmm1, 26
 packssdw xmm1, xmm2
 pmullw  xmm1, xmm6
 paddw   xmm1, xmm0
 movdqu  xmmword, ptr, [rsi, +, 2*r11], xmm1
 add     r11, 16
 cmp     r11, 256
 jne     .LBB15_16
 inc     r8
 add     rax, 512
 add     rcx, 512
 add     rdx, 512
 add     rsi, 512
 add     r10, 512
 cmp     r8, 3
 movdqa  xmm6, xmmword, ptr, [rip, +, .LCPI15_0]
 movdqa  xmm7, xmmword, ptr, [rip, +, .LCPI15_1]
 movdqa  xmm8, xmmword, ptr, [rip, +, .LCPI15_2]
 movdqa  xmm9, xmmword, ptr, [rip, +, .LCPI15_3]
 movdqa  xmm10, xmmword, ptr, [rip, +, .LCPI15_4]
 movdqa  xmm11, xmmword, ptr, [rip, +, .LCPI15_5]
 jne     .LBB15_1
 pop     rbx
 pop     rbp
 ret
