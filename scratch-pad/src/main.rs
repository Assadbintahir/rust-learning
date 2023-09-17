struct User {
    age: i64,
    isActive: bool,
    name: String,
    email: String,
}

fn main() {
    // Ex-1: string mutability
    let mut message = String::from("Hello, World!");
    let word = first_word(&message);
    println!("{}", word);

    // Ex-2: struct mutability
    let user1 = User {
        age: 28,
        isActive: true,
        name: String::from("Asad Ullah"),
        email: String::from("asad.u@outlook.com"),
    };

    let user2 = User {
        name: String::from("Asad"),
        ..user1
    };

    println!("{}", user2.email);
}

fn first_word(message: &String) -> &str {
    let bytes = message.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &message[0..i];
        }
    }

    return &message;
}
