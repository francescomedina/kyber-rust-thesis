.section .text.compilation_analysis_ntt::__cortex_m_rt_main,"ax",%progbits
	.p2align	1
	.type	compilation_analysis_ntt::__cortex_m_rt_main,%function
	.code	16
	.thumb_func
compilation_analysis_ntt::__cortex_m_rt_main:
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
	movs r1, #2
	movw r0, #65535
	strh.w r1, [sp, #490]
	mov.w r3, #65536
	str r1, [sp, #460]
	mov.w r12, #-1
	strh.w r1, [sp, #404]
	mov.w r8, #65537
	strh.w r1, [sp, #358]
	movw r9, #1
	str.w r1, [sp, #354]
	movw r5, #65535
	strh.w r1, [sp, #264]
	movs r6, #2
	strh.w r1, [sp, #260]
	movw r11, #0
	str r1, [sp, #224]
	movs r4, #1
	str r1, [sp, #220]
	movw lr, #65535
	str.w r1, [sp, #170]
	mov.w r10, #0
	str r1, [sp, #148]
	movt r4, #2
	strh.w r1, [sp, #144]
	movs r2, #1
	strh.w r1, [sp, #70]
	movt r6, #65535
	strh.w r1, [sp, #66]
	movs r1, #2
	movt r1, #1
	str r3, [sp, #464]
	mov.w r3, #131072
	str r1, [sp, #20]
	sub.w r1, r12, #1
	str r0, [sp, #524]
	str r0, [sp, #508]
	movs r0, #2
	movt r0, #1
	str r3, [sp, #452]
	movw r3, #65535
	strh.w r1, [sp, #514]
	strh.w r1, [sp, #506]
	movt r5, #2
	strh.w r1, [sp, #492]
	movt r9, #65535
	strh.w r1, [sp, #484]
	movt lr, #1
	strh.w r1, [sp, #472]
	movt r11, #65535
	strh.w r1, [sp, #438]
	strh.w r1, [sp, #434]
	strh.w r1, [sp, #418]
	strh.w r1, [sp, #406]
	strh.w r1, [sp, #398]
	strh.w r1, [sp, #390]
	strh.w r1, [sp, #376]
	strh.w r1, [sp, #366]
	strh.w r1, [sp, #360]
	strh.w r1, [sp, #332]
	strh.w r1, [sp, #326]
	strh.w r1, [sp, #316]
	strh.w r1, [sp, #298]
	strh.w r1, [sp, #276]
	strh.w r1, [sp, #272]
	strh.w r1, [sp, #266]
	strh.w r1, [sp, #262]
	strh.w r1, [sp, #258]
	strh.w r1, [sp, #244]
	strh.w r1, [sp, #232]
	strh.w r1, [sp, #194]
	strh.w r1, [sp, #192]
	strh.w r1, [sp, #186]
	strh.w r1, [sp, #180]
	strh.w r1, [sp, #168]
	strh.w r1, [sp, #166]
	strh.w r1, [sp, #158]
	strh.w r1, [sp, #146]
	strh.w r1, [sp, #142]
	strh.w r1, [sp, #128]
	strh.w r1, [sp, #114]
	strh.w r1, [sp, #112]
	strh.w r1, [sp, #104]
	strh.w r1, [sp, #102]
	strh.w r1, [sp, #86]
	strh.w r1, [sp, #84]
	strh.w r1, [sp, #72]
	strh.w r1, [sp, #68]
	strh.w r1, [sp, #56]
	strh.w r1, [sp, #52]
	strh.w r1, [sp, #50]
	strh.w r1, [sp, #32]
	strh.w r1, [sp, #24]
	strh.w r1, [sp, #18]
	movw r1, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	movt r1, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	str.w r0, [sp, #498]
	str r0, [sp, #420]
	adds r1, #10
	str.w r0, [sp, #382]
	mov.w r0, #131074
	str.w r3, [sp, #338]
	str r3, [sp, #268]
	movs r3, #2
	movt r3, #1
	str.w r12, [sp, #502]
	str.w r12, [sp, #448]
	strh.w r12, [sp, #396]
	str.w r12, [sp, #392]
	str.w r12, [sp, #346]
	str r0, [sp, #308]
	str.w r0, [sp, #278]
	str.w r0, [sp, #238]
	mov.w r0, #131072
	strh.w r12, [sp, #274]
	str.w r12, [sp, #216]
	str.w r12, [sp, #46]
	strh.w r12, [sp, #30]
	add.w r12, sp, #16
	str.w r8, [sp, #516]
	str.w r8, [sp, #478]
	str.w r8, [sp, #456]
	str.w r8, [sp, #378]
	str.w r8, [sp, #250]
	str.w r8, [sp, #212]
	str.w r8, [sp, #38]
	mvn r8, #3328
	str r0, [sp, #188]
	str.w r3, [sp, #182]
	str r3, [sp, #124]
	movw r3, #65535
	str.w r0, [sp, #134]
	str.w r0, [sp, #78]
	movs r0, #1
	str.w r9, [sp, #520]
	strh.w r2, [sp, #512]
	str.w r5, [sp, #494]
	str.w r10, [sp, #486]
	strh.w r10, [sp, #482]
	str.w r6, [sp, #474]
	str.w r11, [sp, #468]
	str r6, [sp, #444]
	str r4, [sp, #440]
	strh.w r10, [sp, #436]
	strh.w r10, [sp, #432]
	str r5, [sp, #428]
	str.w r11, [sp, #424]
	strh.w r10, [sp, #416]
	str.w r9, [sp, #412]
	str r2, [sp, #408]
	str.w r9, [sp, #400]
	str.w r4, [sp, #386]
	str r5, [sp, #372]
	str.w lr, [sp, #368]
	str.w r2, [sp, #362]
	str.w r5, [sp, #350]
	str.w r10, [sp, #342]
	str.w r5, [sp, #334]
	str r2, [sp, #328]
	str.w r2, [sp, #322]
	str.w r4, [sp, #318]
	str r6, [sp, #312]
	str.w r9, [sp, #304]
	str.w lr, [sp, #300]
	str.w lr, [sp, #294]
	str.w r4, [sp, #290]
	str.w r6, [sp, #286]
	str.w r5, [sp, #282]
	str.w r10, [sp, #254]
	str.w r6, [sp, #246]
	strh.w r10, [sp, #242]
	str.w lr, [sp, #234]
	str r4, [sp, #228]
	str.w r11, [sp, #208]
	str.w r11, [sp, #204]
	str r2, [sp, #200]
	str r6, [sp, #196]
	strh.w r10, [sp, #178]
	str.w r4, [sp, #174]
	strh.w r2, [sp, #164]
	str r6, [sp, #160]
	strh.w r2, [sp, #156]
	str r4, [sp, #152]
	str.w r4, [sp, #138]
	str.w lr, [sp, #130]
	str r3, [sp, #120]
	str r4, [sp, #116]
	strh.w r10, [sp, #110]
	str.w r11, [sp, #106]
	strh.w r10, [sp, #100]
	str.w r9, [sp, #96]
	str r2, [sp, #92]
	str r4, [sp, #88]
	strh.w r10, [sp, #82]
	str.w r6, [sp, #74]
	str.w r9, [sp, #62]
	str.w r5, [sp, #58]
	strh.w r10, [sp, #54]
	str.w r3, [sp, #42]
	str.w lr, [sp, #34]
	str.w r6, [sp, #26]
	strh.w r0, [sp, #16]
.LBB1_1:
	cmp r0, #128
	beq.w .LBB1_46
	mov lr, r1
	mov r1, r0
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	mov.w r11, #256
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	cmp.w r10, #256
	it hi
	movhi r11, r10
	ldrsh.w r9, [r0, r1, lsl #1]
	add.w r4, r10, #128
	mov.w r0, #256
	cmp.w r4, #256
	it hi
	movhi r0, r4
	sub.w r2, r0, #128
	adds r1, #1
	str r1, [sp, #12]
.LBB1_3:
	mov r0, r10
	cmp r2, r10
	beq.w .LBB1_42
	cmp r11, r0
	beq.w .LBB1_41
	add.w r6, r12, r0, lsl #1
	movs r1, #0
	movt r1, #62209
	add.w r10, r0, #1
	ldrsh.w r5, [r6, #256]
	cmp r10, r4
	mul r5, r5, r9
	mul r1, r5, r1
	smlabt r1, r8, r1, r5
	ldrh.w r5, [r12, r0, lsl #1]
	add.w r3, r5, r1, lsr #16
	sub.w r1, r5, r1, lsr #16
	strh.w r3, [r12, r0, lsl #1]
	strh.w r1, [r6, #256]
	blo .LBB1_3
	add.w r10, r0, #129
	cmp r0, #127
	ldr r0, [sp, #12]
	add.w r1, lr, #2
	blo .LBB1_1
	mov r4, r0
	movs r5, #0
	mvn r10, #3328
	cmp r0, #128
	it ls
	movls r4, #128
	add.w r1, r12, #128
	str r1, [sp, #4]
	str r4, [sp, #8]
.LBB1_8:
	cmp r0, r4
	beq.w .LBB1_47
	ldr r1, [sp, #4]
	mov.w r11, #256
	str.w lr, [sp, #12]
	cmp.w r5, #256
	add.w r6, r1, r5, lsl #1
	mov r1, r0
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	it hi
	movhi r11, r5
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	add.w r4, r5, #64
	ldrsh.w r9, [r0, r1, lsl #1]
	mov.w r2, #256
	cmp.w r4, #256
	it hi
	movhi r2, r4
	sub.w r8, r2, #64
	adds r1, #1
.LBB1_10:
	mov r2, r5
	cmp r8, r5
	beq.w .LBB1_43
	cmp r11, r2
	beq.w .LBB1_41
	ldrsh.w r5, [r6]
	movs r0, #0
	movt r0, #62209
	mul r5, r5, r9
	mul lr, r5, r0
	ldrh r0, [r6, #-128]
	smlabt r5, r10, lr, r5
	sub.w r3, r0, r5, lsr #16
	add.w r0, r0, r5, lsr #16
	strh r3, [r6], #2
	adds r5, r2, #1
	cmp r5, r4
	strh r0, [r6, #-130]
	blo .LBB1_10
	ldr r6, [sp, #12]
	add.w r5, r2, #65
	ldr r4, [sp, #8]
	cmp r2, #191
	add.w lr, r6, #2
	mov r0, r1
	blo .LBB1_8
	mov r4, r0
	movs r3, #0
	mvn lr, #3328
	cmp r0, #128
	it ls
	movls r4, #128
	add.w r1, r12, #64
	str r1, [sp, #4]
	str r4, [sp, #8]
.LBB1_15:
	cmp r0, r4
	beq.w .LBB1_47
	ldr r1, [sp, #4]
	mov r10, r6
	mov.w r11, #256
	cmp.w r3, #256
	add.w r6, r1, r3, lsl #1
	movw r1, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	movt r1, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	it hi
	movhi r11, r3
	ldrsh.w r8, [r1, r0, lsl #1]
	mov r1, r0
	add.w r9, r3, #32
	mov.w r0, #256
	cmp.w r9, #256
	it hi
	movhi r0, r9
	sub.w r2, r0, #32
	adds r1, #1
.LBB1_17:
	mov r5, r3
	cmp r2, r3
	beq.w .LBB1_44
	cmp r11, r5
	beq.w .LBB1_41
	ldrsh.w r0, [r6]
	movs r3, #0
	movt r3, #62209
	mul r0, r0, r8
	muls r3, r0, r3
	smlabt r0, lr, r3, r0
	ldrh r3, [r6, #-64]
	sub.w r4, r3, r0, lsr #16
	add.w r0, r3, r0, lsr #16
	strh r4, [r6], #2
	adds r3, r5, #1
	cmp r3, r9
	strh r0, [r6, #-66]
	blo .LBB1_17
	ldr r4, [sp, #8]
	add.w r6, r10, #2
	add.w r3, r5, #33
	cmp r5, #223
	mov r0, r1
	blo .LBB1_15
	mov r4, r0
	movs r3, #0
	mvn r9, #3328
	cmp r0, #128
	it ls
	movls r4, #128
	add.w r0, r12, #32
	str r0, [sp, #12]
	str r4, [sp, #8]
.LBB1_22:
	cmp r1, r4
	beq.w .LBB1_47
	ldr r0, [sp, #12]
	mov.w r11, #256
	cmp.w r3, #256
	it hi
	movhi r11, r3
	add.w r6, r0, r3, lsl #1
	movw r0, :lower16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	movt r0, :upper16:.Lanon.e014a0186272dfb360e2d177b7b7066f.61
	mov lr, r10
	ldrsh.w r8, [r0, r1, lsl #1]
	add.w r10, r3, #16
	mov.w r0, #256
	cmp.w r10, #256
	it hi
	movhi r0, r10
	sub.w r2, r0, #16
	adds r1, #1
.LBB1_24:
	mov r4, r3
	cmp r2, r3
	beq.w .LBB1_45
	cmp r11, r4
	beq.w .LBB1_41
	ldrsh.w r0, [r6]
	movs r3, #0
	movt r3, #62209
	mul r0, r0, r8
	muls r3, r0, r3
	smlabt r0, r9, r3, r0
	ldrh r3, [r6, #-32]
	sub.w r5, r3, r0, lsr #16
	add.w r0, r3, r0, lsr #16
	strh r5, [r6], #2
	adds r3, r4, #1
	cmp r3, r10
	strh r0, [r6, #-34]
	blo .LBB1_24
	add.w r3, r4, #17
	cmp r4, #239
	ldr r4, [sp, #8]
	add.w r10, lr, #2
	blo .LBB1_22
	movw r9, #0
	add.w r5, r12, #30
	mov r4, r1
	mov.w r10, #0
	movt r9, #62209
	cmp r1, #128
	it ls
	movls r4, #128
.LBB1_29:
	cmp r4, r1
	beq.w .LBB1_47
	cmp.w r10, #247
	bhi.w .LBB1_48
	add.w r6, r12, r10, lsl #1
	ldrsh r11, [lr], #2
	mvn r8, #3328
	ldrsh.w r0, [r6, #16]
	mul r0, r0, r11
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r5, #-30]
	sub.w r3, r2, r0, lsr #16
	strh r3, [r6, #16]
	add.w r0, r2, r0, lsr #16
	strh r0, [r5, #-30]
	ldrsh.w r0, [r6, #18]
	ldrsh.w r3, [r6, #22]
	strd r3, r1, [sp, #8]
	mul r0, r0, r11
	ldrsh.w r2, [r6, #20]
	ldrsh.w r1, [r6, #24]
	mul r3, r0, r9
	smlabt r0, r8, r3, r0
	ldrh r3, [r6, #2]
	sub.w r8, r3, r0, lsr #16
	add.w r0, r3, r0, lsr #16
	strh r0, [r6, #2]
	mul r0, r2, r11
	strh.w r8, [r6, #18]
	mvn r8, #3328
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r6, #4]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #4]
	ldr r0, [sp, #8]
	strh r3, [r6, #20]
	mul r0, r0, r11
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r6, #6]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #6]
	mul r0, r1, r11
	strh r3, [r6, #22]
	ldr r1, [sp, #12]
	adds r1, #1
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r6, #8]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #8]
	ldrsh.w r0, [r6, #26]
	strh r3, [r6, #24]
	mul r0, r0, r11
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r6, #10]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r0, [r6, #10]
	ldrsh.w r0, [r6, #28]
	strh r3, [r6, #26]
	mul r0, r0, r11
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r6, #12]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r3, [r6, #28]
	strh r0, [r6, #12]
	ldrsh.w r0, [r5]
	mul r0, r0, r11
	mul r2, r0, r9
	smlabt r0, r8, r2, r0
	ldrh r2, [r6, #14]
	sub.w r3, r2, r0, lsr #16
	add.w r0, r2, r0, lsr #16
	strh r3, [r5], #32
	add.w r2, r10, #7
	strh r0, [r6, #14]
	add.w r0, r10, #16
	cmp r2, #247
	mov r10, r0
	blo.w .LBB1_29
	rsb.w r9, r1, #0
	add.w r11, r12, #14
	mov r2, r1
	mov r4, r1
	movs r6, #0
	cmp r1, #128
	it ls
	movls r4, #128
	str r4, [sp, #8]
.LBB1_33:
	cmp r4, r2
	str r2, [sp, #12]
	beq.w .LBB1_47
	cmp r6, #251
	bhi.w .LBB1_49
	add.w r3, r12, r6, lsl #1
	ldrsh r2, [lr], #2
	movs r4, #0
	sub.w r9, r9, #1
	ldrsh.w r0, [r3, #8]
	movt r4, #62209
	muls r0, r2, r0
	mul r5, r0, r4
	smlabt r0, r8, r5, r0
	ldrh r5, [r11, #-14]
	sub.w r1, r5, r0, lsr #16
	strh r1, [r3, #8]
	add.w r0, r5, r0, lsr #16
	strh r0, [r11, #-14]
	ldrsh.w r0, [r3, #10]
	ldrsh.w r10, [r3, #12]
	muls r0, r2, r0
	mul r5, r0, r4
	smlabt r0, r8, r5, r0
	ldrh r5, [r3, #2]
	sub.w r1, r5, r0, lsr #16
	add.w r0, r5, r0, lsr #16
	strh r0, [r3, #2]
	mul r0, r10, r2
	strh r1, [r3, #10]
	mul r1, r0, r4
	smlabt r0, r8, r1, r0
	ldrh r1, [r3, #4]
	sub.w r5, r1, r0, lsr #16
	add.w r0, r1, r0, lsr #16
	strh r5, [r3, #12]
	strh r0, [r3, #4]
	ldrsh.w r0, [r11]
	muls r0, r2, r0
	mul r1, r0, r4
	ldr r4, [sp, #8]
	smlabt r0, r8, r1, r0
	ldrh r1, [r3, #6]
	sub.w r2, r1, r0, lsr #16
	strh r2, [r11], #16
	add.w r0, r1, r0, lsr #16
	ldr r2, [sp, #12]
	strh r0, [r3, #6]
	add.w r0, r6, #8
	adds r1, r6, #3
	adds r2, #1
	cmp r1, #251
	mov r6, r0
	blo .LBB1_33
	cmp r2, #128
	movw r10, #0
	it ls
	movls r2, #128
	add.w r0, r2, r9
	add.w r1, r12, #4
	movs r4, #0
	mvn r12, #3328
	mov r9, r2
	movt r10, #62209
.LBB1_37:
	cmp r0, #0
	beq .LBB1_50
	cmp r4, #253
	bhi .LBB1_51
	ldrsh r3, [lr], #2
	subs r0, #1
	ldrsh.w r6, [r1]
	ldrsh.w r8, [r1, #2]
	muls r6, r3, r6
	mul r2, r6, r10
	smlabt r2, r12, r2, r6
	ldrh r6, [r1, #-4]
	sub.w r5, r6, r2, lsr #16
	strh r5, [r1], #8
	add.w r2, r6, r2, lsr #16
	strh r2, [r1, #-12]
	mul r2, r8, r3
	mul r3, r2, r10
	smlabt r2, r12, r3, r2
	ldrh r3, [r1, #-10]
	sub.w r5, r3, r2, lsr #16
	add.w r2, r3, r2, lsr #16
	strh r2, [r1, #-10]
	adds r2, r4, #4
	adds r3, r4, #1
	strh r5, [r1, #-6]
	cmp r3, #253
	mov r4, r2
	blo .LBB1_37
.LBB1_40:
	b .LBB1_40
.LBB1_41:
	mov r0, r11
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_42:
	adds r0, #128
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_43:
	add.w r0, r2, #64
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_44:
	add.w r0, r5, #32
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_45:
	add.w r0, r4, #16
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_46:
	movs r4, #128
.LBB1_47:
	mov r0, r4
	movs r1, #128
	bl core::panicking::panic_bounds_check
.LBB1_48:
	add.w r0, r10, #8
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_49:
	adds r0, r6, #4
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
.LBB1_50:
	mov r4, r9
	mov r0, r4
	movs r1, #128
	bl core::panicking::panic_bounds_check
.LBB1_51:
	adds r0, r4, #2
	mov.w r1, #256
	bl core::panicking::panic_bounds_check
