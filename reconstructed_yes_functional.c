#include <stdio.h>
#include <unistd.h>

int main(int argc, char *argv[]) {
    while (1) {
        printf("y\n");
        fflush(stdout);
    }
    return 0;
}
