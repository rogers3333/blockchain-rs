use blockchain_rs::common::algorithm::base_58_algorithm::{decode_to_big_integer, base58_encode, base58_decode};
#[test]
fn test_encode() {
    let input = b"hello";
    let encoded = base58_encode(input);
    println!("{}", encoded);
    assert!(!encoded.is_empty());
}

#[test]
fn test_decode() {
    let input = b"hello";
    let encoded = base58_encode(input);
    let decoded = base58_decode(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_decode_to_big_integer() {
    let input = b"hello";
    let encoded = base58_encode(input);
    let _big_integer = decode_to_big_integer(&encoded).unwrap();
}
