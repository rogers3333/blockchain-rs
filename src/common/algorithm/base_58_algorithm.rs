

/// 定义 Base58 编码所使用的字母表
const ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
/// 编码后的零字符
const ENCODED_ZERO: u8 = ALPHABET[0];
/// 用于快速查找字符在字母表中的索引
const INDEXES: [i8; 128] = {
    let mut indexes = [-1; 128];
    let mut i = 0;
    while i < ALPHABET.len() {
        indexes[ALPHABET[i] as usize] = i as i8;
        i += 1;
    }
    indexes
};

/// 对输入的字节数组进行 Base58 编码
pub fn base58_encode(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }
    // 统计前导零的数量
    let mut zeros = 0;
    while zeros < input.len() && input[zeros] == 0 {
        zeros += 1;
    }
    // 复制输入数据，因为后续会进行原地修改
    let mut input = input.to_vec();
    // 初始化编码结果数组，长度为输入长度的两倍作为上限
    let mut encoded = vec![0; input.len() * 2];
    let mut output_start = encoded.len();
    let mut input_start = zeros;
    while input_start < input.len() {
        // 执行除法取模操作并获取编码字符
        let remainder = divmod(&mut input, input_start, 256, 58);
        output_start -= 1;
        encoded[output_start] = ALPHABET[remainder as usize];
        if input[input_start] == 0 {
            input_start += 1;
        }
    }
    // 跳过编码结果中的前导零
    while output_start < encoded.len() && encoded[output_start] == ENCODED_ZERO {
        output_start += 1;
    }
    // 恢复前导零
    while zeros > 0 {
        output_start -= 1;
        encoded[output_start] = ENCODED_ZERO;
        zeros -= 1;
    }
    // 将编码结果转换为字符串
    String::from_utf8_lossy(&encoded[output_start..]).to_string()
}

/// 对输入的 Base58 编码字符串进行解码
pub fn base58_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.is_empty() {
        return Ok(Vec::new());
    }
    // 将输入的 Base58 字符串转换为对应的索引数组
    let mut input58 = vec![0; input.len()];
    for (i, c) in input.bytes().enumerate() {
        if c >= 128 || INDEXES[c as usize] < 0 {
            return Err("Illegal character");
        }
        input58[i] = INDEXES[c as usize] as u8;
    }
    // 统计前导零的数量
    let mut zeros = 0;
    while zeros < input58.len() && input58[zeros] == 0 {
        zeros += 1;
    }
    // 初始化解码结果数组，长度为输入长度
    let mut decoded = vec![0; input.len()];
    let mut output_start = decoded.len();
    let mut input_start = zeros;
    while input_start < input58.len() {
        // 执行除法取模操作并获取解码字节
        let remainder = divmod(&mut input58, input_start, 58, 256);
        output_start -= 1;
        decoded[output_start] = remainder;
        if input58[input_start] == 0 {
            input_start += 1;
        }
    }
    // 跳过解码结果中的多余前导零
    while output_start < decoded.len() && decoded[output_start] == 0 {
        output_start += 1;
    }
    // 截取解码结果，包含原始的前导零
    Ok(decoded[(output_start - zeros)..].to_vec())
}
/// 将 Base58 编码字符串解码为 BigInteger（在 Rust 中使用 `num-bigint` 库的 `BigUint` 替代）
pub fn decode_to_big_integer(input: &str) -> Result<num_bigint::BigUint, &'static str> {
    let decoded = base58_decode(input)?;
    Ok(num_bigint::BigUint::from_bytes_be(&decoded))
}

/// 执行除法取模操作，返回余数
fn divmod(number: &mut [u8], first_digit: usize, base: u32, divisor: u32) -> u8 {
    let mut remainder = 0;
    for i in first_digit..number.len() {
        let digit = number[i] as u32;
        let temp = remainder * base + digit;
        number[i] = (temp / divisor) as u8;
        remainder = temp % divisor;
    }
    remainder as u8
}
