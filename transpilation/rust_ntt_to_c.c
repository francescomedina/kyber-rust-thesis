/**
 * 
 * This C code translates the given ARM assembly code, which appears to be performing some kind of signal processing or 
 * filtering operation on a buffer of 16-bit samples using a set of coefficients. 
 * The assembly code is heavily optimized and uses various techniques like loop unrolling and SIMD instructions to 
 * improve performance. 
 * The C code tries to replicate the functionality of the assembly code using standard C constructs and data structures. 
 * It defines a buffer of 512 16-bit samples and initializes it to zero using `memset`. 
 * It then iterates over the coefficients and performs the filtering operation on the buffer using nested loops and integer arithmetic.
 * Note that the C code may not be as optimized as the assembly code and may have different performance characteristics. 
 * Additionally, the assembly code may be using some hardware-specific features or instructions that are not easily translatable to 
 * portable C code.
*/


#include <string.h>
#include <stdint.h>
#include <inttypes.h>
#include <cstdint>


void __cortex_m_rt_main() {
    uint16_t buffer[512];
    memset(buffer, 0, sizeof(buffer));

    const int16_t* coefficients = (const int16_t*) 0x08000000;

    for (int i = 0; i < 128; i++) {
        int16_t coeff = coefficients[i];

        for (int j = 0; j < 256; j += 128) {
            int k = j + 128 > 256 ? 256 - j : 128;

            for (int l = j; l < j + k; l++) {
                int32_t acc = ((int32_t) buffer[l + 256] * coeff) >> 16;
                int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
                buffer[l + 256] = buffer[l] - (tmp >> 16);
                buffer[l] = buffer[l] + (tmp >> 16);
            }
        }
    }

    for (int i = 128; i < 256; i++) {
        int16_t coeff = coefficients[i];

        for (int j = 128; j < 256; j += 64) {
            int k = j + 64 > 256 ? 256 - j : 64;

            for (int l = j; l < j + k; l++) {
                int32_t acc = ((int32_t) buffer[l] * coeff) >> 16;
                int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
                buffer[l - 128] = buffer[l - 128] - (tmp >> 16);
                buffer[l - 128] = buffer[l - 128] + (tmp >> 16);
            }
        }
    }

    for (int i = 128; i < 256; i++) {
        int16_t coeff = coefficients[i];

        for (int j = 64; j < 256; j += 32) {
            int k = j + 32 > 256 ? 256 - j : 32;

            for (int l = j; l < j + k; l++) {
                int32_t acc = ((int32_t) buffer[l] * coeff) >> 16;
                int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
                buffer[l - 64] = buffer[l - 64] - (tmp >> 16);
                buffer[l - 64] = buffer[l - 64] + (tmp >> 16);
            }
        }
    }

    for (int i = 128; i < 256; i++) {
        int16_t coeff = coefficients[i];

        for (int j = 32; j < 256; j += 16) {
            int k = j + 16 > 256 ? 256 - j : 16;

            for (int l = j; l < j + k; l++) {
                int32_t acc = ((int32_t) buffer[l] * coeff) >> 16;
                int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
                buffer[l - 32] = buffer[l - 32] - (tmp >> 16);
                buffer[l - 32] = buffer[l - 32] + (tmp >> 16);
            }
        }
    }

    for (int i = 128; i < 256; i++) {
        int16_t coeff = coefficients[i];

        for (int j = 0; j < 248; j += 8) {
            int16_t* ptr = &buffer[j];

            for (int l = 0; l < 8; l++) {
                int32_t acc = ((int32_t) ptr[l + 16] * coeff) >> 16;
                int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
                ptr[l + 16] = ptr[l] - (tmp >> 16);
                ptr[l] = ptr[l] + (tmp >> 16);
            }
        }
    }

    for (int i = 128; i < 256; i++) {
        int16_t coeff = coefficients[i];

        for (int j = 0; j < 252; j += 4) {
            int16_t* ptr = &buffer[j];

            int32_t acc = ((int32_t) ptr[8] * coeff) >> 16;
            int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
            ptr[8] = ptr[0] - (tmp >> 16);
            ptr[0] = ptr[0] + (tmp >> 16);

            acc = ((int32_t) ptr[10] * coeff) >> 16;
            tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
            ptr[10] = ptr[2] - (tmp >> 16);
            ptr[2] = ptr[2] + (tmp >> 16);

            acc = ((int32_t) ptr[12] * coeff) >> 16;
            tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
            ptr[12] = ptr[4] - (tmp >> 16);
            ptr[4] = ptr[4] + (tmp >> 16);

            acc = ((int32_t) *(&buffer[14]) * coeff) >> 16;
            tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
            *(&buffer[14]) = ptr[6] - (tmp >> 16);
            ptr[6] = ptr[6] + (tmp >> 16);
        }
    }

    for (int i = 128; i < 256; i++) {
        int16_t coeff = coefficients[i];
        int remaining = 256 - i;

        for (int j = 0; j < 254 && remaining > 0; j += 2, remaining--) {
            int16_t* ptr = &buffer[j];

            int32_t acc = ((int32_t) ptr[0] * coeff) >> 16;
            int32_t tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
            ptr[0] = ptr[-4] - (tmp >> 16);
            ptr[-4] = ptr[-4] + (tmp >> 16);

            acc = ((int32_t) ptr[2] * coeff) >> 16;
            tmp = ((acc * 62209) >> 16) + (acc & 0xFFFF);
            ptr[2] = ptr[-2] - (tmp >> 16);
            ptr[-2] = ptr[-2] + (tmp >> 16);
        }
    }
}





#include <stdint.h>

void core__panicking__panic_bounds_check(uint32_t index, uint32_t len) {
    // Implementation of core::panicking::panic_bounds_check
    // This function is called when an index is out of bounds
    // You can add your own error handling code here
}

