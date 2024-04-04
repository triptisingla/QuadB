fn shortestWord(s: &str) -> Option<&str> {
    let mut shortest: Option<&str> = None;

    for word in s.split_whitespace() {
        shortest = match shortest {
            Some(shortest_word) => {
                if word.len() < shortest_word.len() {
                    Some(word)
                } else {
                    Some(shortest_word)
                }
            }
            None => Some(word),
        };
    }

    shortest
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    match shortestWord(sentence) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the string"),
    }
}
