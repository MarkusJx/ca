pub trait U8VecToString {
    fn to_string(&self) -> String;

    fn to_hex_string(&self, separator: &str) -> String;
}

impl U8VecToString for Vec<u8> {
    fn to_string(&self) -> String {
        self.iter().map(|b| *b as char).collect()
    }

    fn to_hex_string(&self, separator: &str) -> String {
        self.iter()
            .map(|b| format!("{:02x}", b).to_string())
            .collect::<Vec<String>>()
            .join(separator)
    }
}
