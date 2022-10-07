use rand::Rng;
use std::process;

#[derive(Debug)]
pub struct Game {
    level: u64,
    total_retry: u64,
    retry_count: u64,
    screen_messages: Vec<String>,
    bounds: (u64, u64),
    secret_number: u64,
    known_primes: Vec<u64>,
}

impl Game {
    // new method to instantiate the Game
    pub fn new() -> Self {
        Self {
            level: 1,
            total_retry: get_total_retry(1, 0),
            retry_count: 0,
            screen_messages: vec![],
            bounds: get_bounds(1),
            secret_number: 0,
            known_primes: vec![],
        }
    }

    // run the game
    pub fn run(&mut self) {
        loop {
            // clear screen
            Self::clear_screen();
            // initialize level
            self.initialize_level();
        }
    }

    // clear screen messages
    fn clear_screen() {
        // ::new("clear").status().unwrap();
        process::Command::new("clear")
            .status()
            .expect("Error: Not able to clear screen 😟");
    }

    // increment level
    fn increment_level(&mut self) {
        self.level += 1;
    }

    // initialize level
    fn initialize_level(&mut self) {
        self.total_retry = get_total_retry(self.level, self.total_retry - self.retry_count);
        self.retry_count = 0;
        self.screen_messages = vec![];
        self.bounds = get_bounds(self.level);
        self.secret_number = generate_random_number(self.bounds);
        self.generate_primes();
    }

    // generate all prime number upto an upper bound
    // this will help us quickly check if a number is prime or not
    // and factorize a given number.
    // the upper bound will be taken as the upper bound of the next level
    fn generate_primes(&mut self) {
        // start checking for prime numbers from 2
        let mut curr_no = 2;
        // get the last element of the known_primes
        if let Some(p) = self.known_primes.pop() {
            // if the last element exists then
            // set curr_no = last_element + 2
            // because that is the potential prime number
            // put back the last_element which we just have popped
            curr_no = if p == 2 { 3 } else { p + 2 };
            self.known_primes.push(p);
        }
        // get the bounds of the next level
        // we will be checking for prime numbers upto
        // the upper bound of the next level
        let (_, ub) = get_bounds(self.level + 1);

        // loop until upper bound and check for primes
        'outer: loop {
            // if upper bound is reached then break
            // since upper bound is always multiple of the level value
            // it cannot be a prime number
            if curr_no >= ub {
                break;
            }
            // check if curr_no is divisible by any number in known_primes list
            // if divisibles then curr_no is not a prime
            // otherwise it is a prime
            for &n in &self.known_primes {
                // if divisible then increment curr_no by 2
                // and continue the outer loop
                if curr_no % n == 0 {
                    curr_no = if curr_no == 2 { 3 } else { curr_no + 2 };
                    continue 'outer;
                }
                // no need check for all elements
                // checking upto half of curr_no is enough
                // to determine if it is a prime number
                if n > curr_no / 2 {
                    break;
                }
            }
            // push curr_no to the known_primes list
            // and increment curr_no by 2
            self.known_primes.push(curr_no);
            curr_no = if curr_no == 2 { 3 } else { curr_no + 2 };
        }
    }
}

// get total_retry depending on level
fn get_total_retry(level: u64, leftover: u64) -> u64 {
    2 * level + 3 + leftover
}

// get lower bound and upper bound depending on level
fn get_bounds(level: u64) -> (u64, u64) {
    (level * 1000, (level + 1) * 1000)
}

// generate a random number between two bounds
fn generate_random_number(bounds: (u64, u64)) -> u64 {
    rand::thread_rng().gen_range(bounds.0..=bounds.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    // test get_total_retry
    #[test]
    fn test_get_total_retry() {
        let level = 2;
        let leftover = 3;
        assert_eq!(get_total_retry(level, leftover), 2 * level + 3 + leftover);
    }

    // test get_bounds
    #[test]
    fn test_get_bounds() {
        let level = 2;
        assert_eq!(get_bounds(2), (level * 1000, (level + 1) * 1000));
    }

    // test generate_random_number
    #[test]
    fn test_generate_random_number() {
        let lb = 100;
        let ub = 5000;
        let num = generate_random_number((lb, ub));
        assert!(num >= lb && num <= ub);
    }

    // test new methods
    #[test]
    fn test_method_new() {
        let game = Game::new();
        assert_eq!(game.level, 1);
        assert_eq!(game.total_retry, get_total_retry(1, 0));
        assert_eq!(game.retry_count, 0);
        assert_eq!(game.screen_messages, Vec::<String>::new());
        assert_eq!(game.bounds, get_bounds(1));
        assert_eq!(game.secret_number, 0);
        assert_eq!(game.known_primes, vec![]);
    }
}
