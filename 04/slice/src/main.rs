fn main() {
    let mut s1 = String::from("hello world");
    let word = first_word(&s1);

    s1.clear();

    let s2 = String::from("hello world");
    let hello = &s2[0..5];
    let world = &s2[6..11];

    println!("{} {}", hello, world);

    let slice = &s2[6..];
    println!("{}", slice);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