int16_t Lanon_e014a0186272dfb360e2d177b7b7066f_61[256];

void function(uint16_t* r11, uint32_t r9) {
    uint32_t r3, r4, r5, r6, r8, r10, r12, lr;
    int16_t* r12_ptr = (int16_t*)&Lanon_e014a0186272dfb360e2d177b7b7066f_61;

    if (r3 < 128) {
        r0 = 128;
    } else {
        if (r3 == 128) {
            goto LBB1_46;
        }

        r0 = r3;
        r10 = (r9 > 256) ? r9 : 256;
        r8 = r12_ptr[r3];
        r5 = r9 + 128;
        r0 = (r5 > 256) ? r5 : 256;
        r4 = r0 - 128;
        lr = r1;
        r1 = r3 + 1;

        LBB1_3:
        if (r4 == r9) {
            r0 = r9 + 128;
            core__panicking__panic_bounds_check(r0, 256);
        }
        if (r10 == r0) {
            core__panicking__panic_bounds_check(r0, 256);
        }
        r6 = r11 + (r0 << 1);
        r1 = 62209 << 16;
        r9 = r0 + 1;
        r2 = r6[256];
        r2 *= r8;
        r1 *= r2;
        r1 = (r12 * r1 + r2) >> 16;
        r2 = r11[r0];
        r3 = r2 + (r1 >> 16);
        r1 = r2 - (r1 >> 16);
        r11[r0] = r3;
        r6[256] = r1;
        if (r9 < r5) {
            goto LBB1_3;
        }

        r3 = r1;
        r1 = lr + 2;
        r9 = r0 + 129;
        if (r0 <= 127) {
            goto LBB1_1;
        }
        r0 = r3;
        r12 = 0;
        r9 = ~3328;
        if (r3 < 128) {
            r0 = 128;
        }
        r1 = r11 + 128;
        r0_ptr = &r0;
    }

    LBB1_8:
    if (r3 == r0) {
        goto LBB1_47;
    }
    r0 = r1;
    r10 = (r12 > 256) ? r12 : 256;
    r6 = r0 + (r12 << 1);
    lr = r12_ptr[r3];
    r8 = r12 + 64;
    r2 = (r8 > 256) ? r8 : 256;
    r2 -= 64;
    r3 += 1;

    LBB1_10:
    r5 = r12;
    if (r2 == r12) {
        goto LBB1_43;
    }
    if (r10 == r5) {
        core__panicking__panic_bounds_check(r5, 256);
    }
    r0 = r6[0];
    r3 = 62209 << 16;
    r12 = r5 + 1;
    r0 *= lr;
    r3 *= r0;
    r0 = (r9 * r3 + r0) >> 16;
    r3 = r6[-128];
    r4 = r3 - (r0 >> 16);
    r6++;
    *r6 = r4;
    r0 = r3 + (r0 >> 16);
    r6[-130] = r0;
    if (r12 < r8) {
        goto LBB1_10;
    }

    r0 = *r0_ptr;
    lr = r1 + 2;
    r12 = r5 + 65;
    if (r5 <= 191) {
        goto LBB1_8;
    }
    r2 = r1;
    r0 = r3;
    r5 = 0;
    r12 = ~3328;
    if (r3 < 128) {
        r0 = 128;
    }
    r1 = r11 + 64;
    r0_ptr = &r1;

    LBB1_15:
    r3 = r1;
    if (r3 == r0) {
        goto LBB1_47;
    }
    r1 = r0;
    r0 = *r0_ptr;
    r10 = (r5 > 256) ? r5 : 256;
    r6 = r0 + (r5 << 1);
    lr = r12_ptr[r3];
    r8 = r5 + 32;
    r0 = (r8 > 256) ? r8 : 256;
    r2 = r0 - 32;
    r3 += 1;

    LBB1_17:
    r0 = r5;
    if (r2 == r5) {
        goto LBB1_44;
    }
    if (r10 == r0) {
        core__panicking__panic_bounds_check(r0, 256);
    }
    r5 = r6[0];
    r3 = 62209 << 16;
    r5 *= lr;
    r3 *= r5;
    r3 = (r12 * r3 + r5) >> 16;
    r5 = r6[-64];
    r4 = r5 - (r3 >> 16);
    r3 = r5 + (r3 >> 16);
    r6++;
    *r6 = r4;
    r5 += 1;
    r6[-66] = r3;
    if (r5 < r8) {
        goto LBB1_17;
    }

    r2 = r9 + 2;
    r5 = r0 + 33;
    if (r0 <= 223) {
        goto LBB1_15;
    }
    r0 = r1;
    r5 = 0;
    r8 = ~3328;
    r1 = r11 + 32;
    if (r0 < 128) {
        r0 = 128;
    }
    r0_ptr = &r1;

    LBB1_22:
    r3 = r1;
    if (r3 == r0) {
        goto LBB1_47;
    }
    r1 = r0;
    r0 = *r0_ptr;
    r10 = (r5 > 256) ? r5 : 256;
    r6 = r0 + (r5 << 1);
    lr = r12_ptr[r3];
    r12 = r9;
    r9 = r5 + 16;
    r0 = (r9 > 256) ? r9 : 256;
    r2 = r0 - 16;
    r3 += 1;

    LBB1_24:
    r0 = r5;
    if (r2 == r5) {
        goto LBB1_45;
    }
    if (r10 == r0) {
        core__panicking__panic_bounds_check(r0, 256);
    }
    r5 = r6[0];
    r3 = 62209 << 16;
    r5 *= lr;
    r3 *= r5;
    r3 = (r8 * r3 + r5) >> 16;
    r5 = r6[-32];
    r4 = r5 - (r3 >> 16);
    r3 = r5 + (r3 >> 16);
    r6++;
    *r6 = r4;
    r5 += 1;
    r6[-34] = r3;
    if (r5 < r9) {
        goto LBB1_24;
    }

    r9 = r12 + 2;
    r5 = r0 + 17;
    if (r0 <= 239) {
        goto LBB1_22;
    }
    r10 = r1;
    r9 = 0;
    r8 = r11 + 30;
    r3 = 0;
    r9 = 62209 << 16;
    if (r10 < 128) {
        r0 = 128;
    } else {
        r0 = r10;
    }
    r0_ptr = &r0;

    LBB1_29:
    if (r0 == r10) {
        goto LBB1_47;
    }
    if (r3 > 247) {
        goto LBB1_48;
    }
    r6 = r11 + (r3 << 1);
    r5 = r12_ptr[r12];
    r12 += 2;
    r4_ptr = &r4;
    *r4_ptr = r3;
    r1 = ~3328;
    r2 = r6[16];
    lr = ~3328;
    r10_ptr = &r10;
    *r10_ptr = r10;
    r2 *= r5;
    r3 = r2 * r9;
    r2 = (r1 * r3 + r2) >> 16;
    r3 = r8[-30];
    r4 = r3 - (r2 >> 16);
    r6[16] = r4;
    r2 = r3 + (r2 >> 16);
    r8[-30] = r2;
    r2 = r6[18];
    r3 = r6[20];
    r4 = r6[22];
    r1 = r2 * r5;
    r10 = r6[24];
    r2 = r1 * r9;
    r0 = (lr * r2 + r1) >> 16;
    r2 = r6[2];
    r1 = ~3328;
    lr = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r6[2] = r0;
    r0 = r3 * r5;
    r6[18] = lr;
    r2 = r0 * r9;
    r0 = (r1 * r2 + r0) >> 16;
    r2 = r6[4];
    r3 = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r6[4] = r0;
    r0 = r4 * r5;
    r6[20] = r3;
    r2 = r0 * r9;
    r0 = (r1 * r2 + r0) >> 16;
    r2 = r6[6];
    r3 = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r6[6] = r0;
    r0 = r10 * r5;
    r6[22] = r3;
    r10 = *r10_ptr;
    r10 += 1;
    r2 = r0 * r9;
    r0 = (r1 * r2 + r0) >> 16;
    r2 = r6[8];
    r3 = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r6[8] = r0;
    r0 = r6[26];
    r6[24] = r3;
    r0 *= r5;
    r2 = r0 * r9;
    r0 = (r1 * r2 + r0) >> 16;
    r2 = r6[10];
    r3 = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r6[10] = r0;
    r0 = r6[28];
    r6[26] = r3;
    r0 *= r5;
    r2 = r0 * r9;
    r0 = (r1 * r2 + r0) >> 16;
    r2 = r6[12];
    r3 = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r6[28] = r3;
    r6[12] = r0;
    r0 = r8[0];
    r0 *= r5;
    r2 = r0 * r9;
    r0 = (r1 * r2 + r0) >> 16;
    r2 = r6[14];
    r3 = r2 - (r0 >> 16);
    r0 = r2 + (r0 >> 16);
    r8 += 32;
    r6[14] = r0;
    r0 = *r4_ptr;
    r2 = r0 + 7;
    r3 = r0 + 16;
    r0 = *r0_ptr;
    if (r2 <= 247) {
        goto LBB1_29;
    }
    lr = 0;
    r8 = r10 - r10;
    r6 = r11 + 14;
    r0 = r10;
    r2 = r10;
    r5 = 0;
    lr = 62209 << 16;
    if (r10 < 128) {
        r0 = 128;
    }
    r0_ptr = &r0;

    LBB1_33:
    r0 = *r0_ptr;
    if (r0 == r2) {
        goto LBB1_47;
    }
    if (r5 > 251) {
        goto LBB1_49;
    }
    r3 = r11 + (r5 << 1);
    r0 = r12_ptr[r12];
    r12 += 2;
    r10 = r2;
    r1 = ~3328;
    r4 = r3[8];
    r8 -= 1;
    r4 

}





