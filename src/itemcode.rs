use base64::{Engine, engine::general_purpose};

pub fn decode_chat_code_for_item_or_skin(fullcode: &str) -> u32 {
    if !fullcode.starts_with("[&") {
        return 0;
    }

    let code = fullcode.trim_matches(|c| c == '[' || c == ']' || c == '&');
    let binary = general_purpose::STANDARD.decode(&code).unwrap();
    let octets = binary.iter().map(|b| *b as u32).collect::<Vec<u32>>();

    if octets.len() >= 2 {
        if octets[0] == 2 {
            return octets[2]
                + (octets[3] << 8)
                + (if octets.len() >= 5 {
                    octets[4] << 16
                } else {
                    0
                });
        } else if octets[0] == 11 {
            return octets[1]
                + (octets[2] << 8)
                + (if octets.len() >= 4 {
                    octets[3] << 16
                } else {
                    0
                });
        } else {
            println!("{} must be a valid chat code", fullcode);
        }
    }

    0
}

pub fn generate_chat_code_for_item(item_id: u32, quantity: u32, upgrade1_id: Option<u32>,
    upgrade2_id: Option<u32>, skin_id: Option<u32>) -> String {

    let separator = 16
        * ((skin_id.is_some() as u32)
            + (upgrade1_id.is_some() as u32)
            + (upgrade2_id.is_some() as u32));

    let ids = [
        2,
        quantity % 256,
        item_id,
        separator,
        skin_id.unwrap_or(0),
        upgrade1_id.unwrap_or(0),
        upgrade2_id.unwrap_or(0),
    ];

    let lengths = [
        1,
        1,
        3,
        1,
        if skin_id.is_some() { 4 } else { 0 },
        if upgrade1_id.is_some() { 4 } else { 0 },
        if upgrade2_id.is_some() { 4 } else { 0 },
    ];

    let mut bytes = Vec::new();
    for i in 0..ids.len() {
        for j in 0..lengths[i] {
            let b = (ids[i] >> (8 * j)) as u8 & 0xff;
            bytes.push(b);
        }
    }

    let output = general_purpose::STANDARD.encode(&bytes);
    return format!("[&{}]", output);
}

mod tests {
    use super::*;

    #[test]
    fn test_decode_chat_code_for_item_or_skin() {
        let item_code = decode_chat_code_for_item_or_skin("[&AgFeNAEA]");
        assert_eq!(item_code, 78942);
    }

    #[test]
    fn test_generate_chat_code_for_item() {
        let chat_code = generate_chat_code_for_item(78942, 44, None, None, None);

        assert_eq!(chat_code, "[&AixeNAEA]");
    }
}
