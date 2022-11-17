fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {} is {}.", s1, len);

    let mut s = String::from("hello"); // mutをつけないとエラーになる
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", wold");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
