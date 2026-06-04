fn main() {
    let s = String::from("Hello");
    let len = length(&s);
    println!("{len}");
}

fn length(s: &String) -> usize {
    s.len()
}
