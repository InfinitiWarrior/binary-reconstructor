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

struct event {
    unsigned long long timestamp;
    unsigned long long func_addr;
    unsigned char is_entry;
};

static int handle_event(void *ctx, void *data, size_t data_sz) {
    if (data_sz != sizeof(struct event)) {
        return 1;
    }
    
    struct event *e = (struct event *)data;
    const char *type = e->is_entry ? "entry" : "exit";
    
    printf("{\"timestamp\":%llu,\"func\":\"0x%llx\",\"type\":\"%s\"}\n",
           e->timestamp, e->func_addr, type);
    fflush(stdout);
    return 0;
}

int main(int argc, char *argv[]) {
    if (argc < 3) {
        fprintf(stderr, "Usage: %s <bpf_obj> <binary> [duration_secs]\n", argv[0]);
        return 1;
    }
    
    const char *obj_file = argv[1];
    const char *binary = argv[2];
    const char *symbol = "process_data";
    int duration = argc > 3 ? atoi(argv[3]) : 10;
    
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
    bpf_object__for_each_program(prog, obj) {
        const char *name = bpf_program__name(prog);
        bool is_return = (strstr(name, "exit") != NULL);
        fprintf(stderr, "Attaching %s to %s:%s (%s)\n", name, binary, symbol, is_return ? "return" : "entry");
        bpf_program__attach_uprobe(prog, is_return, -1, binary, 0);
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
