/// ErrorNum 枚举定义了一系列错误编号和对应的错误消息，用于表示不同类型的错误。
///
/// The ErrorNum enum defines a series of error codes and corresponding error messages
/// to represent different types of errors.
#[derive(Debug, PartialEq)]
pub enum ErrorNum {
    /// 参数错误
    /// Invalid parameter error
    InvalidParamError,
    /// DES3 加解密错误
    /// DES3 encryption/decryption error
    Des3EncryptError,
    /// AES 加解密错误
    /// AES encryption/decryption error
    AesEncryptError,
    /// ECDSA 加解密错误
    /// ECDSA encryption/decryption error
    EcdsaEncryptError,
    /// 签名错误
    /// Signature error
    SignError,
    /// 生成签名错误
    /// Error generating signature
    GenerateSignError,
    /// 生成 SQL 错误
    /// Error generating SQL
    GenerateSqlError,
    /// 验证签名错误
    /// Error verifying signature
    VerifySignError,
}

impl ErrorNum {
    /// 获取错误编号
    /// Get the error code
    pub fn get_ret_code(&self) -> &'static str {
        match self {
            ErrorNum::InvalidParamError => "001",
            ErrorNum::Des3EncryptError => "002",
            ErrorNum::AesEncryptError => "003",
            ErrorNum::EcdsaEncryptError => "004",
            ErrorNum::SignError => "005",
            ErrorNum::GenerateSignError => "006",
            ErrorNum::GenerateSqlError => "007",
            ErrorNum::VerifySignError => "008",
        }
    }

    /// 获取错误消息
    /// Get the error message
    pub fn get_ret_msg(&self) -> &'static str {
        match self {
            ErrorNum::InvalidParamError => "参数错误",
            ErrorNum::Des3EncryptError => "DES3加解密错误",
            ErrorNum::AesEncryptError => "AES加解密错误",
            ErrorNum::EcdsaEncryptError => "ECDSA加解密错误",
            ErrorNum::SignError => "签名错误",
            ErrorNum::GenerateSignError => "生成签名错误",
            ErrorNum::GenerateSqlError => "生成SQL错误",
            ErrorNum::VerifySignError => "验证签名错误",
        }
    }
}
