use aes::{
    // Aes128,
    // Aes128Enc, Aes128Dec,
    Aes256Enc, Aes256Dec
};
use aes::cipher::{
    // BlockCipher, BlockEncrypt, BlockDecrypt,
    KeyInit,
    generic_array::GenericArray,
};
use aes::cipher::{BlockEncryptMut, BlockDecryptMut};
use aes::cipher::block_padding::Pkcs7;
use std::str;

use sha2::{Sha256, Digest};

use aes::cipher::typenum::U32;

use base64::{engine, alphabet, Engine as _};


fn key_hash(key:&str)->GenericArray<u8, U32>{
    // 初始化SHA-256哈希对象
    let mut hasher = Sha256::new();
        // 输入数据
    hasher.update(key.as_bytes());
    // 计算哈希值
    let result = hasher.finalize();
    // 将结果转换为字节数组
    let hash_bytes = result[..].to_vec();
    // 确保hash_bytes长度为32字节,因为key的长度需要是256
    let mut key: [u8; 32] = [0; 32];
    key.copy_from_slice(&hash_bytes[..32]);
    let key = GenericArray::from(key);
    return key;
}

fn aes_encrypt(plaintext: &str, key: &str) -> String {
    let plaintext = plaintext.as_bytes();
    let key = key_hash(key);
    let enc_cipher256 = Aes256Enc::new(&key);
    let pt_len = plaintext.len();
    // in-place注意这里的长度是 ((pt_len + 15)/16) * 16不然会panic
    let mut ct_buf = vec![0u8;((pt_len + 15)/16) * 16]; 
    enc_cipher256.encrypt_padded_b2b_mut::<Pkcs7>(&plaintext, &mut ct_buf).unwrap();
    let b64: String = encode_custom_base64(&ct_buf);
    return b64
}

fn main() {
    let plaintext = "hello world! this is my plaintext. 尝试插入一些中文和emoji😊";
    let key = "hello world!";
    let ciphertext = aes_encrypt(plaintext, key);

    let decrypted_text = aes_decrypt(&ciphertext, key);
    println!("{}", decrypted_text);

    assert_eq!(plaintext, decrypted_text);
}

fn aes_decrypt(ciphertext: &str, key: &str) -> String {
    let ciphertext = decode_custom_base64(ciphertext).unwrap();

    let key = key_hash(key);

    let dec_cipher256 = Aes256Dec::new(&key);

    let pt_len = ciphertext.len();

    let mut pt_buf = vec![0u8; pt_len];
    dec_cipher256.decrypt_padded_b2b_mut::<Pkcs7>(&ciphertext, &mut pt_buf).unwrap();

    // Remove padding bytes
    let padding_len = pt_buf[pt_len - 1] as usize;
    let plaintext = str::from_utf8(&pt_buf[..pt_len - padding_len]).unwrap().to_string();

    return plaintext;
}

/**
 * 这里按照默认配置设置base64，并修改编码本
 */
fn custom_config_base64_engine()-> engine::GeneralPurpose {
    // 这里按照默认配置设置base64，并修改编码本
    let alphabet =
        // alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")  // 等同于 let alphabet =base64::alphabet::STANDARD;
        alphabet::Alphabet::new("LWniakeyRS/xHzcUr8OmAs4p1K5NVGBlQJZuD7dtP0f3vIjX9qwY6T+MhobFCg2E")
        .unwrap();
    // let alphabet =base64::alphabet::STANDARD;
    let crazy_config = engine::GeneralPurposeConfig::new()
        .with_decode_allow_trailing_bits(false)
        .with_encode_padding(true)
        .with_decode_padding_mode(engine::DecodePaddingMode::RequireCanonical);
    let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);
    return crazy_engine;
}

fn encode_custom_base64(input:&Vec<u8>)-> String {
    let crazy_engine = custom_config_base64_engine();
    // let encoded: String = engine::general_purpose::STANDARD_NO_PAD.encode(input); // 如果使用默认配置可以直接使用这一行
    let encoded = crazy_engine.encode(input);
    encoded
}

fn decode_custom_base64(input:&str) -> Result<Vec<u8>, base64::DecodeError> {
    let crazy_engine = custom_config_base64_engine();
    let decoded = crazy_engine.decode(input.as_bytes());
    decoded
}
