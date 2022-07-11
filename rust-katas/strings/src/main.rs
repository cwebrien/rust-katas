/*
Convert strings to pig latin. The first consonant of each word is moved to
the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
that start with a vowel have “hay” added to the end instead (“apple” becomes
“apple-hay”). Keep in mind the details about UTF-8 encoding!
*/

fn pig_latin(original: &String) -> String {
    return String::from("to implement");
}

fn main() {
    let to_convert = "Hello, my name is Cameron Brien";
    let result = pig_latin(&to_convert.to_string());

    println!("Original:  {}", to_convert);
    println!("Converted: {}", result);
}
