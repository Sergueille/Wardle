use rand::seq::IndexedRandom;


const ROOM_CODE_SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn center() -> Position {
        Position { x: 0.0, y: 0.0 }
    }

    pub fn get_json(&self) -> String {
        format!("[{},{}]", self.x.to_string(), self.y.to_string())
    }

    pub fn from_json_array(arr: &Vec<serde_json::Value>) -> Option<Position> {
        if arr.len() != 2 { return None }

        Some(Position { x: arr[0].as_f64()? as f32, y: arr[1].as_f64()? as f32 })
    }

}

/// Generate a room code at is (hopefully) pronounceable
pub fn create_random_code() -> String {
    let consonants: Vec<&str> = vec![
        "z", "r", "t", "tt", "tr", "p", "pr", "q", "s", "ss", "sr", "st", "sp", "d", "f", "g", "gs", "h", "th", "sh", "sk",
        "j", "k", "l", "lt", "lp", "ls", "ld", "pl", "ll", "m", "mp", "sm", "w", "x", "c", "ct", "cl", "ch", "cr", "ck", "v", "b", "sb", "mb", "n", "nt", "ns", "ng"
    ];

    let vowels: Vec<&str> = vec! [
        "a", "e", "i", "o", "u", "y", "ei", "ou", "oo", "ee", "ay", "ea"
    ];

    let mut res = String::with_capacity(ROOM_CODE_SIZE as usize);

    let mut vowel = true;
    while res.chars().count() < ROOM_CODE_SIZE {
        if vowel {
            res.push_str(vowels.choose(&mut rand::rng()).unwrap());
            vowel = false;
        }
        else {
            res.push_str(consonants.choose(&mut rand::rng()).unwrap());
            vowel = true;
        }
    }

    return res;
}

pub fn get_random_secret_word(lang: crate::Language) -> String {
    let n = crate::game::WORD_LENGTH as usize;
    let bytes: &[u8] = match lang {
        crate::Language::English => include_bytes!("../../words/english-few.txt"),
        crate::Language::French => include_bytes!("../../words/francais-few.txt"),
    };

    let word_count = bytes.len() / (n+1);
    let rand_id = rand::random_range(0..word_count);
    
    let mut res = String::with_capacity(n);
    for i in 0..n {
        res.push(bytes[(n+1) * rand_id + i] as char);
    }
    res.to_uppercase()
}

// Should be in O(log(nb_words))
pub fn is_valid_word(w: &str, lang: crate::Language) -> bool {
    let n = crate::game::WORD_LENGTH as usize;

    if !w.is_ascii() || w.chars().count() != n { return false; }

    let bytes: &[u8] = match lang {
        crate::Language::English => include_bytes!("../../words/english-all.txt"),
        crate::Language::French => include_bytes!("../../words/francais-all.txt"),
    };
        
    let w_lower = w.to_lowercase();
    let w_bytes = w_lower.bytes();

    enum Cmp { Less, Greater, Equal }
    fn compare(word_id: usize, w_bytes: &std::str::Bytes, all_bytes: &[u8]) -> Cmp {
        let n = crate::game::WORD_LENGTH as usize;
        
        for (i, b) in w_bytes.clone().enumerate() {
            let ref_b = all_bytes[(n+1) * word_id + i];
            if b < ref_b { return Cmp::Less; }
            else if b > ref_b { return Cmp::Greater; }
        }

        return Cmp::Equal;
    }

    let mut min = 0;
    let mut max = bytes.len() / (n+1);

    while max - min > 0 {
        let mid = (min + max) / 2;

        let cmp = compare(mid, &w_bytes, bytes);
        
        match cmp {
            Cmp::Less => {
                max = mid;
            },
            Cmp::Greater => {
                min = mid + 1;
            },
            Cmp::Equal => return true
        }
    }

    return false;
}

pub fn get_json_obj<'a>(o: &'a serde_json::Map<String, serde_json::Value>, field: &str) -> Result<&'a serde_json::Map<String, serde_json::Value>, String> {
    match o.get(field).ok_or("No type field")? {
        serde_json::Value::Object(map) => Ok(map),
        _ => Err(format!("Wrong type, expected object for field {}", field))
    }
}

pub fn get_json_str<'a>(o: &'a serde_json::Map<String, serde_json::Value>, field: &str) -> Result<&'a str, String> {
    match o.get(field).ok_or("No type field")? {
        serde_json::Value::String(s) => Ok(s),
        _ => Err(format!("Wrong type, expected string for field {}", field))
    }
}

pub fn get_json_arr<'a>(o: &'a serde_json::Map<String, serde_json::Value>, field: &str) -> Result<&'a Vec<serde_json::Value>, String> {
    match o.get(field).ok_or("No type field")? {
        serde_json::Value::Array(vec) => Ok(vec),
        _ => Err(format!("Wrong type, expected array for field {}", field))
    }
}


pub fn get_json_number<'a>(o: &'a serde_json::Map<String, serde_json::Value>, field: &str) -> Result<&'a serde_json::Number, String> {
    match o.get(field).ok_or("No type field")? {
        serde_json::Value::Number(n) => Ok(n),
        _ => Err(format!("Wrong type, expected number for field {}", field))
    }
}