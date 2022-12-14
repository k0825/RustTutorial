fn main() {
    let s1 = String::from("long string is long");
    let result;
    {
        let s2 = String::from("abc");
        result = longest(s1.as_str(), s2.as_str());
    }
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
