/// 基础算法工具模块，提供加密相关功能
use sha2::{Digest, Sha256};

/// 对输入的数据使用指定的哈希算法进行编码
///
/// # 参数
/// - `algorithm`: 哈希算法的名称，目前仅支持 "SHA-256"
/// - `data`: 待编码的数据
///
/// # 返回值
/// 如果输入数据为 `None`，则返回 `None`；否则返回编码后的字节数组
pub fn encode(algorithm: &str, data: Option<&[u8]>) -> Option<Vec<u8>> {
    // 如果输入数据为 None，直接返回 None
    if data.is_none() {
        return None;
    }
    let data = data.unwrap();
    // 根据算法名称选择对应的哈希算法
    match algorithm {
        "SHA-256" => {
            // 创建一个新的 Sha256 实例
            let mut hasher = Sha256::new();
            // 更新哈希器的输入数据
            hasher.update(data);
            // 完成哈希计算并获取结果
            let result = hasher.finalize();
            // 将结果转换为 Vec<u8> 类型并返回
            Some(result.to_vec())
        }
        _ => panic!("Unsupported algorithm: {}", algorithm),
    }
}

/// 对输入的数据使用指定的哈希算法进行两次编码
///
/// # 参数
/// - `algorithm`: 哈希算法的名称，目前仅支持 "SHA-256"
/// - `data`: 待编码的数据
///
/// # 返回值
/// 如果输入数据为 `None`，则返回 `None`；否则返回两次编码后的字节数组
pub fn encode_twice(algorithm: &str, data: Option<&[u8]>) -> Option<Vec<u8>> {
    // 如果输入数据为 None，直接返回 None
    if data.is_none() {
        return None;
    }
    let data = data.unwrap();
    // 根据算法名称选择对应的哈希算法
    match algorithm {
        "SHA-256" => {
            // 第一次编码
            let first_encoded = {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize()
            };
            // 第二次编码
            let second_encoded = {
                let mut hasher = Sha256::new();
                hasher.update(&first_encoded);
                hasher.finalize()
            };
            // 将结果转换为 Vec<u8> 类型并返回
            Some(second_encoded.to_vec())
        }
        _ => panic!("Unsupported algorithm: {}", algorithm),
    }
}
