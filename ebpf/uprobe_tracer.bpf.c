#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct event {
    u64 timestamp;
    u64 func_addr;
    u8 is_entry;
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} uprobes SEC(".maps");

SEC("uprobe")
int trace_entry(struct pt_regs *ctx) {
    struct event *e = bpf_ringbuf_reserve(&uprobes, sizeof(*e), 0);
    if (!e) return 0;
    
    e->timestamp = bpf_ktime_get_ns();
    e->func_addr = ctx->ip;
    e->is_entry = 1;
    
    bpf_ringbuf_submit(e, 0);
    return 0;
}

SEC("uretprobe")
int trace_exit(struct pt_regs *ctx) {
    struct event *e = bpf_ringbuf_reserve(&uprobes, sizeof(*e), 0);
    if (!e) return 0;
    
    e->timestamp = bpf_ktime_get_ns();
    e->func_addr = ctx->ip;
    e->is_entry = 0;
    
    bpf_ringbuf_submit(e, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
