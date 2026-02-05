use std::collections::HashMap;

/// Parse AWS Query protocol string (x-www-form-urlencoded)
pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            // value might be url encoded, but for now simple string
            // In a real impl, decode percent encoding
            let decoded_value = percent_encoding::percent_decode_str(value).decode_utf8_lossy().to_string();
            params.insert(key.to_string(), decoded_value);
        }
    }
    params
}
