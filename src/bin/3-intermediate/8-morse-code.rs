// 8. Write a program that automatically converts English text to Morse code and vice versa.

use std::collections::HashMap;

fn main() {
    let morse_table = HashMap::from([
        ("a".to_string(), ".-".to_string()),
        ("b".to_string(), "-...".to_string()),
        ("c".to_string(), "-.-.".to_string()),
        ("d".to_string(), "-..".to_string()),
        ("e".to_string(), ".".to_string()),
        ("f".to_string(), "..-.".to_string()),
        ("g".to_string(), "--.".to_string()),
        ("h".to_string(), "....".to_string()),
        ("i".to_string(), "..".to_string()),
        ("j".to_string(), ".---".to_string()),
        ("k".to_string(), "-.-".to_string()),
        ("l".to_string(), ".-..".to_string()),
        ("m".to_string(), "--".to_string()),
        ("n".to_string(), "-.".to_string()),
        ("o".to_string(), "---".to_string()),
        ("p".to_string(), ".--.".to_string()),
        ("q".to_string(), "--.-".to_string()),
        ("r".to_string(), ".-.".to_string()),
        ("s".to_string(), "...".to_string()),
        ("t".to_string(), "-".to_string()),
        ("u".to_string(), "..-".to_string()),
        ("v".to_string(), "...-".to_string()),
        ("w".to_string(), ".--".to_string()),
        ("x".to_string(), "-..-".to_string()),
        ("y".to_string(), "-.--".to_string()),
        ("z".to_string(), "--..".to_string()),
    ]);

    let mut morse_reverse= HashMap::new();
    for (letter, code) in &morse_table {
        morse_reverse.insert(code.to_owned(), letter.to_owned());
    }

    let input = String::from("Hello World");
    let input_lc = input.to_lowercase();

    let morse_string:String = convert_morse(morse_table,input);
    println!("morse translated: {:?}", morse_string);

    let english_string = convert_english(morse_reverse, morse_string);
    println!("retranslated back to english: {:?}", english_string);

    assert_eq!(input_lc, english_string);
}

fn convert_morse(map: HashMap<String,String>, input: String) -> String {
    let lc = input.to_lowercase();
    let mut translated = String::new();

    for c in lc.chars() {
        let s = c.to_string();
        if s == String::from(" ") {
            translated += &String::from("  ");
            continue;
        }
        translated += map.get(&s).unwrap();
        translated += &String::from(" ");
    }
    return translated;
}

fn convert_english(map: HashMap<String,String>, input: String) -> String {
    let mut converted = String::new();
    let words = input.split("  ");
    for word in words {
        for symbol in word.split(" ") {
            if symbol == String::from("") {
                continue;
            }
            println!("symbol {:?}", symbol);
            let english = map.get(&symbol.to_string()).unwrap();
            converted += english;
        }
        converted += &String::from(" ");
    }
    return converted.trim_end().to_string();
}