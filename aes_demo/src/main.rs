
mod aes256_crypto;
// mod aes128_crypto;

fn main() {
    aes256_crypto::test_aes256_cbc();
    // aes128_crypto::test_aes128_cbc();
}