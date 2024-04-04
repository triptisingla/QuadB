fn longestCommonPrefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new(); 
    }
    
    for (i, &ch) in strings[0].as_bytes().iter().enumerate() {
        for s in &strings[1..] {
            if i >= s.len() || s.as_bytes()[i] != ch {
                return strings[0][..i].to_string(); 
            }
        }
    }
    
    strings[0].clone()
}

fn main() {
    let strings = vec!["racing".to_string(), "racecar".to_string(), "racer".to_string()];
    println!("Longest common prefix: {}", longestCommonPrefix(&strings));
}
