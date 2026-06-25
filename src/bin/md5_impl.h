#ifndef MD5_H
#define MD5_H
#include <stdint.h>
#include <string.h>

typedef struct {
    uint32_t a, b, c, d;
    uint64_t count;
    uint8_t buffer[64];
} MD5_CTX;

void md5_init(MD5_CTX *ctx);
void md5_update(MD5_CTX *ctx, const uint8_t *data, uint32_t len);
void md5_final(uint8_t *digest, MD5_CTX *ctx);

#endif
