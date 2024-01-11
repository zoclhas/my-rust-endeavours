struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("Zoclhas"),
        email: String::from("hi@zoclhas.com"),
        sign_in_count: 1,
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("Zoch");

    let user2 = build_user(
        String::from("LokiPoki"),
        String::from("lokiiscool@loki.dev"),
    );
    let user3 = User {
        username: String::from("John"),
        email: String::from("john@john.doe"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: false,
    }
}
