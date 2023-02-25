// 8. Write a program that automatically converts English text to Morse code and vice versa.

use std::collections::HashMap;

fn main() {
    let morse_table = HashMap::from([
        ("A", ".-"),
        ("B", "-..."),
        ("C", "-.-."),
        ("D", "-.."),
        ("E", "."),
        ("F", "..-."),
        ("G", "--."),
        ("H", "...."),
        ("I", ".."),
        ("J", ".---"),
        ("K", "-.-"),
        ("L", ".-.."),
        ("M", "--"),
        ("N", "-."),
        ("O", "---"),
        ("P", ".--."),
        ("Q", "--.-"),
        ("R", ".-."),
        ("S", "..."),
        ("T", "-"),
        ("U", "..-"),
        ("V", "...-"),
        ("W", ".--"),
        ("X", "-..-"),
        ("Y", "-.--"),
        ("Z", "--.."),
    ])

}