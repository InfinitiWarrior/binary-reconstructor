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
    unsigned int pid;
    unsigned int tid;
    unsigned long long syscall_nr;
};

static int handle_event(void *ctx, void *data, size_t data_sz) {
    if (data_sz != sizeof(struct event)) {
        fprintf(stderr, "Invalid event size\n");
        return 1;
    }
    
    struct event *e = (struct event *)data;
    printf("{\"timestamp\":%llu,\"pid\":%u,\"tid\":%u,\"syscall\":%llu}\n",
           e->timestamp, e->pid, e->tid, e->syscall_nr);
    fflush(stdout);
    return 0;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <bpf_obj> [duration_secs]\n", argv[0]);
        return 1;
    }
    
    const char *obj_file = argv[1];
    int duration = argc > 2 ? atoi(argv[2]) : 10;
    
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
        if (bpf_program__attach(prog) < 0) {
            perror("bpf_program__attach");
            return 1;
        }
    }
    
    struct bpf_map *events_map = bpf_object__find_map_by_name(obj, "events");
    if (!events_map) {
        fprintf(stderr, "events map not found\n");
        return 1;
    }
    
    struct ring_buffer *ringbuf = ring_buffer__new(bpf_map__fd(events_map), handle_event, NULL, NULL);
    if (!ringbuf) {
        perror("ring_buffer__new");
        return 1;
    }
    
    time_t start = time(NULL);
    int first = 1;
    
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
