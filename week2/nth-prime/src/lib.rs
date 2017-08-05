struct PrimeIterator {
    n: usize
}

impl PrimeIterator {
    fn new() -> PrimeIterator {
        PrimeIterator { n: 1 }
    }
}

impl Iterator for PrimeIterator{
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = (self.n + 1 .. ).find(|m: &usize| (2 .. *m).all(|l: usize| m % l != 0));
        match result {
            Some(k) => self.n = k,
            None => {}
        };
        result
    }
}

pub fn nth(n: usize) -> Result<usize, &'static str> {
    if n == 0 {
        return Err("There is no zeroth prime")
    }

    let mut iter = PrimeIterator::new();
    match iter.nth(n - 1) {
        Some(m) => Ok(m),
        None => Err("I guess there are no more primes..")
    }
}
