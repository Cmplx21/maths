// This module contains simple mathematical functions.

/// Checks if a number is even.
///
/// # Example
///
/// ```
/// assert!(!is_even(1));
/// assert!(is_even(2));
/// ```
///
/// # Return
///
/// A boolean indicating if the number is even.
///
pub fn is_even(n: i64) -> bool {
    n % 2 == 0
}

/// Checks if a number is odd.
///
/// # Example
///
/// ```
/// assert!(is_odd(1));
/// assert!(!is_odd(2));
/// ```
///
/// # Return
///
/// A boolean indicating if the number is odd.
///
pub fn is_odd(n: i64) -> bool {
    !is_even(n)
}

/// Checks if a number is prime.
///
/// # Example
///
/// ```
/// assert_eq!(is_prime(1), (false, vec![]));
/// assert_eq!(is_prime(2), (true, vec![]));
/// assert_eq!(is_prime(213), (false, vec![3, 71]));
/// ```
///
/// # Return
///
/// A tuple containing a boolean indicating if the number is prime and a vector of divisors
///
pub fn is_prime(n: u64) -> (bool, Vec<u64>) {
    let mut divisors: Vec<u64> = Vec::new();
    let mut num_is_prime: bool = true;

    // Numbers less than or equal to 1 are not prime.
    if n > 1 {
        // Iterate from 2 to n - 1.
        for i in 2..n {
            // If n is divisible by i, then n is not prime.
            if n % i == 0 {
                divisors.push(i);
                num_is_prime = false;
            }
        }
    } else {
        // If n is less than or equal to 1, then n is not prime.
        num_is_prime = false;
    }

    // If no divisors were found, then n is prime.
    (num_is_prime, divisors)
}

// This module contains test functions.
#[cfg(test)]
mod tests {
    // Import functions from the parent module.
    use super::*;

    // Test cases for the is_even function.
    #[test]
    fn test_is_even() {
        assert!(!is_even(1));
        assert!(is_even(2));
    }

    // Test cases for the is_odd function.
    #[test]
    fn test_is_odd() {
        assert!(is_odd(1));
        assert!(!is_odd(2));
    }

    // Test cases for the is_prime function.
    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), (false, vec![]));
        assert_eq!(is_prime(2), (true, vec![]));
        assert_eq!(is_prime(3), (true, vec![]));
        assert_eq!(is_prime(4), (false, vec![2]));
        assert_eq!(is_prime(29), (true, vec![]));
        assert_eq!(is_prime(100), (false, vec![2, 4, 5, 10, 20, 25, 50]));
        assert_eq!(is_prime(213), (false, vec![3, 71]));
    }
}
