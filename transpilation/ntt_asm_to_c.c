#include <stdint.h>

void pqcrystals_kyber1024_ref_montgomery_reduce(int16_t *);

void pqcrystals_kyber1024_ref_ntt(int16_t *a) {
    int16_t r15, r14, r13, r12, rbp, rbx;
    int32_t edx, eax, ecx, esi;
    int16_t *r14_ptr, *rbp_ptr;
    int16_t edi, ebx;

    r15 = a[0];
    a++;
    r14 = a[0];
    r13 = 1;
    r12 = 0;
    rbp = 0;
    rbx = -502;
    a -= 2;

    int32_t *rsp_ptr = (int32_t *)a;
    rsp_ptr[2] = 0x80;
    rsp_ptr[4] = 7;

    r14_ptr = a + 2;
    rbp_ptr = a + 4;

    while (1) {
        edx = rsp_ptr[2];
        eax = 0;
        edx--;
        rsp_ptr[6] = edx;

        edx = rsp_ptr[2];
        r13++;
        ecx = edx + eax;
        if (ecx <= eax)
            break;

        esi = ecx - eax;
        ecx += eax;
        r15 = esi;
        r14_ptr = a + ecx;
        rbp_ptr = a + ecx * 2;

        do {
            edi = *r14_ptr;
            edi *= rbx;
            pqcrystals_kyber1024_ref_montgomery_reduce(&edi);
            ecx = *r14_ptr;
            ecx -= eax;
            *r14_ptr = (int16_t)ecx;
            *r14_ptr += (int16_t)eax;
            r14_ptr += 1;
        } while (rbp_ptr != r14_ptr);

        edx = rsp_ptr[2];
        edx += ecx;
        if (edx > 0xFF)
            break;

        ebx = *(int16_t *)(pqcrystals_kyber1024_ref_zetas + r13 * 2);
    }

    a -= 2;
    a[0] = r15;
    a[1] = r14;
}


// ALTERNATIVE

#include <stdint.h>
#include <stddef.h>

void pqcrystals_kyber1024_ref_ntt(uint16_t *r) {
    size_t i, j, k;
    uint16_t t, zeta;
    const uint16_t zetas[128] = { /* precomputed zetas */ };
    const uint16_t q = 3329; // Example modulus, replace with actual value if different

    for (i = 1; i < 128; i <<= 1) {
        for (j = 0; j < 256; j = k) {
            zeta = zetas[i + (j >> 1)];
            for (k = j; k < j + i; ++k) {
                t = r[k + i] * zeta;
                r[k + i] = r[k] + q - t;
                r[k] = r[k] + t;
            }
        }
    }
}