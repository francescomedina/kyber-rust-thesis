.section .text.speed_evaluation::__cortex_m_rt_main,"ax",%progbits
	.p2align	1
	.type	speed_evaluation::__cortex_m_rt_main,%function
	.code	16
	.thumb_func
speed_evaluation::__cortex_m_rt_main:
	.fnstart
	.cfi_startproc
	.save	{r7, lr}
	push {r7, lr}
	.cfi_def_cfa_offset 8
	.cfi_offset lr, -4
	.cfi_offset r7, -8
	.setfp	r7, sp
	mov r7, sp
	.cfi_def_cfa_register r7
	.pad	#528
	sub.w sp, sp, #528
	add.w r9, sp, #16
	mov.w r1, #512
	mov r0, r9
	bl __aeabi_memclr4
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	mov.w r10, #1
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	add.w r1, r0, #10
	movs r5, #0
	mvn r12, #3328
.LBB1_1:
	cmp.w r10, #128
	beq.w .LBB1_46
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	mov.w r11, #256
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	cmp.w r5, #256
	it hi
	movhi r11, r5
	ldrsh.w r8, [r0, r10, lsl #1]
	add.w r6, r5, #128
	mov.w r2, #256
	cmp.w r6, #256
	it hi
	movhi r2, r6
	sub.w r4, r2, #128
	add.w r10, r10, #1
	mov lr, r1
.LBB1_3:
	mov r2, r5
	cmp r4, r5
	beq.w .LBB1_42
	cmp r11, r2
	beq.w .LBB1_41
	add.w r5, r9, r2, lsl #1
	movs r1, #0
	movt r1, #62209
	ldrsh.w r0, [r5, #256]
	mul r0, r0, r8
	muls r1, r0, r1
	smlabt r0, r12, r1, r0
	ldrh.w r1, [r9, r2, lsl #1]
	add.w r3, r1, r0, lsr #16
	sub.w r0, r1, r0, lsr #16
	strh.w r0, [r5, #256]
	adds r5, r2, #1
	cmp r5, r6
	strh.w r3, [r9, r2, lsl #1]
	blo .LBB1_3
	add.w r1, lr, #2
	add.w r5, r2, #129
	cmp r2, #127
	blo .LBB1_1
	cmp.w r10, #128
	mov r4, r10
	mov r1, r10
	mov.w r6, #0
	mvn r10, #3328
	it ls
	movls r4, #128
	add.w r0, r9, #128
	str r0, [sp, #4]
	str r4, [sp, #12]
.LBB1_8:
	cmp r1, r4
	beq.w .LBB1_47
	ldr r0, [sp, #4]
	mov.w r11, #256
	str.w lr, [sp, #8]
	cmp.w r6, #256
	add.w r5, r0, r6, lsl #1
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	it hi
	movhi r11, r6
	ldrsh.w r8, [r0, r1, lsl #1]
	add.w r2, r6, #64
	mov.w r4, #256
	cmp.w r2, #256
	it hi
	movhi r4, r2
	sub.w lr, r4, #64
	adds r1, #1
.LBB1_10:
	mov r4, r6
	cmp lr, r6
	beq.w .LBB1_43
	cmp r11, r4
	beq.w .LBB1_41
	ldrsh.w r6, [r5]
	movs r0, #0
	movt r0, #62209
	mul r6, r6, r8
	mul r12, r6, r0
	ldrh r0, [r5, #-128]
	smlabt r6, r10, r12, r6
	sub.w r3, r0, r6, lsr #16
	add.w r0, r0, r6, lsr #16
	strh r3, [r5], #2
	adds r6, r4, #1
	cmp r6, r2
	strh r0, [r5, #-130]
	blo .LBB1_10
	ldr r2, [sp, #8]
	add.w r6, r4, #65
	cmp r4, #191
	ldr r4, [sp, #12]
	add.w lr, r2, #2
	blo .LBB1_8
	mov r4, r1
	movs r3, #0
	mvn r12, #3328
	cmp r1, #128
	it ls
	movls r4, #128
	add.w r0, r9, #64
	str r0, [sp, #4]
	str r4, [sp, #12]
.LBB1_15:
	cmp r1, r4
	beq.w .LBB1_47
	ldr r0, [sp, #4]
	mov.w r11, #256
	cmp.w r3, #256
	it hi
	movhi r11, r3
	add.w r5, r0, r3, lsl #1
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	add.w r8, r3, #32
	ldrsh.w lr, [r0, r1, lsl #1]
	mov.w r0, #256
	cmp.w r8, #256
	it hi
	movhi r0, r8
	sub.w r4, r0, #32
	adds r1, #1
	mov r10, r2
.LBB1_17:
	mov r6, r3
	cmp r4, r3
	beq.w .LBB1_44
	cmp r11, r6
	beq.w .LBB1_41
	ldrsh.w r0, [r5]
	movs r2, #0
	movt r2, #62209
	mul r0, r0, lr
	mul r3, r0, r2
	smlabt r0, r12, r3, r0
	ldrh r3, [r5, #-64]
	sub.w r2, r3, r0, lsr #16
	add.w r0, r3, r0, lsr #16
	strh r2, [r5], #2
	adds r3, r6, #1
	cmp r3, r8
	strh r0, [r5, #-66]
	blo .LBB1_17
	ldr r4, [sp, #12]
	add.w r2, r10, #2
	add.w r3, r6, #33
	cmp r6, #223
	blo .LBB1_15
	mov r4, r1
	movs r3, #0
	mvn r8, #3328
	cmp r1, #128
	it ls
	movls r4, #128
	add.w r0, r9, #32
	str r0, [sp, #8]
	str r4, [sp, #12]
.LBB1_22:
	cmp r1, r4
	beq.w .LBB1_47
	ldr r0, [sp, #8]
	mov.w r11, #256
	cmp.w r3, #256
	it hi
	movhi r11, r3
	add.w r5, r0, r3, lsl #1
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	mov r12, r10
	ldrsh.w lr, [r0, r1, lsl #1]
	add.w r10, r3, #16
	mov.w r0, #256
	cmp.w r10, #256
	it hi
	movhi r0, r10
	sub.w r4, r0, #16
	adds r1, #1
.LBB1_24:
	mov r6, r3
	cmp r4, r3
	beq.w .LBB1_45
	cmp r11, r6
	beq.w .LBB1_41
	ldrsh.w r0, [r5]
	movs r2, #0
	movt r2, #62209
	mul r0, r0, lr
	mul r3, r0, r2
	smlabt r0, r8, r3, r0
	ldrh r3, [r5, #-32]
	sub.w r2, r3, r0, lsr #16
	add.w r0, r3, r0, lsr #16
	strh r2, [r5], #2
	adds r3, r6, #1
	cmp r3, r10
	strh r0, [r5, #-34]
	blo .LBB1_24
	ldr r4, [sp, #12]
	add.w r10, r12, #2
	add.w r3, r6, #17
	cmp r6, #239
	blo .LBB1_22
	movw r11, #0
	mov r10, r1
	cmp r1, #128
	mov r4, r1
	add.w r1, r9, #30
	mov.w r8, #0
	movt r11, #62209
	it ls
	movls r4, #128
.LBB1_29:
	cmp r4, r10
	beq.w .LBB1_47
	cmp.w r8, #247
	bhi.w .LBB1_48
	add.w r6, r9, r8, lsl #1
	ldrsh r5, [r12], #2
	mvn lr, #3328
	ldrsh.w r0, [r6, #16]
	muls r0, r5, r0
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r1, #-30]
	sub.w r3, r2, r0, lsr #16
	strh r3, [r6, #16]
	add.w r0, r2, r0, lsr #16
	strh r0, [r1, #-30]
	ldrsh.w r0, [r6, #18]
	ldrsh.w r3, [r6, #22]
	strd r3, r10, [sp, #8]
	mov r3, r4
	muls r0, r5, r0
	ldrsh.w r2, [r6, #20]
	ldrsh.w r10, [r6, #24]
	mul r4, r0, r11
	smlabt r0, lr, r4, r0
	ldrh r4, [r6, #2]
	sub.w lr, r4, r0, lsr #16
	add.w r0, r4, r0, lsr #16
	strh r0, [r6, #2]
	mul r0, r2, r5
	strh.w lr, [r6, #18]
	mvn lr, #3328
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r6, #4]
	sub.w r4, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #4]
	ldr r0, [sp, #8]
	strh r4, [r6, #20]
	mov r4, r3
	muls r0, r5, r0
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r6, #6]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #6]
	mul r0, r10, r5
	strh r3, [r6, #22]
	ldr.w r10, [sp, #12]
	add.w r10, r10, #1
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r6, #8]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #8]
	ldrsh.w r0, [r6, #26]
	strh r3, [r6, #24]
	muls r0, r5, r0
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r6, #10]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #10]
	ldrsh.w r0, [r6, #28]
	strh r3, [r6, #26]
	muls r0, r5, r0
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r6, #12]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r3, [r6, #28]
	strh r0, [r6, #12]
	ldrsh.w r0, [r1]
	muls r0, r5, r0
	mul r2, r0, r11
	smlabt r0, lr, r2, r0
	ldrh r2, [r6, #14]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r3, [r1], #32
	add.w r2, r8, #7
	strh r0, [r6, #14]
	add.w r0, r8, #16
	cmp r2, #247
	mov r8, r0
	blo.w .LBB1_29
	movw lr, #0
	rsb.w r8, r10, #0
	add.w r6, r9, #14
	mov r4, r10
	movs r5, #0
	movt lr, #62209
	cmp.w r10, #128
	it ls
	movls r4, #128
	str r4, [sp, #12]
.LBB1_33:
	cmp r4, r10
	beq.w .LBB1_47
	cmp r5, #251
	bhi.w .LBB1_49
	add.w r2, r9, r5, lsl #1
	ldrsh r3, [r12], #2
	mvn r1, #3328
	mov r11, r10
	ldrsh.w r0, [r2, #8]
	sub.w r8, r8, #1
	muls r0, r3, r0
	mul r4, r0, lr
	smlabt r0, r1, r4, r0
	ldrh r4, [r6, #-14]
	sub.w r1, r4, r0, lsr #16
	strh r1, [r2, #8]
	add.w r0, r4, r0, lsr #16
	strh r0, [r6, #-14]
	ldrsh.w r0, [r2, #10]
	mvn r1, #3328
	ldrsh.w r10, [r2, #12]
	muls r0, r3, r0
	mul r4, r0, lr
	smlabt r0, r1, r4, r0
	ldrh r4, [r2, #2]
	sub.w r1, r4, r0, lsr #16
	add.w r0, r4, r0, lsr #16
	strh r0, [r2, #2]
	mul r0, r10, r3
	strh r1, [r2, #10]
	mvn r4, #3328
	mov r10, r11
	mvn r11, #3328
	add.w r10, r10, #1
	mul r1, r0, lr
	smlabt r0, r4, r1, r0
	ldrh r1, [r2, #4]
	sub.w r4, r1, r0, lsr #16
	strh r4, [r2, #12]
	add.w r0, r1, r0, lsr #16
	strh r0, [r2, #4]
	ldrsh.w r0, [r6]
	ldr r4, [sp, #12]
	muls r0, r3, r0
	mul r1, r0, lr
	smlabt r0, r11, r1, r0
	ldrh r1, [r2, #6]
	sub.w r3, r1, r0, lsr #16
	add.w r0, r1, r0, lsr #16
	strh r3, [r6], #16
	adds r1, r5, #3
	strh r0, [r2, #6]
	add.w r0, r5, #8
	cmp r1, #251
	mov r5, r0
	blo .LBB1_33
	cmp.w r10, #128
	it ls
	movls.w r10, #128
	add.w r1, r9, #4
	movw r9, #0
	add.w r0, r10, r8
	movs r6, #0
	mvn lr, #3328
	movt r9, #62209
.LBB1_37:
	cmp r0, #0
	beq .LBB1_50
	cmp r6, #253
	bhi .LBB1_51
	ldrsh r3, [r12], #2
	subs r0, #1
	ldrsh.w r5, [r1]
	ldrsh.w r8, [r1, #2]
	muls r5, r3, r5
	mul r2, r5, r9
	smlabt r2, lr, r2, r5
	ldrh r5, [r1, #-4]
	sub.w r4, r5, r2, lsr #16
	strh r4, [r1], #8
	add.w r2, r5, r2, lsr #16
	strh r2, [r1, #-12]
	mul r2, r8, r3
	mul r3, r2, r9
	smlabt r2, lr, r3, r2
	ldrh r3, [r1, #-10]
	sub.w r4, r3, r2, lsr #16
	add.w r2, r3, r2, lsr #16
	strh r2, [r1, #-10]
	adds r2, r6, #4
	adds r3, r6, #1
	strh r4, [r1, #-6]
	cmp r3, #253
	mov r6, r2
	blo .LBB1_37
.LBB1_40:
	b .LBB1_40
.LBB1_41:
	mov r0, r11
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_42:
	add.w r0, r2, #128
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_43:
	add.w r0, r4, #64
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_44:
	add.w r0, r6, #32
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_45:
	add.w r0, r6, #16
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_46:
	movs r4, #128
.LBB1_47:
	mov r0, r4
	movs r1, #128
	bl core::panicking::panic_bounds_check
.LBB1_48:
	add.w r0, r8, #8
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_49:
	adds r0, r5, #4
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_50:
	mov r4, r10
	mov r0, r4
	movs r1, #128
	bl core::panicking::panic_bounds_check
.LBB1_51:
	adds r0, r6, #2
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
