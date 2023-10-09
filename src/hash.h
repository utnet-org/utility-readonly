#ifndef HASH_FUNCTIONS_H //? 防止重复包含
#define HASH_FUNCTIONS_H //? 防止重复包含

#include <stddef.h> //? 为了使用 size_t

// 哈希类型枚举
enum HashType
{
    HashSha256,
    HashMd5,
    HashRipemd160,
    HashWhirlpool,
    // ... 如果您添加了其他的哈希类型
};

// 函数声明
void hash_function(const char *input, unsigned char *output, enum HashType type);
char *hash_to_hex_string(const unsigned char *hash);

#endif // HASH_FUNCTIONS_H