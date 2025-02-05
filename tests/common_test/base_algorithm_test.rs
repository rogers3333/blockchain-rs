use blockchain_rs::common::algorithm::base_algorithm::{encode, encode_twice};

#[test]
fn test_encode() {
    // 待编码的数据
    let data = b"test data";
    // 调用 encode 函数进行编码
    let result = encode("SHA-256", Some(data));
    // 确保结果不为 None
    assert!(result.is_some());
}

#[test]
fn test_encode_twice() {
    // 待编码的数据
    let data = b"test data";
    // 调用 encode_twice 函数进行两次编码
    let result = encode_twice("SHA-256", Some(data));
    // 确保结果不为 None
    assert!(result.is_some());
}