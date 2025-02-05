use blockchain_rs::common::algorithm::base_58_algorithm::Base58Algorithm;
#[test]
fn test_encode() {
    let input = b"hello";
    let encoded =  Base58Algorithm::encode(input);
    println!("{}", encoded);
    assert!(!encoded.is_empty());
}

#[test]
fn test_decode() {
    let input = b"hello";
    let encoded =  Base58Algorithm::encode(input);
    let decoded =  Base58Algorithm::decode(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn test_decode_to_big_integer() {
    let input = b"hello";
    let encoded =  Base58Algorithm::encode(input);
    let _big_integer = Base58Algorithm::decode_to_big_integer(&encoded).unwrap();
}
