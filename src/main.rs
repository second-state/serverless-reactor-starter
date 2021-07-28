mod lib;

fn main() {
    let out = lib::text_received(String::from("update"), String::from(""), String::from(""));
    println!("{}", out);
}