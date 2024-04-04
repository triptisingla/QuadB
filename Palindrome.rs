fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

fn main() {
    let input = "madam";
    let output = is_palindrome(input);
    if output==true {println!("'{}' is a palindrome.", input)} else {println!("'{}' is not a palindrome.", input)};
}