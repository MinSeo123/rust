struct User {
    username: String,
    age : u64
}



fn main() {
    let mut minseo : User = add_user("minpal", 25);
    println!("{}" , minseo.username);
    minseo.username = String::from("mingu");
    println!("{}" , minseo.username);
    let user2 = User {
        username : String::from("minchil"),
        ..minseo
    };
    println!("{}", user2.age);
}

fn add_user (username : &str , age : u64) -> User {
    User {
        username : String::from(username),
        age : age
    }
}