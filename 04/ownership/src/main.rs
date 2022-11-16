fn main() {
    let s = String::from("hello");
    takes_ownership(s); // sがムーブされる。以降sは使えない
    let x = 5;
    makes_copy(x); // xはムーブされるが、以降も使える

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2は関数にムーブされる。戻り値はs3にムーブされる

    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("The length of '{}' is {}", s5, len);
} // s1, s3はスコープを抜け、ドロップされる。s2もスコープは抜けているが、すでにムーブされているので何も起こらない

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_stringがdropされ、メモリ解放される

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
