use aes::{Aes128, Aes128Enc, Aes128Dec};
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
use aes::cipher::{BlockEncryptMut, BlockDecryptMut};
use aes::cipher::block_padding::Pkcs7;

pub fn test_aes128_cbc() {
    let text = "EntySquare666";
    // 如果数据长度不是16字节的整数倍，进行填充
    let mut padded_data = Vec::from(text);
    let block_size = 16;
    let padding_len = block_size - (text.len() % block_size);
    let padding_byte = padding_len as u8;
    padded_data.extend(vec![padding_byte; padding_len]);

    println!("text: {:?}", padded_data);

    let key = GenericArray::from([48u8; 16]);
    let mut block = GenericArray::from([48u8; 16]);
    // Initialize cipher
    let cipher = Aes128::new(&key);
    // Encrypt block in-place
    println!("明文: {:?}", text);
    block.copy_from_slice(&padded_data[..block_size]);
    cipher.encrypt_block(&mut block);
    println!("密文: {:?}", base64::encode(block));

    // block.copy_from_slice(&padded_data[..block_size]);
    cipher.decrypt_block(&mut block);
    println!("明文: {:?}", std::str::from_utf8(&block[..text.len()]));

}