use blockchain_rs::common::algorithm::aes_algorithm::AESAlgorithm;

#[test]
fn test_aes_ecb() {
    let key = b"0123456789abcdef"; // 16字节密钥
    let data = b"Hello, AES ECB!";

    // 加密
    let encrypted = AESAlgorithm::encode(key, data).unwrap();
    println!("加密字节: {:?}", encrypted);
    assert_ne!(encrypted, data);

    // 解密
    let decrypted = AESAlgorithm::decode(key, &encrypted).unwrap();
    //打印解密后的数据
    println!("解密字节: {:?}", decrypted);
    println!();
    assert_eq!(decrypted, data);
}

#[test]
fn test_invalid_key_length() {
    let invalid_key = b"short_key";
    let data = b"test";

    assert!(AESAlgorithm::encode(invalid_key, data).is_err());
    assert!(AESAlgorithm::decode(invalid_key, data).is_err());
}