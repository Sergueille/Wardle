
pub fn get_hints(secret_word: &str, input_word: &str, sabotage_index: usize) -> Vec<crate::HintType> {
    let secret_chars: Vec<char> = secret_word.chars().collect();
    let mut res = Vec::with_capacity(secret_chars.len());

    let mut letter_counts = Vec::with_capacity(26);
    for i in 0..26 {
        let letter = ('A' as u8 + i) as char;
        letter_counts.push(secret_chars.iter().filter(|c| **c == letter).count());
    }

    let mut shown_counts = vec![0; 26];

    for (i, char) in input_word.chars().enumerate() {
        let id = (char as u8 - 'A' as u8) as usize;

        if i == sabotage_index {
            res.push(crate::HintType::Red);
        }
        else if secret_chars[i] == char {
            res.push(crate::HintType::Green);
        }
        else if secret_chars.contains(&char) && shown_counts[id] < letter_counts[id] {
            res.push(crate::HintType::Yellow);
        }
        else {
            res.push(crate::HintType::Gray);
        }

        if secret_chars.contains(&char) {
            shown_counts[id] += 1;
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
