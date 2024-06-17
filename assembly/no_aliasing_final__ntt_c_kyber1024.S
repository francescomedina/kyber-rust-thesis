
final.o:     file format elf64-x86-64


Disassembly of section .text.pqcrystals_kyber1024_ref_ntt:

0000000000000000              <pqcrystals_kyber1024_ref_ntt>:
   0:	f3 0f 1e fa          	endbr64 
   4:	41 57                	push   r15
   6:	48 8d 47 02          	lea    rax,[rdi+0x2]
   a:	41 56                	push   r14
   c:	41 55                	push   r13
   e:	41 bd 01 00 00 00    	mov    r13d,0x1
  14:	41 54                	push   r12
  16:	55                   	push   rbp
  17:	53                   	push   rbx
  18:	bb 0a fd ff ff       	mov    ebx,0xfffffd0a
  1d:	48 83 ec 28          	sub    rsp,0x28
  21:	48 89 7c 24 08       	mov    QWORD PTR [rsp+0x8],rdi
  26:	c7 44 24 1c 07 00 00 00 	mov    DWORD PTR [rsp+0x1c],0x7
  2e:	c7 44 24 04 80 00 00 00 	mov    DWORD PTR [rsp+0x4],0x80
  36:	48 89 44 24 10       	mov    QWORD PTR [rsp+0x10],rax
  3b:	0f 1f 44 00 00       	nop    DWORD PTR [rax+rax*1+0x0]
  40:	8b 54 24 04          	mov    edx,DWORD PTR [rsp+0x4]
  44:	31 c0                	xor    eax,eax
  46:	83 ea 01             	sub    edx,0x1
  49:	89 54 24 18          	mov    DWORD PTR [rsp+0x18],edx
  4d:	0f 1f 00             	nop    DWORD PTR [rax]
  50:	8b 54 24 04          	mov    edx,DWORD PTR [rsp+0x4]
  54:	41 83 c5 01          	add    r13d,0x1
  58:	44 8d 24 02          	lea    r12d,[rdx+rax*1]
  5c:	41 39 c4             	cmp    r12d,eax
  5f:	0f 86 9b 00 00 00    	jbe    100 <pqcrystals_kyber1024_ref_ntt+0x100>
  65:	48 8b 54 24 08       	mov    rdx,QWORD PTR [rsp+0x8]
  6a:	8b 4c 24 18          	mov    ecx,DWORD PTR [rsp+0x18]
  6e:	44 89 e6             	mov    esi,r12d
  71:	48 29 c6             	sub    rsi,rax
  74:	4c 8d 34 42          	lea    r14,[rdx+rax*2]
  78:	48 8b 54 24 10       	mov    rdx,QWORD PTR [rsp+0x10]
  7d:	48 01 c1             	add    rcx,rax
  80:	49 89 f7             	mov    r15,rsi
  83:	48 8d 2c 4a          	lea    rbp,[rdx+rcx*2]
  87:	66 0f 1f 84 00 00 00 00 00 	nop    WORD PTR [rax+rax*1+0x0]
  90:	43 0f bf 3c 7e       	movsx  edi,WORD PTR [r14+r15*2]
  95:	0f af fb             	imul   edi,ebx
  98:	e8 00 00 00 00       	call   9d <pqcrystals_kyber1024_ref_ntt+0x9d>	99: R_X86_64_PLT32	pqcrystals_kyber1024_ref_montgomery_reduce-0x4
  9d:	41 0f b7 0e          	movzx  ecx,WORD PTR [r14]
  a1:	29 c1                	sub    ecx,eax
  a3:	66 43 89 0c 7e       	mov    WORD PTR [r14+r15*2],cx
  a8:	66 41 01 06          	add    WORD PTR [r14],ax
  ac:	49 83 c6 02          	add    r14,0x2
  b0:	4c 39 f5             	cmp    rbp,r14
  b3:	75 db                	jne    90 <pqcrystals_kyber1024_ref_ntt+0x90>
  b5:	8b 44 24 04          	mov    eax,DWORD PTR [rsp+0x4]
  b9:	44 01 e0             	add    eax,r12d
  bc:	3d ff 00 00 00       	cmp    eax,0xff
  c1:	77 1d                	ja     e0 <pqcrystals_kyber1024_ref_ntt+0xe0>
  c3:	44 89 ea             	mov    edx,r13d
  c6:	48 8d 35 00 00 00 00 	lea    rsi,[rip+0x0]        # cd <pqcrystals_kyber1024_ref_ntt+0xcd>	c9: R_X86_64_PC32	pqcrystals_kyber1024_ref_zetas-0x4
  cd:	0f bf 1c 56          	movsx  ebx,WORD PTR [rsi+rdx*2]
  d1:	e9 7a ff ff ff       	jmp    50 <pqcrystals_kyber1024_ref_ntt+0x50>
  d6:	66 2e 0f 1f 84 00 00 00 00 00 	cs nop WORD PTR [rax+rax*1+0x0]
  e0:	d1 6c 24 04          	shr    DWORD PTR [rsp+0x4],1
  e4:	83 6c 24 1c 01       	sub    DWORD PTR [rsp+0x1c],0x1
  e9:	74 1a                	je     105 <pqcrystals_kyber1024_ref_ntt+0x105>
  eb:	44 89 e8             	mov    eax,r13d
  ee:	48 8d 15 00 00 00 00 	lea    rdx,[rip+0x0]        # f5 <pqcrystals_kyber1024_ref_ntt+0xf5>	f1: R_X86_64_PC32	pqcrystals_kyber1024_ref_zetas-0x4
  f5:	0f bf 1c 42          	movsx  ebx,WORD PTR [rdx+rax*2]
  f9:	e9 42 ff ff ff       	jmp    40 <pqcrystals_kyber1024_ref_ntt+0x40>
  fe:	66 90                	xchg   ax,ax
 100:	44 89 e0             	mov    eax,r12d
 103:	eb be                	jmp    c3 <pqcrystals_kyber1024_ref_ntt+0xc3>
 105:	48 83 c4 28          	add    rsp,0x28
 109:	5b                   	pop    rbx
 10a:	5d                   	pop    rbp
 10b:	41 5c                	pop    r12
 10d:	41 5d                	pop    r13
 10f:	41 5e                	pop    r14
 111:	41 5f                	pop    r15
 113:	c3                   	ret    
