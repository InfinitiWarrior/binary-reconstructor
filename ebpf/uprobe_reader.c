#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <signal.h>
#include <time.h>
#include <errno.h>
#include <bpf/libbpf.h>
#include <bpf/bpf.h>

static volatile int exiting = 0;

static void sig_handler(int sig) {
    exiting = 1;
}

struct uprobe_event {
    unsigned long long timestamp;
    unsigned int pid;
    unsigned int tid;
    unsigned long long func_addr;
    unsigned char is_entry;
    unsigned long long rax;
    unsigned long long rdi;
    unsigned long long rsi;
    unsigned long long rdx;
    unsigned long long rcx;
    unsigned long long r8;
    unsigned long long r9;
};

static int handle_event(void *ctx, void *data, size_t data_sz) {
    if (data_sz != sizeof(struct uprobe_event)) {
        return 1;
    }
    
    struct uprobe_event *e = (struct uprobe_event *)data;
    const char *type = e->is_entry ? "entry" : "exit";
    
    printf("{\"timestamp\":%llu,\"pid\":%u,\"type\":\"%s\",\"func\":\"0x%llx\",\"rdi\":%llu,\"rsi\":%llu,\"rax\":%llu}\n",
           e->timestamp, e->pid, type, e->func_addr, e->rdi, e->rsi, e->rax);
    fflush(stdout);
    return 0;
}

int main(int argc, char *argv[]) {
    if (argc < 4) {
        fprintf(stderr, "Usage: %s <bpf_obj> <binary> <offset> [duration_secs]\n", argv[0]);
        return 1;
    }
    
    const char *obj_file = argv[1];
    const char *binary = argv[2];
    unsigned long offset = strtoul(argv[3], NULL, 0);
    int duration = argc > 4 ? atoi(argv[4]) : 10;
    
    signal(SIGINT, sig_handler);
    signal(SIGTERM, sig_handler);
    
    printf("[\n");
    
    struct bpf_object *obj = bpf_object__open_file(obj_file, NULL);
    if (!obj) {
        perror("bpf_object__open_file");
        return 1;
    }
    
    if (bpf_object__load(obj)) {
        perror("bpf_object__load");
        return 1;
    }
    
    struct bpf_program *prog;
    int prog_count = 0;
    bpf_object__for_each_program(prog, obj) {
        const char *name = bpf_program__name(prog);
        fprintf(stderr, "Attaching %s to %s:0x%lx\n", name, binary, offset);
        bpf_program__attach_uprobe(prog, prog_count > 0, -1, binary, offset);
        prog_count++;
    }
    
    struct bpf_map *uprobes_map = bpf_object__find_map_by_name(obj, "uprobes");
    if (!uprobes_map) {
        fprintf(stderr, "uprobes map not found\n");
        return 1;
    }
    
    struct ring_buffer *ringbuf = ring_buffer__new(bpf_map__fd(uprobes_map), handle_event, NULL, NULL);
    if (!ringbuf) {
        perror("ring_buffer__new");
        return 1;
    }
    
    fprintf(stderr, "Tracing for %d seconds...\n", duration);
    time_t start = time(NULL);
    
    while (!exiting && (time(NULL) - start) < duration) {
        int ret = ring_buffer__poll(ringbuf, 100);
        if (ret < 0 && ret != -EINTR) {
            perror("ring_buffer__poll");
            break;
        }
    }
    
    printf("]\n");
    
    ring_buffer__free(ringbuf);
    bpf_object__close(obj);
    
    return 0;
}
