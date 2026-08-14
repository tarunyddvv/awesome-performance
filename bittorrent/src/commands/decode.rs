pub fn decode_bencoded_value(encoded_value: String) -> serde_json::Value {
    match encoded_value.chars().next() {
        // 5:hello
        Some('0'..='9') => {
            if let Some((len, value)) = encoded_value.split_once(':') {
                let len = len.parse::<usize>().expect("parsing the len of string");
                serde_json::Value::String(value[..len].to_string())
            } else {
                panic!("not a valid bencoded string value");
            }
        }
        _ => unreachable!(),
    }
}
