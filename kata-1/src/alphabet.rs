pub fn alphabet_position(text: &str) -> String {
    let mut string = String::new();
    for character in text.chars() {
        if string.len() != 0 {
            string.push(' ');
        }
        let integer_char = 1 + (character as u32) - ('a' as u32);
        
        if integer_char >= 1 || integer_char <= 26 {
            let result = integer_char.to_string();
            string.push_str(&result);
        };
    };
    
    return string;
}

