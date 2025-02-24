fn find_first_word(str: String) -> String {
    println!("{}", str.split(" ").next().unwrap());

    String::from("Hello, world!")
}

fn main() {
    let mut str = String::from("");

    std::io::stdin().read_line(&mut str).unwrap();

    find_first_word(str);

    println!("Hello, world!");
}
