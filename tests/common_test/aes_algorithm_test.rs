use blockchain_rs::common::algorithm::aes_algorithm::{aes_decode, aes_encode};

#[test]
fn test_aes_ecb() {
    let key = b"0123456789abcdef"; // 16字节密钥
    let data = b"Hello, AES ECB!";

    // 加密
    let encrypted = aes_encode(key, data).unwrap();
    println!("加密字节: {:?}", encrypted);
    assert_ne!(encrypted, data);

    // 解密
    let decrypted = aes_decode(key, &encrypted).unwrap();
    //打印解密后的数据
    println!("解密字节: {:?}", decrypted);
    println!();
    assert_eq!(decrypted, data);
}

#[test]
fn test_invalid_key_length() {
    let invalid_key = b"short_key";
    let data = b"test";

    assert!(aes_encode(invalid_key, data).is_err());
    assert!(aes_decode(invalid_key, data).is_err());
}