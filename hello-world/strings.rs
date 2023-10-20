fn main() {
    let text = String::from("Gabriel");
    thief(text);
    println!("{}", text);
}

fn thief(text: String) {
    println!("{}", text);
}