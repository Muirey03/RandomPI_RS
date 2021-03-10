use rand::Rng;
use decimal::d128;

fn is_coprime(mut n1: u32, n2: u32) -> bool {
    if n1 == n2 {
        return false;
    }

    let mut divisor = 2;
    while divisor <= (n1 as f64).sqrt() as u32 {
        while n1 % divisor == 0 {
            n1 /= divisor;
            if n2 % divisor == 0 || n2 % n1 == 0 {
                return false;
            }
        }

        if divisor == 2 {
            divisor += 1;
        } else {
            divisor += 2;
        }
    }
    true
}

/* The probability of 2 numbers being coprime is 6/pi^2. 
We can therefore generate random numbers and compute pi by seeing how many were coprime. */

fn main() {
    let mut rng = rand::thread_rng();
    let count = 100000;
    let mut coprimes = 0;

    for _x in 0..count {
        let n1: u32 = rng.gen();
        let n2: u32 = rng.gen();
        coprimes += is_coprime(n1, n2) as u32;
    }

    let pi_squared = d128!(6) * d128::from(count) / d128::from(coprimes);
    let pi = pi_squared.pow(d128!(0.5));
    println!("pi = {}", pi);
}
