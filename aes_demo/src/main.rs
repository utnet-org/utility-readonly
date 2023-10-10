use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::seq::SliceRandom;
type AesCbc = Cbc<Aes256, Pkcs7>;
const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

fn main() {
    let plaintext = "EntSquare666";   // 明文
    let key = "01234567012345670123456701234567";   // 32位的密钥
    let iv = gen_ascii_chars(16);  // Initialization Vector（IV，初始化向量） 16位
    let enc = aes256_cbc_encrypt(key, plaintext, &iv);
    println!("encryption: {}", enc);
    let dec = aes256_cbc_decrypt(key, &enc);
    assert_eq!(plaintext, dec);
    println!("decryption: {}", dec);
}

// 生成随机IV
fn gen_ascii_chars(size: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    String::from_utf8(
        BASE_STR.as_bytes()
            .choose_multiple(&mut rng, size)
            .cloned()
            .collect()
    ).unwrap()
}

// 加密
fn aes256_cbc_encrypt(key: &str, data: &str, iv: &str)  -> String {
    let iv_bytes = iv.as_bytes();
    let cipher = AesCbc::new_from_slices(key.as_bytes(), iv_bytes).unwrap();
    let ciphertext = cipher.encrypt_vec(data.as_bytes());
    let mut buffer = bytebuffer::ByteBuffer::from_bytes(iv_bytes);
    buffer.write_bytes(&ciphertext);
    base64::encode(buffer.to_bytes())

}

// 解密
fn aes256_cbc_decrypt(key: &str, data: &str) -> String {
    let bytes = base64::decode(data).unwrap();
    let cipher = AesCbc::new_from_slices(key.as_bytes(), &bytes[0..16]).unwrap();
    String::from_utf8(cipher.decrypt_vec(&bytes[16..]).unwrap()).unwrap()
}
