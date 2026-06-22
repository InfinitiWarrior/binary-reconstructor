#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

SEC("uprobe")
int trace_entry(struct pt_regs *ctx) {
    bpf_trace_printk("uprobe fired at 0x%llx", PT_REGS_IP(ctx));
    return 0;
}

SEC("uretprobe")
int trace_exit(struct pt_regs *ctx) {
    bpf_trace_printk("uretprobe fired, rax=0x%llx", PT_REGS_RC(ctx));
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
