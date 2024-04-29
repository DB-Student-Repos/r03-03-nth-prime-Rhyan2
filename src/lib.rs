pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut sieve = vec![true; 1_000_000]; 

    for num in 2..sieve.len() {
        if sieve[num] {
            primes.push(num as u32);
            if primes.len() == (n + 1) as usize {
                break;
            }
            let mut multiple = num * num;
            while multiple < sieve.len() {
                sieve[multiple] = false;
                multiple += num;
            }
        }
    }

    primes[n as usize]
}
