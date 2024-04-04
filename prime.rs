fn isPrime(n: u32) -> bool {
    if n <= 1 {
        return false; // 0 and 1 are not prime numbers
    }
    let sqrtOfn = (n as f64).sqrt() as u32;
    for i in 2..=sqrtOfn {
        if n % i == 0 {
            return false; // n is divisible by i, hence not prime
        }
    }
    true // n is not divisible by any number within the range, hence prime
}

fn main() {
    let num = 9;
    if isPrime(num) {
        println!("{} is prime.", num);
    } else {
        println!("{} is not prime.", num);
    }
}
