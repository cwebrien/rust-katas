/*
Convert strings to pig latin. The first consonant of each word is moved to
the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
that start with a vowel have “hay” added to the end instead (“apple” becomes
“apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn is_vowel(s: &str) -> bool {
    match s {
        "A" | "a" | "E" | "e" | "I" | "i" | "O" | "o" | "U" | "u" => true,
        _ => false,
    }
}

fn pig_latin(original: &String) -> String {
    let mut result = String::new();

    for word in original.split(" ") {
        let first_char;
        match word.get(0..1) {
            Some(v) => first_char = v,
            None => panic!(),
        }

        let mut pigified = String::from(word);
        if is_vowel(&first_char) {
            pigified.push_str("hay");
        } else {
            let removed = pigified.remove(0);
            pigified.push(removed);
            pigified.push_str("ay");
        }

        result.push_str(&pigified);
        result.push(' ');
    }

    return result;
}

fn main() {
    let to_convert = "Hello my name is Cameron Brien";
    let result = pig_latin(&to_convert.to_string());

    println!("Original:  {}", to_convert);
    println!("Converted: {}", result);
}
