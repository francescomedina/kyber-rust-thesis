
ntt.o:     file format elf32-littlearm


Disassembly of section .text.pqcrystals_kyber1024_ref_ntt:

00000000 <pqcrystals_kyber1024_ref_ntt>:
   0:	e92d4ff0 	push	{r4, r5, r6, r7, r8, r9, sl, fp, lr}
   4:	e3a09001 	mov	r9, #1
   8:	e3a05007 	mov	r5, #7
   c:	e3a01080 	mov	r1, #128	@ 0x80
  10:	e3a02002 	mov	r2, #2
  14:	e1a0b009 	mov	fp, r9
  18:	e1a0a000 	mov	sl, r0
  1c:	e59f70fc 	ldr	r7, [pc, #252]	@ 120 <pqcrystals_kyber1024_ref_ntt+0x120>
  20:	e24dd01c 	sub	sp, sp, #28
  24:	e3a09000 	mov	r9, #0
  28:	e59f30f4 	ldr	r3, [pc, #244]	@ 124 <pqcrystals_kyber1024_ref_ntt+0x124>
  2c:	e0832002 	add	r2, r3, r2
  30:	e1a08007 	mov	r8, r7
  34:	e1a0e00a 	mov	lr, sl
  38:	e1a0700b 	mov	r7, fp
  3c:	e1a0b009 	mov	fp, r9
  40:	e1a09002 	mov	r9, r2
  44:	e2413001 	sub	r3, r1, #1
  48:	e58d300c 	str	r3, [sp, #12]
  4c:	e2813001 	add	r3, r1, #1
  50:	e58d3010 	str	r3, [sp, #16]
  54:	e58d1008 	str	r1, [sp, #8]
  58:	e58d5014 	str	r5, [sp, #20]
  5c:	e59d3008 	ldr	r3, [sp, #8]
  60:	e083a00b 	add	sl, r3, fp
  64:	e08e508a 	add	r5, lr, sl, lsl #1
  68:	e1a03005 	mov	r3, r5
  6c:	e24b4106 	sub	r4, fp, #-2147483647	@ 0x80000001
  70:	e08e4084 	add	r4, lr, r4, lsl #1
  74:	e1a05004 	mov	r5, r4
  78:	e1a0600b 	mov	r6, fp
  7c:	e1a04003 	mov	r4, r3
  80:	e58de004 	str	lr, [sp, #4]
  84:	e2877001 	add	r7, r7, #1
  88:	e1d400f0 	ldrsh	r0, [r4]
  8c:	e0000098 	mul	r0, r8, r0
  90:	ebfffffe 	bl	0 <pqcrystals_kyber1024_ref_montgomery_reduce>	90: R_ARM_CALL	pqcrystals_kyber1024_ref_montgomery_reduce
  94:	e1f520b2 	ldrh	r2, [r5, #2]!
  98:	e1a00800 	lsl	r0, r0, #16
  9c:	e1a00820 	lsr	r0, r0, #16
  a0:	e0422000 	sub	r2, r2, r0
  a4:	e0c420b2 	strh	r2, [r4], #2
  a8:	e1d520b0 	ldrh	r2, [r5]
  ac:	e2866001 	add	r6, r6, #1
  b0:	e0800002 	add	r0, r0, r2
  b4:	e156000a 	cmp	r6, sl
  b8:	e1c500b0 	strh	r0, [r5]
  bc:	3afffff1 	bcc	88 <pqcrystals_kyber1024_ref_ntt+0x88>
  c0:	e28b1001 	add	r1, fp, #1
  c4:	e15a0001 	cmp	sl, r1
  c8:	e59d300c 	ldr	r3, [sp, #12]
  cc:	33a03000 	movcc	r3, #0
  d0:	e59d2010 	ldr	r2, [sp, #16]
  d4:	e082b00b 	add	fp, r2, fp
  d8:	e083b00b 	add	fp, r3, fp
  dc:	e35b00ff 	cmp	fp, #255	@ 0xff
  e0:	e59de004 	ldr	lr, [sp, #4]
  e4:	91f980f2 	ldrshls	r8, [r9, #2]!
  e8:	9affffdb 	bls	5c <pqcrystals_kyber1024_ref_ntt+0x5c>
  ec:	e59d5014 	ldr	r5, [sp, #20]
  f0:	e59d1008 	ldr	r1, [sp, #8]
  f4:	e2555001 	subs	r5, r5, #1
  f8:	e1a0b007 	mov	fp, r7
  fc:	e1a0a00e 	mov	sl, lr
 100:	e1a010a1 	lsr	r1, r1, #1
 104:	159f3018 	ldrne	r3, [pc, #24]	@ 124 <pqcrystals_kyber1024_ref_ntt+0x124>
 108:	11a02087 	lslne	r2, r7, #1
 10c:	119370f2 	ldrshne	r7, [r3, r2]
 110:	1affffc3 	bne	24 <pqcrystals_kyber1024_ref_ntt+0x24>
 114:	e28dd01c 	add	sp, sp, #28
 118:	e8bd4ff0 	pop	{r4, r5, r6, r7, r8, r9, sl, fp, lr}
 11c:	e12fff1e 	bx	lr	11c: R_ARM_V4BX	*ABS*
 120:	fffffd0a 	.word	0xfffffd0a
 124:	00000000 	.word	0x00000000	124: R_ARM_ABS32	.rodata
