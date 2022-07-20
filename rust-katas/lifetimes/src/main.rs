fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "abc";
    let longer;
    let y = String::from("def");
    {
        longer = longest(&x, y.as_str());
    }
    println!("Longer: {}", longer);
}
