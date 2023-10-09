#include "hash.h"
#include <openssl/sha.h>
#include <openssl/buffer.h>
#include <string.h>

void hash_function(const char *input, unsigned char *output)
{
    SHA256((const unsigned char *)input, strlen(input), output);
}

char *hash_to_hex_string(const unsigned char *hash)
{
    return OPENSSL_buf2hexstr(hash, SHA256_DIGEST_LENGTH);
}
