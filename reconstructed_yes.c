#include <stdio.h>
#include <stdint.h>
#include <string.h>

// Forward declarations
void func_0x215f(uint64_t arg_rdi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x21ca(uint64_t arg_rcx);
void func_0x2277(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x237c(uint64_t arg_r9);
void func_0x2470(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x28d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_r9);
void func_0x2970(uint64_t arg_rdi);
void func_0x29c0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x2df0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x3150(void);
void func_0x3160(void);
void func_0x3170(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x3230(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x34b0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x3530(void);
void func_0x3540(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x35b0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x3720(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x3870(void);
void func_0x3910(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x39a0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x5550(uint64_t arg_rdi);
void func_0x55a0(void);
void func_0x55c0(void);
void func_0x55e0(uint64_t arg_rdi);
void func_0x5620(void);
void func_0x5640(void);
void func_0x5680(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x5700(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x57f0(uint64_t arg_rcx);
void func_0x5800(uint64_t arg_rdi);
void func_0x58b0(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x58d0(uint64_t arg_rcx);
void func_0x58e0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5900(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5920(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5990(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5a00(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5a70(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5ae0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x5b80(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5c20(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5cb0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5d30(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5db0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5e50(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5ef0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x5f90(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6030(uint64_t arg_rcx);
void func_0x6040(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6060(uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6080(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x60a0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x6110(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x6620(void);
void func_0x6640(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x66f0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8);
void func_0x67d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6890(void);
void func_0x68b0(void);
void func_0x68d0(void);
void func_0x68f0(void);
void func_0x6910(void);
void func_0x6940(void);
void func_0x6970(void);
void func_0x6990(void);
void func_0x69b0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x69e0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x6a10(uint64_t arg_rsi);
void func_0x6a70(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6ae0(uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r9);
void func_0x6bd0(void);
void func_0x6bf0(void);
void func_0x6c10(void);
void func_0x6c30(void);
void func_0x6c50(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6c90(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6cd0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6d20(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6d70(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx);
void func_0x6dc0(uint64_t arg_rdi);
void func_0x6e30(uint64_t arg_rdx);
void func_0x6e80(void);
void func_0x6eb0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9);
void func_0x74d0(uint64_t arg_rdi);
void func_0x76a0(uint64_t arg_rsi);
void func_0x76e0(uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x7760(uint64_t arg_rdx);
void func_0x7790(void);
void func_0x77a0(void);
void func_0x77b0(void);
void func_0x77c0(void);
void func_0x77d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx);
void func_0x7880(uint64_t arg_rdx);

void func_0x215f(uint64_t arg_rdi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x3530();
}

void func_0x21ca(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x215f(0, 0, 0);
    return;
}

void func_0x2277(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x3530();
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
    func_0x3910(0, 0);
    // data reference
    // data reference
    // data reference
    func_0x7880(0);
    func_0x39a0(0, 0, 0, 0, 0, 0);
    func_0x3720(0, 0, 0, 0, 0);
    func_0x68b0();
    while (result != 0) {
        func_0x34b0(0, 0);
        func_0x21ca(0);
        func_0x3540(0, 0);
        func_0x3870();
        func_0x34b0(0, 0);
    }  // end loop
}

void func_0x28d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    // data reference
    return;
    // data reference
    return;
}

void func_0x2970(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    return;
}

void func_0x29c0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i > min) {
    }  // end loop
    return;
    func_0x3230(0, 0);
    return;
    func_0x3230(0, 0);
    while (result != 0) {
        // data reference
        // data reference
        // data reference
        // data reference
        // data reference
    }  // end loop
}

void func_0x2df0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
        // data reference
        func_0x67d0(0, 0, 0, 0);
        func_0x67d0(0, 0, 0, 0);
    }  // end loop
}

void func_0x3150(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x3160(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x3170(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x6dc0(0);
    func_0x6dc0(0);
    return;
    // data reference
    func_0x5c20(0, 0, 0);
    func_0x21ca(0);
    func_0x21ca(0);
}

void func_0x3230(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x34b0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (condition) {
        func_0x60a0(0, 0);
    }  // end loop
    return;
    return;
}

void func_0x3530(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x3540(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x35b0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    func_0x6640(0, 0, 0, 0);
}

void func_0x3720(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6640(0, 0, 0, 0);
}

void func_0x3870(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x74d0(0);
    func_0x6e30(0);
    func_0x6e30(0);
    return;
}

void func_0x3910(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x39a0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    // data reference
    return;
    func_0x6eb0(0, 0, 0, 0, 0, 0);
    while (result != 0) {
        // data reference
        // data reference
        // data reference
        func_0x6eb0(0, 0, 0, 0, 0, 0);
    }  // end loop
    return;
    while (result != 0) {
        func_0x6ae0(0, 0, 0);
        func_0x68f0();
    }  // end loop
    return;
    func_0x6ae0(0, 0, 0);
}

void func_0x5550(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x6c50(0, 0, 0, 0);
    return;
}

void func_0x55a0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x55c0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x55e0(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5620(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5640(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5680(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5700(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x68f0();
    return;
}

void func_0x57f0(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x5800(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x58b0(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x58d0(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x58e0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x5900(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x5920(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5990(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5a00(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5a70(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5ae0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5b80(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5c20(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5cb0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5d30(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5db0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5e50(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5ef0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x5f90(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6030(uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6040(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6060(uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6080(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x60a0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result == 0) {
    }  // end loop
    return;
}

void func_0x6110(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
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

void func_0x6620(void) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
}

void func_0x6640(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i <= max) {
        func_0x6110(0, 0, 0, 0, 0, 0);
    }  // end loop
    return;
}

void func_0x66f0(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i <= max) {
        func_0x6110(0, 0, 0, 0, 0, 0);
    }  // end loop
    return;
}

void func_0x67d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    // data reference
    // data reference
}

void func_0x6890(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x76a0(0);
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x68b0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x68d0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x68f0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6910(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6940(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6970(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x76a0(0);
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6990(void) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x76a0(0);
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x69b0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x76a0(0);
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x69e0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x76a0(0);
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6a10(uint64_t arg_rsi) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x76a0(0);
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6a70(uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
        func_0x76a0(0);
    }  // end loop
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6ae0(uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6bd0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6bf0(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6c10(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6c30(void) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6c50(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6c90(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6cd0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6d20(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x6d70(0, 0, 0, 0);
}

void func_0x6d70(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx) {
    int i = 0, result = 0, max = 0, min = 0;
    // data reference
    func_0x21ca(0);
}

void func_0x6dc0(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x6e30(uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    func_0x6e80();
    return;
}

void func_0x6e80(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x6eb0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx, uint64_t arg_rcx, uint64_t arg_r8, uint64_t arg_r9) {
    int i = 0, result = 0, max = 0, min = 0;
    while (i <= max) {
    }  // end loop
    return;
    func_0x7760(0);
    while (result != 0) {
        func_0x76e0(0, 0);
    }  // end loop
}

void func_0x74d0(uint64_t arg_rdi) {
    int i = 0, result = 0, max = 0, min = 0;
    while (result != 0) {
    }  // end loop
    return;
    func_0x3230(0, 0);
    func_0x3230(0, 0);
    func_0x3230(0, 0);
    func_0x3230(0, 0);
    while (result == 0) {
        func_0x3230(0, 0);
        func_0x3230(0, 0);
        func_0x3230(0, 0);
        func_0x3230(0, 0);
    }  // end loop
    return;
    return;
}

void func_0x76a0(uint64_t arg_rsi) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
}

void func_0x76e0(uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x77a0();
    while (result != 0) {
    }  // end loop
    return;
}

void func_0x7760(uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    func_0x7790();
    return;
}

void func_0x7790(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x77a0(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x77b0(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x77c0(void) {
    int i = 0, result = 0, max = 0, min = 0;
}

void func_0x77d0(uint64_t arg_rdi, uint64_t arg_rsi, uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
    return;
    return;
}

void func_0x7880(uint64_t arg_rdx) {
    int i = 0, result = 0, max = 0, min = 0;
}

int main(int argc, char *argv[]) {
    return 0;
}
