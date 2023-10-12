
use std::env;

mod aes128_crypto;
// mod aes256_crypto;

fn main() {
    let args: Vec<String> = env::args().collect();
    let txt =  args[1].as_str();

    aes128_crypto::test_aes128(txt);
    // aes256_crypto::test_aes256_cbc();

}