
pub fn get_hints(secret_word: &str, input_word: &str, sabotage_index: usize) -> Vec<crate::HintType> {
    let secret_chars: Vec<char> = secret_word.chars().collect();
    let mut res = Vec::with_capacity(secret_chars.len());

    for (i, char) in input_word.chars().enumerate() {
        if i == sabotage_index {
            res.push(crate::HintType::Red);
        }
        else if secret_chars[i] == char {
            res.push(crate::HintType::Green);
        }
        else if secret_chars.contains(&char) {
            res.push(crate::HintType::Yellow);
        }
        else {
            res.push(crate::HintType::Gray);
        }
    }

    return res;
}

pub fn get_hints_strings(hints: Vec<crate::HintType>) -> Vec<String> {
    hints.iter().map(|h| {
        String::from(match h {
            crate::HintType::Gray => "gray",
            crate::HintType::Green => "green",
            crate::HintType::Yellow => "yellow",
            crate::HintType::Red => "red",
            crate::HintType::None => "none",
        })
    }).collect()
}
