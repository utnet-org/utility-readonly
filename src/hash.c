#include "hash.h"
#include <openssl/sha.h>
#include <openssl/md5.h>
#include <openssl/ripemd.h>
#include <openssl/buffer.h>
#include <openssl/whrlpool.h>
#include <string.h>

void hash_function(const char *input, unsigned char *output, enum HashType type)
{
    switch (type)
    {
    case HashSha256:
        SHA256((const unsigned char *)input, strlen(input), output);
        break;
    case HashMd5:
        MD5((const unsigned char *)input, strlen(input), output);
        break;
    case HashRipemd160:
        RIPEMD160((const unsigned char *)input, strlen(input), output);
        break;
    case HashWhirlpool:
        WHIRLPOOL((const unsigned char *)input, strlen(input), output);
        break;
    // ... 添加其他哈希算法处理
    default:
        // Maybe handle an unrecognized type or just do nothing
        break;
    }
}

char *hash_to_hex_string(const unsigned char *hash)
{
    return OPENSSL_buf2hexstr(hash, SHA256_DIGEST_LENGTH);
}
