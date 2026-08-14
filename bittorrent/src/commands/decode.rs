pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, &str) {
    match encoded_value.chars().next() {
        // 5:hello
        Some('0'..='9') => {
            if let Some((len, value)) = encoded_value.split_once(':') {
                let len = len.parse::<usize>().expect("parsing the len of string");
                (
                    serde_json::Value::String(value[..len].to_string()),
                    &value[len..],
                )
            } else {
                panic!("not a valid bencoded string value");
            }
        }
        // i42e
        Some('i') => {
            if let Some((value, rest)) = encoded_value
                .strip_prefix('i')
                .and_then(|rest| rest.split_once('e'))
                .and_then(|(digit, rest)| Some((digit.parse::<i64>().ok(), rest)))
            {
                (value.into(), rest)
            } else {
                panic!("not a valid bencoded string value");
            }
        }
        // l5:helloi52ee
        Some('l') => {
            let mut values = Vec::new();
            let mut rest = encoded_value.strip_prefix('l').expect("strip l prefix");

            while !rest.starts_with('e') {
                let (value, remainder) = decode_bencoded_value(rest);

                values.push(value);
                rest = remainder;
            }

            (values.into(), &rest[1..])
        }
        Some('d') => {
            let mut values = serde_json::Map::new();
            let mut rest = encoded_value.strip_prefix('d').expect("strip d prefix");

            while !rest.starts_with('e') {
                let (k, remainder) = decode_bencoded_value(rest);
                let (v, left) = decode_bencoded_value(remainder);

                let k = match k {
                    serde_json::Value::String(k) => k,
                    k => panic!("dict keys must be string not '{k:?}'"),
                };
                values.insert(k, v);
                rest = left;
            }

            (values.into(), &rest[1..])
        }
        _ => unreachable!(),
    }
}
