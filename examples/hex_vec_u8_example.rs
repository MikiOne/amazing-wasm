use hex;

fn main() {
    // 创建一个 Vec<u8>
    let data: Vec<u8> = vec![10, 20, 30, 40, 50];

    // 将 Vec<u8> 编码为十六进制字符串
    let hex_string = hex::encode(&data);
    println!("Encoded hex: {}", hex_string);

    // 将十六进制字符串解码回 Vec<u8>
    let decoded_data = hex::decode(&hex_string).expect("Decoding failed");
    println!("Decoded data: {:?}", decoded_data);
}