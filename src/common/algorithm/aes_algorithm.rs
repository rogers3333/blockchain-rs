
use openssl::symm::{decrypt, encrypt, Cipher};
use std::error::Error;

/// 使用AES ECB模式加密数据，使用PKCS7填充。
///
/// # 参数
/// - `key`: 密钥，长度必须为16、24或32字节（分别对应AES-128、AES-192、AES-256）。
/// - `data`: 要加密的明文数据。
///
/// # 返回
/// 成功时返回加密后的密文字节向量，否则返回错误。
pub fn aes_encode(key: &[u8], data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = get_aes_cipher(key)?;
    Ok(encrypt(cipher, key, None, data)?)
}

/// 使用AES ECB模式解密数据，处理PKCS7填充。
///
/// # 参数
/// - `key`: 密钥，必须与加密时使用的密钥相同。
/// - `encrypted_data`: 要解密的密文字节向量。
///
/// # 返回
/// 成功时返回解密后的明文字节向量，否则返回错误。
pub fn aes_decode(key: &[u8], encrypted_data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    let cipher = get_aes_cipher(key)?;
    Ok(decrypt(cipher, key, None, encrypted_data)?)
}

/// 根据密钥长度选择对应的AES密码器。
fn get_aes_cipher(key: &[u8]) -> Result<Cipher, Box<dyn Error>> {
    match key.len() {
        16 => Ok(Cipher::aes_128_ecb()),
        24 => Ok(Cipher::aes_192_ecb()),
        32 => Ok(Cipher::aes_256_ecb()),
        _ => Err("无效的密钥长度，必须为16、24或32字节".into()),
    }
}
