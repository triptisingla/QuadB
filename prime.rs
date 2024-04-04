fn isPrime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let sqrtOfn = (n as f64).sqrt() as u32;
    for i in 2..=sqrtOfn {
        if n % i == 0 {
            return false; 
        }
    }
    true 
}

fn main() {
    let num = 9;
    if isPrime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
