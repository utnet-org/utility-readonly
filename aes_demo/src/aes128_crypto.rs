
use aes::{Aes128}; // Aes128Enc, Aes128Dec
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use rand::RngCore;

const DIGIT: usize = 16;
pub fn test_aes128() {
    let plaintext = "EntySquare666";
    // 生成随机的16字节密钥
    let mut key_data = [0u8; DIGIT];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key_data);
    let key = GenericArray::from(key_data);

    let enc = aes128_cbc_encrypt(plaintext, key);

    let dec = aes128_cbc_decrypt(enc, key);

    println!("明文: {:?}", std::str::from_utf8(&dec[..plaintext.len()]));

}

fn aes128_cbc_encrypt(text: &str, key: GenericArray<u8, typenum::U16>) -> GenericArray<u8, typenum::U16> {
    // 如果数据长度不是16字节的整数倍，进行填充
    let mut padded_data = Vec::from(text);
    let block_size = DIGIT;
    let padding_len = block_size - (text.len() % block_size);
    let padding_byte = padding_len as u8;
    padded_data.extend(vec![padding_byte; padding_len]);

    println!("text: {:?}", padded_data);

    let mut block = GenericArray::from([48u8; DIGIT]);
    // Initialize cipher
    let cipher = Aes128::new(&key);
    // Encrypt block in-place
    println!("明文: {:?}", text);
    block.copy_from_slice(&padded_data[..block_size]);
    cipher.encrypt_block(&mut block);
    println!("密文: {:?}", base64::encode(block));
    block
}

fn aes128_cbc_decrypt(enc: GenericArray<u8, typenum::U16>, key: GenericArray<u8, typenum::U16>)  -> GenericArray<u8, typenum::U16> {
    let cipher = Aes128::new(&key);
    let mut block = enc;
    cipher.decrypt_block(&mut block);
    block
}