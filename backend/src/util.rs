use unicode_normalization::UnicodeNormalization;

/// Canonicalize a card name for string comparison
pub fn canonicalize_name(name: &str) -> String {
    name.trim().replace(" / ", " // ").nfc().collect()
}

/// Normalize a string for string fuzzy search
pub fn normalize_name(name: &str) -> String {
    let mut output = String::new();
    let mut last_char_space = false;
    for char in name.trim().nfkd() {
        if char.is_ascii_alphanumeric() {
            output.push(char.to_ascii_lowercase());
            last_char_space = false;
        } else if char.is_whitespace() && !last_char_space {
            output.push('-');
            last_char_space = true;
        }
    }
    output
}

/// Split a potentially split card name into parts
pub fn split_name(name: &str) -> (String, Option<String>) {
    let name = name.replace(" / ", " // ");
    let mut splits = name.split(" // ");
    let front = splits.next().unwrap_or("").to_string();
    let back = splits.next().map(|x| x.to_string());
    (front, back)
}

/// Split a potentially split card name and normalize parts for fuzzy search
pub fn split_normalize_name(name: &str) -> (String, Option<String>) {
    let (front, back) = split_name(name);
    let front = normalize_name(&front);
    let back = back.as_ref().map(|b| normalize_name(b));
    (front, back)
}
