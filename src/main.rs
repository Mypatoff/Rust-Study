fn add_world(text: &String) {
    text.push_str(" world");
}

fn main() {
    let a = String::from("hello");
    add_world(&a);
    println!("{}", a);
}