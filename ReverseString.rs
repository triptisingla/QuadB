fn reverseString(s: &str) -> String {
    let mut reversedString = String::new();
    for c in s.chars().rev() {
        reversedString.push(c);
    }
    reversedString
}

fn main() {
    let s = "Hello, world!";
    let reversed = reverseString(s);
    println!("Original string: {}", s);
    println!("Reversed string: {}", reversed);
}