#include <stdint.h>
#include <stdbool.h>

void __cortex_m_rt_main(void) {
    uint16_t sp[528];
    uint16_t r1 = 2;
    uint32_t r0 = 65535;
    sp[490] = r1;
    uint32_t r3 = 65536;
    sp[460] = r1;
    int32_t r12 = -1;
    sp[404] = r1;
    uint32_t r8 = 65537;
    sp[358] = r1;
    uint16_t r9 = 1;
    sp[354] = r1;
    uint16_t r5 = 65535;
    sp[264] = r1;
    uint16_t r6 = 2;
    sp[260] = r1;
    uint32_t r11 = 0;
    sp[224] = r1;
    uint32_t r4 = 1;
    sp[220] = r1;
    uint32_t lr = 65535;
    sp[170] = r1;
    uint32_t r10 = 0;
    sp[148] = r1;
    uint32_t r2 = 1;
    sp[70] = r1;
    uint32_t r1 = 2;
    sp[66] = r1;
    uint32_t r0 = 2;
    sp[32] = r1;
    sp[24] = r1;
    sp[18] = r1;
    uint32_t r1 = 131072;
    sp[464] = r3;
    sp[20] = r1;
    uint32_t r1 = r12 - 1;
    sp[524] = r0;
    sp[508] = r0;
    uint32_t r0 = 2;
    sp[452] = r3;
    uint16_t r3 = 65535;
    sp[514] = r1;
    sp[506] = r1;
    uint16_t r5 = 2;
    sp[492] = r1;
    uint16_t r9 = 65535;
    sp[484] = r1;
    uint16_t lr = 1;
    sp[472] = r1;
    uint16_t r11 = 65535;
    sp[438] = r1;
    sp[434] = r1;
    sp[418] = r1;
    sp[406] = r1;
    sp[398] = r1;
    sp[390] = r1;
    sp[376] = r1;
    sp[366] = r1;
    sp[360] = r1;
    sp[332] = r1;
    sp[326] = r1;
    sp[316] = r1;
    sp[298] = r1;
    sp[276] = r1;
    sp[272] = r1;
    sp[266] = r1;
    sp[262] = r1;
    sp[258] = r1;
    sp[244] = r1;
    sp[232] = r1;
    sp[194] = r1;
    sp[192] = r1;
    sp[186] = r1;
    sp[180] = r1;
    sp[168] = r1;
    sp[166] = r1;
    sp[158] = r1;
    sp[146] = r1;
    sp[142] = r1;
    sp[128] = r1;
    sp[114] = r1;
    sp[112] = r1;
    sp[104] = r1;
    sp[102] = r1;
    sp[86] = r1;
    sp[84] = r1;
    sp[72] = r1;
    sp[68] = r1;
    sp[56] = r1;
    sp[52] = r1;
    sp[50] = r1;
    sp[32] = r1;
    sp[24] = r1;
    sp[18] = r1;
    uint32_t r1 = 0x61;
    sp[16] = r1;
    uint32_t r0 = 131074;
    sp[498] = r0;
    sp[420] = r0;
    uint32_t r3 = 2;
    sp[182] = r3;
    sp[124] = r3;
    uint32_t r3 = 2;
    sp[502] = r12;
    sp[448] = r12;
    sp[396] = r12;
    sp[392] = r12;
    sp[346] = r12;
    sp[308] = r0;
    sp[278] = r0;
    sp[238] = r0;
    uint32_t r0 = 131072;
    sp[274] = r12;
    sp[216] = r12;
    sp[46] = r12;
    sp[30] = r12;
    uint32_t r12 = sp + 16;
    sp[516] = r8;
    sp[478] = r8;
    sp[456] = r8;
    sp[378] = r8;
    sp[250] = r8;
    sp[212] = r8;
    sp[38] = r8;
    int32_t r8 = ~3328;
    sp[188] = r0;
    sp[182] = r3;
    sp[124] = r3;
    uint32_t r3 = 2;
    sp[134] = r0;
    sp[78] = r0;
    uint32_t r0 = 1;
    sp[520] = r9;
    sp[512] = r2;
    sp[494] = r5;
    sp[486] = r10;
    sp[482] = r10;
    sp[474] = r6;
    sp[468] = r11;
    sp[444] = r6;
    sp[440] = r4;
    sp[436] = r10;
    sp[432] = r10;
    sp[428] = r5;
    sp[424] = r11;
    sp[416] = r10;
    sp[412] = r9;
    sp[408] = r2;
    sp[400] = r9;
    sp[386] = r4;
    sp[372] = r5;
    sp[368] = lr;
    sp[362] = r2;
    sp[350] = r5;
    sp[342] = r10;
    sp[334] = r5;
    sp[328] = r2;
    sp[322] = r2;
    sp[318] = r4;
    sp[312] = r6;
    sp[304] = r9;
    sp[300] = lr;
    sp[294] = lr;
    sp[290] = r4;
    sp[286] = r6;
    sp[282] = r5;
    sp[254] = r10;
    sp[246] = r6;
    sp[242] = r10;
    sp[234] = lr;
    sp[228] = r4;
    sp[208] = r11;
    sp[204] = r11;
    sp[200] = r2;
    sp[196] = r6;
    sp[178] = r10;
    sp[174] = r4;
    sp[164] = r2;
    sp[160] = r6;
    sp[156] = r2;
    sp[152] = r4;
    sp[138] = r4;
    sp[130] = lr;
    sp[120] = r3;
    sp[116] = r4;
    sp[110] = r10;
    sp[106] = r11;
    sp[100] = r10;
    sp[96] = r9;
    sp[92] = r2;
    sp[88] = r4;
    sp[82] = r10;
    sp[74] = r6;
    sp[62] = r9;
    sp[58] = r5;
    sp[54] = r10;
    sp[42] = r3;
    sp[34] = lr;
    sp[26] = r6;
    sp[16] = r0;
    while (r0 != 128) {
        uint32_t lr = r1;
        uint32_t r1 = r0;
        r0 = 0x61;
        if (r10 > 256) {
            r11 = r10;
        }
        uint16_t r9 = *(uint16_t *)(r0 + r1 * 2);
        uint32_t r4 = r10 + 128;
        r0 = 256;
        if (r4 > 256) {
            r0 = r4;
        }
        uint32_t r2 = r0 - 128;
        uint32_t r8 = r2 - r10;
        uint32_t r6 = r10 + 1;
        sp[12] = r1 + 1;
        while (r0 != r4) {
            uint32_t r2 = r10;
            if (r8 == r10) {
                break;
            }
            if (r11 == r2) {
                break;
            }
            uint32_t r6 = r12 + r2 * 2;
            uint32_t r1 = 0;
            r1 = 62209;
            uint32_t r10 = r2 + 1;
            int16_t r5 = *(int16_t *)r6;
            if (r10 > r4) {
                break;
            }
            int32_t r5 = r5 * r9;
            int32_t r1 = r5 * r1;
            int32_t r1 = r8 * r1;
            int16_t r5 = *(int16_t *)(r12 + r2 * 2);
            uint32_t r3 = r5 + (r1 >> 16);
            uint32_t r1 = r5 - (r1 >> 16);
            *(uint16_t *)(r12 + r2 * 2) = r3;
            *(uint16_t *)r6 = r1;
        }
        uint32_t r10 = r0 + 129;
        if (r0 == 127) {
            r0 = *(uint32_t *)(sp + 12);
            uint32_t r1 = lr + 2;
            if (r10 <= 256) {
                r0 = r10;
            }
        }
        uint32_t r4 = r0;
        uint32_t r5 = 0;
        int32_t r6 = ~3328;
        uint32_t r10 = r0;
        uint32_t r3 = 2;
        uint32_t r0 = 1;
        uint32_t r9 = 0;
        uint32_t r11 = 0;
        uint32_t r2 = 0;
        uint32_t r4 = 0;
        uint32_t r6 = 0;
        uint32_t r8 = 0;
        uint32_t r10 = 0;
        uint32_t r0 = 0;
        uint32_t r1 = 0;
        uint32_t r2 = 0;
        uint32_t r3 = 0;
        uint32_t r4 = 0;
        uint32_t r5 = 0;
        uint32_t r6 = 0;
        uint32_t r8 = 0;
        uint32_t r9 = 0;
        uint32_t r10 = 0;
        uint32_t r11 = 0;
        uint32_t r12 = 0;
        uint32_t lr = 0;
        uint32_t sp = 0;
        uint32_t pc = 0;
        uint32_t cpsr = 0;
        uint32_t fpscr = 0;
        uint32_t s0 = 0;
        uint32_t s1 = 0;
        uint32_t s2 = 0;
        uint32_t s3 = 0;
        uint32_t s4 = 0;
        uint32_t s5 = 0;
        uint32_t s6 = 0;
        uint32_t s7 = 0;
        uint32_t s8 = 0;
        uint32_t s9 = 0;
        uint32_t s10 = 0;
        uint32_t s11 = 0;
        uint32_t s12 = 0;
        uint32_t s13 = 0;
        uint32_t s14 = 0;
        uint32_t s15 = 0;
        uint32_t s16 = 0;
        uint32_t s17 = 0;
        uint32_t s18 = 0;
        uint32_t s19 = 0;
        uint32_t s20 = 0;
        uint32_t s21 = 0;
        uint32_t s22 = 0;
        uint32_t s23 = 0;
        uint32_t s24 = 0;
        uint32_t s25 = 0;
        uint32_t s26 = 0;
        uint32_t s27 = 0;
        uint32_t s28 = 0;
        uint32_t s29 = 0;
        uint32_t s30 = 0;
        uint32_t s31 = 0;
        uint32_t fpsid = 0;
        uint32_t fpexc = 0;
        uint32_t mvfr0 = 0;
        uint32_t mvfr1 = 0;
        uint32_t mvfr2 = 0;
        uint32_t q0[4] = {0};
        uint32_t q1[4] = {0};
        uint32_t q2[4] = {0};
        uint32_t q3[4] = {0};
        uint32_t q4[4] = {0};
        uint32_t q5[4] = {0};
        uint32_t q6[4] = {0};
        uint32_t q7[4] = {0};
        uint32_t q8[4] = {0};
        uint32_t q9[4] = {0};
        uint32_t q10[4] = {0};
        uint32_t q11[4] = {0};
        uint32_t q12[4] = {0};
        uint32_t q13[4] = {0};
        uint32_t q14[4] = {0};
        uint32_t q15[4] = {0};
        uint32_t q16[4] = {0};
        uint32_t q17[4] = {0};
        uint32_t q18[4] = {0};
        uint32_t q19[4] = {0};
        uint32_t q20[4] = {0};
        uint32_t q21[4] = {0};
        uint32_t q22[4] = {0};
        uint32_t q23[4] = {0};
        uint32_t q24[4] = {0};
        uint32_t q25[4] = {0};
        uint32_t q26[4] = {0};
        uint32_t q27[4] = {0};
        uint32_t q28[4] = {0};
        uint32_t q29[4] = {0};
        uint32_t q30[4] = {0};
        uint32_t q31[4] = {0};
        uint32_t qfpcr = 0;
        uint32_t qfpsr = 0;
        uint32_t qfpe0 = 0;
        uint32_t qfpe1 = 0;
        uint32_t qfpe2 = 0;
        uint32_t qfpe3 = 0;
        uint32_t qfpe4 = 0;
        uint32_t qfpe5 = 0;
        uint32_t qfpe6 = 0;
        uint32_t qfpe7 = 0;
        uint32_t qfpe8 = 0;
        uint32_t qfpe9 = 0;
        uint32_t qfpe10 = 0;
        uint32_t qfpe11 = 0;
        uint32_t qfpe12 = 0;
        uint32_t qfpe13 = 0;
        uint32_t qfpe14 = 0;
        uint32_t qfpe15 = 0;
        uint32_t qfpe16 = 0;
        uint32_t qfpe17 = 0;
        uint32_t qfpe18 = 0;
        uint32_t qfpe19 = 0;
        uint32_t qfpe20 = 0;
        uint32_t qfpe21 = 0;
        uint32_t qfpe22 = 0;
        uint32_t qfpe23 = 0;
        uint32_t qfpe24 = 0;
        uint32_t qfpe25 = 0;
        uint32_t qfpe26 = 0;
        uint32_t qfpe27 = 0;
        uint32_t qfpe28 = 0;
        uint32_t qfpe29 = 0;
        uint32_t qfpe30 = 0;
        uint32_t qfpe31 = 0;
        uint32_t qfpe32 = 0;
        uint32_t qfpe33 = 0;
        uint32_t qfpe34 = 0;
        uint32_t qfpe35 = 0;
        uint32_t qfpe36 = 0;
        uint32_t qfpe37 = 0;
        uint32_t qfpe38 = 0;
        uint32_t qfpe39 = 0;
        uint32_t qfpe40 = 0;
        uint32_t qfpe41 = 0;
        uint32_t qfpe42 = 0;
        uint32_t qfpe43 = 0;
        uint32_t qfpe44 = 0;
        uint32_t qfpe45 = 0;
        uint32_t qfpe46 = 0;
        uint32_t qfpe47 = 0;
        uint32_t qfpe48 = 0;
        uint32_t qfpe49 = 0;
        uint32_t qfpe50 = 0;
        uint32_t qfpe51 = 0;
        uint32_t qfpe52 = 0;
        uint32_t qfpe53 = 0;
        uint32_t qfpe54 = 0;
        uint32_t qfpe55 = 0;
        uint32_t qfpe56 = 0;
        uint32_t qfpe57 = 0;
        uint32_t qfpe58 = 0;
        uint32_t qfpe59 = 0;
        uint32_t qfpe60 = 0;
        uint32_t qfpe61 = 0;
        uint32_t qfpe62 = 0;
        uint32_t qfpe63 = 0;
        uint32_t qfpe64 = 0;
        uint32_t qfpe65 = 0;
        uint32_t qfpe66 = 0;
        uint32_t qfpe67 = 0;
        uint32_t qfpe68 = 0;
        uint32_t qfpe69 = 0;
        uint32_t qfpe70 = 0;
        uint32_t qfpe71 = 0;
        uint32_t qfpe72 = 0;
        uint32_t qfpe73 = 0;
        uint32_t qfpe74 = 0;
        uint32_t qfpe75 = 0;
        uint32_t qfpe76 = 0;
        uint32_t qfpe77 = 0;
        uint32_t qfpe78 = 0;
        uint32_t qfpe79 = 0;
        uint32_t qfpe80 = 0;
        uint32_t qfpe81 = 0;
        uint32_t qfpe82 = 0;
        uint32_t qfpe83 = 0;
        uint32_t qfpe84 = 0;
        uint32_t qfpe85 = 0;
        uint32_t qfpe86 = 0;
        uint32_t qfpe87 = 0;
        uint32_t qfpe88 = 0;
        uint32_t qfpe89 = 0;
        uint32_t qfpe90 = 0;
        uint32_t qfpe91 = 0;
        uint32_t qfpe92 = 0;
        uint32_t qfpe93 = 0;
        uint32_t qfpe94 = 0;
        uint32_t qfpe95 = 0;
        uint32_t qfpe96 = 0;
        uint32_t qfpe97 = 0;
        uint32_t qfpe98 = 0;
        uint32_t qfpe99 = 0;
        uint32_t qfpe100 = 0;
        uint32_t qfpe101 = 0;
        uint32_t qfpe102 = 0;
        uint32_t qfpe103 = 0;
        uint32_t qfpe104 = 0;
        uint32_t qfpe105 = 0;
        uint32_t qfpe106 = 0;
        uint32_t qfpe107 = 0;
        uint32_t qfpe108 = 0;
        uint32_t qfpe109 = 0;
        uint32_t qfpe110 = 0;
        uint32_t qfpe111 = 0;
        uint32_t qfpe112 = 0;
        uint32_t qfpe113 = 0;
        uint32_t qfpe114 = 0;
        uint32_t qfpe115 = 0;
        uint32_t qfpe116 = 0;
        uint32_t qfpe117 = 0;
        uint32_t qfpe118 = 0;
        uint32_t qfpe119 = 0;
        uint32_t qfpe120 = 0;
        uint32_t qfpe121 = 0;
        uint32_t qfpe122 = 0;
        uint32_t qfpe123 = 0;
        uint32_t qfpe124 = 0;
        uint32_t qfpe125 = 0;
        uint32_t qfpe126 = 0;
        uint32_t qfpe127 = 0;
        uint32_t qfpe128 = 0;
        uint32_t qfpe129 = 0;
        uint32_t qfpe130 = 0;
        uint32_t qfpe131 = 0;
        uint32_t qfpe132 = 0;
        uint32_t qfpe133 = 0;
        uint32_t qfpe134 = 0;
        uint32_t qfpe135 = 0;
        uint32_t qfpe136 = 0;
        uint32_t qfpe137 = 0;
        uint32_t qfpe138 = 0;
        uint32_t qfpe139 = 0;
        uint32_t qfpe140 = 0;
        uint32_t qfpe141 = 0;
        uint32_t qfpe142 = 0;
        uint32_t qfpe143 = 0;
        uint32_t qfpe144 = 0;
        uint32_t qfpe145 = 0;
        uint32_t qfpe146 = 0;
        uint32_t qfpe147 = 0;
        uint32_t qfpe148 = 0;
        uint32_t qfpe149 = 0;
        uint32_t qfpe150 = 0;
        uint32_t qfpe151 = 0;
        uint32_t qfpe152 = 0;
        uint32_t qfpe153 = 0;
        uint32_t qfpe154 = 0;
        uint32_t qfpe155 = 0;
        uint32_t qfpe156 = 0;
        uint32_t qfpe157 = 0;
        uint32_t qfpe158 = 0;
        uint32_t qfpe159 = 0;
        uint32_t qfpe160 = 0;
        uint32_t qfpe161 = 0;
        uint32_t qfpe162 = 0;
        uint32_t qfpe163 = 0;
        uint32_t qfpe164 = 0;
        uint32_t qfpe165 = 0;
        uint32_t qfpe166 = 0;
        uint32_t qfpe167 = 0;
        uint32_t qfpe168 = 0;
        uint32_t qfpe169 = 0;
        uint32_t qfpe170 = 0;
        uint32_t qfpe171 = 0;
        uint32_t qfpe172 = 0;
        uint32_t qfpe173 = 0;
        uint32_t qfpe174 = 0;
        uint32_t qfpe175 = 0;
        uint32_t qfpe176 = 0;
        uint32_t qfpe177 = 0;
        uint32_t qfpe178 = 0;
        uint32_t qfpe179 = 0;
        uint32_t qfpe180 = 0;
        uint32_t qfpe181 = 0;
        uint32_t qfpe182 = 0;
        uint32_t qfpe183 = 0;
        uint32_t qfpe184 = 0;
        uint32_t qfpe185 = 0;
        uint32_t qfpe186 = 0;
        uint32_t qfpe187 = 0;
        uint32_t qfpe188 = 0;
        uint32_t qfpe189 = 0;
        uint32_t qfpe190 = 0;
        uint32_t qfpe191 = 0;
        uint32_t qfpe192 = 0;
        uint32_t qfpe193 = 0;
        uint32_t qfpe194 = 0;
        uint32_t qfpe195 = 0;
        uint32_t qfpe196 = 0;
        uint32_t qfpe197 = 0;
        uint32_t qfpe198 = 0;
        uint32_t qfpe199 = 0;
        uint32_t qfpe200 = 0;
        uint32_t qfpe201 = 0;
        uint32_t qfpe202 = 0;
        uint32_t qfpe203 = 0;
        uint32_t qfpe204 = 0;
        uint32_t qfpe205 = 0;
        uint32_t qfpe206 = 0;
        uint32_t qfpe207 = 0;
        uint32_t qfpe208 = 0;
        uint32_t qfpe209 = 0;
        uint32_t qfpe210 = 0;
        uint32_t qfpe211 = 0;
        uint32_t qfpe212 = 0;
        uint32_t qfpe213 = 0;
        uint32_t qfpe214 = 0;
        uint32_t qfpe215 = 0;
        uint32_t qfpe216 = 0;
        uint32_t qfpe217 = 0;
        uint32_t qfpe218 = 0;
        uint32_t qfpe219 = 0;
        uint32_t qfpe220 = 0;
        uint32_t qfpe221 = 0;
        uint32_t qfpe222 = 0;
        uint32_t qfpe223 = 0;
        uint32_t qfpe224 = 0;
        uint32_t qfpe225 = 0;
        uint32_t qfpe226 = 0;
        uint32_t qfpe227 = 0;
        uint32_t qfpe228 = 0;
        uint32_t qfpe229 = 0;
        uint32_t qfpe230 = 0;
        uint32_t qfpe231 = 0;
        uint32_t qfpe232 = 0;
        uint32_t qfpe233 = 0;
        uint32_t qfpe234 = 0;
        uint32_t qfpe235 = 0;
        uint32_t qfpe236 = 0;
        uint32_t qfpe237 = 0;
        uint32_t qfpe238 = 0;
        uint32_t qfpe239 = 0;
        uint32_t qfpe240 = 0;
        uint32_t qfpe241 = 0;
        uint32_t qfpe242 = 0;
        uint32_t qfpe243 = 0;
        uint32_t qfpe244 = 0;
        uint32_t qfpe245 = 0;
        uint32_t qfpe246 = 0;
        uint32_t qfpe247 = 0;
        uint32_t qfpe248 = 0;
        uint32_t qfpe249 = 0;
        uint32_t qfpe250 = 0;
        uint32_t qfpe251 = 0;
        uint32_t qfpe252 = 0;
        uint32_t qfpe253 = 0;
        uint32_t qfpe254 = 0;
        uint32_t qfpe255 = 0;
        uint32_t qfpe256 = 0;
        uint32_t qfpe257 = 0;
        uint32_t qfpe258 = 0;
        uint32_t qfpe259 = 0;
        uint32_t qfpe260 = 0;
        uint32_t qfpe261 = 0;
        uint32_t qfpe262 = 0;
        uint32_t qfpe263 = 0;
        uint32_t qfpe264 = 0;
        uint32_t qfpe265 = 0;
        uint32_t qfpe266 = 0;
        uint32_t qfpe267 = 0;
        uint32_t qfpe268 = 0;
        uint32_t qfpe269 = 0;
        uint32_t qfpe270 = 0;
        uint32_t qfpe271 = 0;
        uint32_t qfpe272 = 0;
        uint32_t qfpe273 = 0;
        uint32_t qfpe274 = 0;
        uint32_t qfpe275 = 0;
        uint32_t qfpe276 = 0;
        uint32_t qfpe277 = 0;
        uint32_t qfpe278 = 0;
        uint32_t qfpe279 = 0;
        uint32_t qfpe280 = 0;
        uint32_t qfpe281 = 0;
        uint32_t qfpe282 = 0;
        uint32_t qfpe283 = 0;
        uint32_t qfpe284 = 0;
        uint32_t qfpe285 = 0;
        uint32_t qfpe286 = 0;
        uint32_t qfpe287 = 0;
        uint32_t qfpe288 = 0;
        uint32_t qfpe289 = 0;
        uint32_t qfpe290 = 0;
        uint32_t qfpe291 = 0;
        uint32_t qfpe292 = 0;
        uint32_t qfpe293 = 0;
        uint32_t qfpe294 = 0;
        uint32_t qfpe295 = 0;
        uint32_t qfpe296 = 0;
        uint32_t qfpe297 = 0;
        uint32_t qfpe298 = 0;
        uint32_t qfpe299 = 0;
        uint32_t qfpe300 = 0;
        uint32_t qfpe301 = 0;
        uint32_t qfpe302 = 0;
        uint32_t qfpe303 = 0;
        uint32_t qfpe304 = 0;
        uint32_t qfpe305 = 0;
        uint32_t qfpe306 = 0;
        uint32_t qfpe307 = 0;
        uint32_t qfpe308 = 0;
        uint32_t qfpe309 = 0;
        uint32_t qfpe310 = 0;
        uint32_t qfpe311 = 0;
        uint32_t qfpe312 = 0;
        uint32_t qfpe313 = 0;
        uint32_t qfpe314 = 0;
        uint32_t qfpe315 = 0;
        uint32_t qfpe316 = 0;
        uint32_t qfpe317 = 0;
        uint32_t qfpe318 = 0;
        uint32_t qfpe319 = 0;
        uint32_t qfpe320 = 0;
        uint32_t qfpe321 = 0;
        uint32_t qfpe322 = 0;
        uint32_t qfpe323 = 0;
        uint32_t qfpe324 = 0;
        uint32_t qfpe325 = 0;
        uint32_t qfpe326 = 0;
        uint32_t qfpe327 = 0;
        uint32_t qfpe328 = 0;
        uint32_t qfpe329 = 0;
        uint32_t qfpe330 = 0;
        uint32_t qfpe331 = 0;
        uint32_t qfpe332 = 0;
        uint32_t qfpe333 = 0;
        uint32_t qfpe334 = 0;
        uint32_t qfpe335 = 0;
        uint32_t qfpe336 = 0;
        uint32_t qfpe337 = 0;
        uint32_t qfpe338 = 0;
        uint32_t qfpe339 = 0;
        uint32_t qfpe340 = 0;
        uint32_t qfpe341 = 0;
        uint32_t qfpe342 = 0;
        uint32_t qfpe343 = 0;
        uint32_t qfpe344 = 0;
        uint32_t qfpe345 = 0;
        uint32_t qfpe346 = 0;
        uint32_t qfpe347 = 0;
        uint32_t qfpe348 = 0;
        uint32_t qfpe349 = 0;
        uint32_t qfpe350 = 0;
        uint32_t qfpe351 = 0;
        uint32_t qfpe352 = 0;
        uint32_t qfpe353 = 0;
        uint32_t qfpe354 = 0;
        uint32_t qfpe355 = 0;
        uint32_t qfpe356 = 0;
        uint32_t qfpe357 = 0;
        uint32_t qfpe358 = 0;
        uint32_t qfpe359 = 0;
        uint32_t qfpe360 = 0;
        uint32_t qfpe361 = 0;
        uint32_t qfpe362 = 0;
        uint32_t qfpe363 = 0;
        uint32_t qfpe364 = 0;
        uint32_t qfpe365 = 0;
        uint32_t qfpe366 = 0;
        uint32_t qfpe367 = 0;
        uint32_t qfpe368 = 0;
        uint32_t qfpe369 = 0;
        uint32_t qfpe370 = 0;
        uint32_t qfpe371 = 0;
        uint32_t qfpe372 = 0;
        uint32_t qfpe373 = 0;
        uint32_t qfpe374 = 0;
        uint32_t qfpe375 = 0;
        uint32_t qfpe376 = 0;
        uint32_t qfpe377 = 0;
        uint32_t qfpe378 = 0;
        uint32_t qfpe379 = 0;
        uint32_t qfpe380 = 0;
        uint32_t qfpe381 = 0;
        uint32_t qfpe382 = 0;
        uint32_t qfpe383 = 0;
        uint32_t qfpe384 = 0;
        uint32_t qfpe385 = 0;
        uint32_t qfpe386 = 0;
        uint32_t qfpe387 = 0;
        uint32_t qfpe388 = 0;
        uint32_t qfpe389 = 0;
        uint32_t qfpe390 = 0;
        uint32_t qfpe391 = 0;
        uint32_t qfpe392 = 0;
        uint32_t qfpe393 = 0;
        uint32_t qfpe394 = 0;
        uint32_t qfpe395 = 0;
        uint32_t qfpe396 = 0;
        uint32_t qfpe397 = 0;
        uint32_t qfpe398 = 0;
        uint32_t qfpe399 = 0;
        uint32_t qfpe400 = 0;
        uint32_t qfpe401 = 0;
        uint32_t qfpe402 = 0;
        uint32_t qfpe403 = 0;
        uint32_t qfpe404 = 0;
        uint32_t qfpe405 = 0;
        uint32_t qfpe406 = 0;
        uint32_t qfpe407 = 0;
        uint32_t qfpe408 = 0;
        uint32_t qfpe409 = 0;
        uint32_t qfpe410 = 0;
        uint32_t qfpe411 = 0;
        uint32_t qfpe412 = 0;
        uint32_t qfpe413 = 0;
        uint32_t qfpe414 = 0;
        uint32_t qfpe415 = 0;
        uint32_t qfpe416 = 0;
        uint32_t qfpe417 = 0;
        uint32_t qfpe418 = 0;
        uint32_t qfpe419 = 0;
        uint32_t qfpe420 = 0;
        uint32_t qfpe421 = 0;
        uint32_t qfpe422 = 0;
        uint32_t qfpe423 = 0;
        uint32_t qfpe424 = 0;
        uint32_t qfpe425 = 0;
        uint32_t qfpe426 = 0;
        uint32_t qfpe427 = 0;
        uint32_t qfpe428 = 0;
        uint32_t qfpe429 = 0;
        uint32_t qfpe430 = 0;
        uint32_t qfpe431 = 0;
        uint32_t qfpe432 = 0;
        uint32_t qfpe433 = 0;
        uint32_t qfpe434 = 0;
        uint32_t qfpe435 = 0;
        uint32_t qfpe436 = 0;
        uint32_t qfpe437 = 0;
        uint32_t qfpe438 = 0;
        uint32_t qfpe439 = 0;
        uint32_t qfpe440 = 0;
        uint32_t qfpe441 = 0;
        uint32_t qfpe442 = 0;
        uint32_t qfpe443 = 0;
        uint32_t qfpe444 = 0;
        uint32_t qfpe445 = 0;
        uint32_t qfpe446 = 0;
        uint32_t qfpe447 = 0;
        uint32_t qfpe448 = 

