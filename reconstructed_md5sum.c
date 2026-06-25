#include <stdio.h>
#include <stdint.h>
#include <string.h>

// Forward declarations
void func_0x215f(uint64_t arg_rdi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x21ca(uint64_t arg_rcx);
void func_0x2277(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x237c(uint64_t arg_r9);
void func_0x2470(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x36d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_r9);
void func_0x3770(uint64_t arg_rdi);
void func_0x37c0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x3ed0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x4410(void);
void func_0x4420(void);
void func_0x4430(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x44f0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_r8);
void func_0x4630(void);
void func_0x4640(void);
void func_0x4680(uint64_t arg_rdi, uint64_t arg_rsi);
void func_0x4740(void);
void func_0x4760(void);
void func_0x4770(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x4800(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x63b0(uint64_t arg_rdi);
void func_0x6400(void);
void func_0x6420(void);
void func_0x6440(uint64_t arg_rdi);
void func_0x6480(void);
void func_0x64a0(void);
void func_0x64e0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x6560(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x6650(uint64_t arg_rcx);
void func_0x6660(uint64_t arg_rdi);
void func_0x6710(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6730(uint64_t arg_rcx);
void func_0x6740(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6760(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6780(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x67f0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6860(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x68d0(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6940(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x69e0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6a80(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6b10(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6b90(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6c10(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6cb0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6d50(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6df0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6e90(uint64_t arg_rcx);
void func_0x6ea0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6ec0(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6ee0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6f00(void);
void func_0x6f20(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x7430(void);
void func_0x7450(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x7500(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x75e0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x76a0(void);
void func_0x76c0(void);
void func_0x76e0(void);
void func_0x7700(void);
void func_0x7720(void);
void func_0x7750(void);
void func_0x7780(void);
void func_0x77a0(void);
void func_0x77c0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x77f0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x7820(uint64_t arg_rsi);
void func_0x7880(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x78f0(uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r9);
void func_0x79e0(void);
void func_0x7a00(void);
void func_0x7a20(void);
void func_0x7a40(void);
void func_0x7a60(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x7aa0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x7ae0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x7b30(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x7b80(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x7bd0(uint64_t arg_rdi);
void func_0x7c40(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x7ec0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x84e0(uint64_t arg_rsi);
void func_0x8520(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x85a0(uint64_t arg_rdx);
void func_0x85d0(void);
void func_0x85e0(void);
void func_0x85f0(void);
void func_0x8600(void);
void func_0x8610(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x86c0(uint64_t arg_rdx);

void func_0x215f(uint64_t arg_rdi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x4760();
}

void func_0x21ca(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x215f(0, 0, 0);
    return;
}

void func_0x2277(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x4760();
    return;
}

void func_0x237c(uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x2277(0, 0, 0, 0, 0);
    return;
    while (i > min) {
    }  // end loop
}

void func_0x2470(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x4770(0, 0);
    // data reference
    // data reference
    // data reference
    func_0x86c0(0);
    func_0x4800(0, 0, 0, 0, 0, 0);
    func_0x4800(0, 0, 0, 0, 0, 0);
    func_0x4800(0, 0, 0, 0, 0, 0);
    func_0x7500(0, 0, 0, 0);
    while (result == 0) {
    }  // end loop
    return;
    // data reference
    // data reference
    // data reference
    while (result != 0) {
        func_0x6b90(0, 0, 0);
        func_0x4680(0, 0);
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
        func_0x6b90(0, 0, 0);
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
        func_0x21ca(0);
        func_0x3ed0(0, 0, 0, 0);
        // data reference
        // data reference
        func_0x3ed0(0, 0, 0, 0);
        func_0x21ca(0);
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
        func_0x21ca(0);
        func_0x21ca(0);
        func_0x6b90(0, 0, 0);
        func_0x6b90(0, 0, 0);
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
    }  // end loop
}

void func_0x36d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    // data reference
    return;
    // data reference
    return;
}

void func_0x3770(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    return;
}

void func_0x37c0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i > min) {
    }  // end loop
    return;
    return;
    func_0x4740();
    func_0x21ca(0);
    while (result != 0) {
        // data reference
        // data reference
    }  // end loop
    return;
    // data reference
    return;
    while (result != 0) {
        // data reference
        // data reference
        // data reference
        // data reference
        // data reference
        func_0x4640();
        func_0x44f0(0, 0, 0, 0);
    }  // end loop
    return;
    func_0x4680(0, 0);
    func_0x4640();
    func_0x44f0(0, 0, 0, 0);
    while (result == 0) {
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
        func_0x6b90(0, 0, 0);
        func_0x21ca(0);
    }  // end loop
}

void func_0x3ed0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
        // data reference
        func_0x75e0(0, 0, 0, 0);
        func_0x75e0(0, 0, 0, 0);
    }  // end loop
}

void func_0x4410(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x4420(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x4430(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7bd0(0);
    func_0x7bd0(0);
    return;
    // data reference
    func_0x6a80(0, 0, 0);
    func_0x21ca(0);
    func_0x21ca(0);
}

void func_0x44f0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result == 0) {
    }  // end loop
    return;
}

void func_0x4630(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x4640(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x4680(uint64_t arg_rdi, uint64_t arg_rsi) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    func_0x6f00();
}

void func_0x4740(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x4760(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x4770(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x4800(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    // data reference
    return;
    func_0x7ec0(0, 0, 0, 0, 0, 0);
    while (result != 0) {
        // data reference
        // data reference
        // data reference
        func_0x7ec0(0, 0, 0, 0, 0, 0);
    }  // end loop
    return;
    while (result != 0) {
        func_0x78f0(0, 0, 0);
        func_0x7700();
    }  // end loop
    return;
    func_0x78f0(0, 0, 0);
}

void func_0x63b0(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7a60(0, 0, 0, 0);
    return;
}

void func_0x6400(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6420(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6440(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6480(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x64a0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x64e0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6560(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7700();
    return;
}

void func_0x6650(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6660(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x6710(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6730(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6740(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6760(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6780(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x67f0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6860(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x68d0(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6940(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x69e0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6a80(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6b10(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6b90(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6c10(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6cb0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6d50(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6df0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6e90(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6ea0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6ec0(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6ee0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6f00(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6f20(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    // data reference
    // data reference
    return;
    // data reference
    // data reference
    // data reference
    // data reference
    // data reference
    return;
    // data reference
    // data reference
    return;
    // data reference
}

void func_0x7430(void) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
}

void func_0x7450(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i <= max) {
        func_0x6f20(0, 0, 0, 0, 0, 0);
    }  // end loop
    return;
}

void func_0x7500(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i <= max) {
        func_0x6f20(0, 0, 0, 0, 0, 0);
    }  // end loop
    return;
}

void func_0x75e0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    // data reference
    // data reference
}

void func_0x76a0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x84e0(0);
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x76c0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x76e0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7700(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7720(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7750(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7780(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x84e0(0);
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x77a0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x84e0(0);
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x77c0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x84e0(0);
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x77f0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x84e0(0);
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7820(uint64_t arg_rsi) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x84e0(0);
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7880(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
        func_0x84e0(0);
    }  // end loop
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x78f0(uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x79e0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7a00(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7a20(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7a40(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7a60(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7aa0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7ae0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7b30(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7b80(0, 0, 0, 0);
}

void func_0x7b80(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    func_0x21ca(0);
}

void func_0x7bd0(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x7c40(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x7ec0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i <= max) {
    }  // end loop
    return;
    func_0x85a0(0);
    while (result != 0) {
        func_0x8520(0, 0);
    }  // end loop
}

void func_0x84e0(uint64_t arg_rsi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x8520(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x85e0();
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x85a0(uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x85d0();
    return;
}

void func_0x85d0(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x85e0(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x85f0(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x8600(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x8610(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    return;
}

void func_0x86c0(uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
}

int main(int argc, char *argv[]) {
    return 0;
}
