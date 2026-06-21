#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

struct event {
    u64 timestamp;
    u32 pid;
    u32 tid;
    u64 syscall_nr;
};

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} events SEC(".maps");

SEC("tp/raw_syscalls/sys_enter")
int trace_enter(struct trace_event_raw_sys_enter *ctx) {
    struct event *e = bpf_ringbuf_reserve(&events, sizeof(*e), 0);
    if (!e) return 0;
    
    e->timestamp = bpf_ktime_get_ns();
    e->pid = bpf_get_current_pid_tgid() >> 32;
    e->tid = (u32)bpf_get_current_pid_tgid();
    e->syscall_nr = ctx->id;
    
    bpf_ringbuf_submit(e, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
