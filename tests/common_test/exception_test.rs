use blockchain_rs::common::exception::error_enum::ErrorNum;

#[test]
fn test_error_num() {
    // 测试 InvalidParamError
    let error = ErrorNum::InvalidParamError;
    assert_eq!(error.get_ret_code(), "001");
    assert_eq!(error.get_ret_msg(), "参数错误");

    // 测试 Des3EncryptError
    let error = ErrorNum::Des3EncryptError;
    assert_eq!(error.get_ret_code(), "002");
    assert_eq!(error.get_ret_msg(), "DES3加解密错误");

    // 测试 AesEncryptError
    let error = ErrorNum::AesEncryptError;
    assert_eq!(error.get_ret_code(), "003");
    assert_eq!(error.get_ret_msg(), "AES加解密错误");

    // 测试 EcdsaEncryptError
    let error = ErrorNum::EcdsaEncryptError;
    assert_eq!(error.get_ret_code(), "004");
    assert_eq!(error.get_ret_msg(), "ECDSA加解密错误");

    // 测试 SignError
    let error = ErrorNum::SignError;
    assert_eq!(error.get_ret_code(), "005");
    assert_eq!(error.get_ret_msg(), "签名错误");

    // 测试 GenerateSignError
    let error = ErrorNum::GenerateSignError;
    assert_eq!(error.get_ret_code(), "006");
    assert_eq!(error.get_ret_msg(), "生成签名错误");

    // 测试 GenerateSqlError
    let error = ErrorNum::GenerateSqlError;
    assert_eq!(error.get_ret_code(), "007");
    assert_eq!(error.get_ret_msg(), "生成SQL错误");

    // 测试 VerifySignError
    let error = ErrorNum::VerifySignError;
    assert_eq!(error.get_ret_code(), "008");
    assert_eq!(error.get_ret_msg(), "验证签名错误");
}