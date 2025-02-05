/*
 * ECDSA 椭圆曲线数字签名算法工具类 / ECDSA Elliptic Curve Digital Signature Algorithm Utilities
 *
 * 主要功能 / Main functionalities:
 * 1. 生成SECP256K1曲线密钥对 / Generate SECP256K1 key pairs
 * 2. 支持ECDSA签名与验证 / Support ECDSA signing and verification
 * 3. 比特币风格地址生成 / Bitcoin-style address generation
 * 4. DER格式签名编解码 / DER format signature encoding/decoding
 */
use crate::common::algorithm::base_58_algorithm::Base58Algorithm;
use base64::{engine::general_purpose, Engine as _};
use k256::{
    ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyingKey},
    elliptic_curve::sec1::{EncodedPoint, ToEncodedPoint},
    Secp256k1,
};
use rand::rngs::OsRng;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// 自定义错误类型 / Custom error type
#[derive(Debug, Error)]
pub enum EcdsaError {
    #[error("Base64解码失败 / Base64 decode failed: {0}")]
    Base64Error(#[from] base64::DecodeError),

    #[error("密钥生成失败 / Key generation failed")]
    KeyGenerationError,

    #[error("签名验证失败 / Signature verification failed")]
    VerificationFailed,

    #[error("地址生成错误 / Address generation error: {0}")]
    AddressError(String),
}
pub struct ECDSAAlgorithm;
impl ECDSAAlgorithm {
    /// 生成BASE64编码的私钥 / Generate BASE64 encoded private key
    pub fn generate_private_key() -> String {
        let signing_key = SigningKey::random(&mut OsRng);
        general_purpose::STANDARD.encode(signing_key.to_bytes())
    }

    /// 从私钥生成公钥 / Generate public key from private key
    ///
    /// 参数 / Parameters:
    /// - private_key: BASE64编码的私钥 / BASE64 encoded private key
    /// - compressed: 是否生成压缩公钥 / Whether to generate compressed public key
    pub fn generate_public_key(private_key: &str, compressed: bool) -> Result<String, EcdsaError> {
        let bytes = general_purpose::STANDARD.decode(private_key)?;
        let signing_key =
            SigningKey::from_bytes(&bytes).map_err(|_| EcdsaError::KeyGenerationError)?;

        let verifying_key = signing_key.verifying_key();
        let point = verifying_key.to_encoded_point(compressed);
        Ok(general_purpose::STANDARD.encode(point.as_bytes()))
    }

    /// 生成比特币风格地址 / Generate Bitcoin-style address
    pub fn get_address(public_key: &str) -> Result<String, EcdsaError> {
        let pub_bytes = general_purpose::STANDARD.decode(public_key)?;
        let point = EncodedPoint::<Secp256k1>::from_bytes(pub_bytes)
            .map_err(|e| EcdsaError::AddressError(e.to_string()))?;

        // SHA-256哈希 / SHA-256 hash
        let mut sha256 = Sha256::new();
        sha256.update(point.as_bytes());
        let sha_result = sha256.finalize();

        // RIPEMD-160哈希 / RIPEMD-160 hash
        let mut ripemd = Ripemd160::new();
        ripemd.update(sha_result);
        let ripemd_result = ripemd.finalize();

        // Base58编码 / Base58 encoding
        Ok(Base58Algorithm::encode(&ripemd_result))
    }

    /// 生成ECDSA签名 / Generate ECDSA signature
    pub fn sign(private_key: &str, data: &[u8]) -> Result<String, EcdsaError> {
        let bytes = general_purpose::STANDARD.decode(private_key)?;
        let signing_key =
            SigningKey::from_bytes(&bytes).map_err(|_| EcdsaError::KeyGenerationError)?;

        let signature: Signature = signing_key.sign(data);
        Ok(general_purpose::STANDARD.encode(signature.to_der().as_bytes()))
    }

    /// 验证ECDSA签名 / Verify ECDSA signature
    pub fn verify(public_key: &str, data: &[u8], signature: &str) -> Result<bool, EcdsaError> {
        let pub_bytes = general_purpose::STANDARD.decode(public_key)?;
        let point = EncodedPoint::<Secp256k1>::from_bytes(pub_bytes)
            .map_err(|e| EcdsaError::AddressError(e.to_string()))?;

        let verifying_key =
            VerifyingKey::from_encoded_point(&point).map_err(|_| EcdsaError::VerificationFailed)?;

        let sig_bytes = general_purpose::STANDARD.decode(signature)?;
        let signature =
            Signature::from_der(&sig_bytes).map_err(|_| EcdsaError::VerificationFailed)?;

        Ok(verifying_key.verify(data, &signature).is_ok())
    }
}
