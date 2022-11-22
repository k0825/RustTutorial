struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("k0825"),
        email: String::from("testtest@test.com"),
        sign_in_count: 100,
        active: true,
    };

    let mut user2 = User {
        username: String::from("k0825"),
        email: String::from("testtest@test.com"),
        sign_in_count: 100,
        active: true,
    };
    user2.email = String::from("test2@test.com");

    println!("{}", user2.email);

    let user3 = User {
        username: String::from("k0825"),
        email: String::from("test3@test.com"),
        ..user1
    };

    println!(
        "{} {} {} {}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // username: username
        email,    // email: email
        sign_in_count: 0,
        active: true,
    }
}
