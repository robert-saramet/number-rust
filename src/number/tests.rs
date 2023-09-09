#[cfg(test)]
mod i32_tests {
    use super::super::*;

    #[cfg(feature = "random_tests")]
    use rand::prelude::*;

    #[test]
    fn new_for_37_eq() {
        assert_eq!(
            Number::new(37),
            Number {
                value: 37,
                positive: true,
                sign: Sign::Positive,
                parity: Parity::Odd
            }
        )
    }
    #[test]
    fn new_for_44_ne() {
        assert_ne!(
            Number::new(44),
            Number {
                value: 44,
                positive: false,
                sign: Sign::Positive,
                parity: Parity::Even
            }
        )
    }
    #[test]
    fn positive_for_69_eq() {
        assert!(Number::new(69).positive)
    }
    #[test]
    fn negative_for_minus24_eq() {
        assert!(!Number::new(-24).positive)
    }
    #[test]
    fn new_for_min_i32_eq() {
        assert_eq!(
            Number::new(i32::MIN),
            Number {
                value: -2_147_483_648,
                positive: false,
                sign: Sign::Negative,
                parity: Parity::Even
            }
        )
    }
    #[test]
    fn new_for_max_i32_eq() {
        assert_eq!(
            Number::new(i32::MAX),
            Number {
                value: 2_147_483_647,
                positive: true,
                sign: Sign::Positive,
                parity: Parity::Odd
            }
        )
    }
    #[test]
    fn change_min_i32_to_max() {
        let mut num = Number::new(i32::MIN);
        num.set_value(i32::MAX);
        assert_eq!(
            num,
            Number {
                value: 2_147_483_647,
                positive: true,
                sign: Sign::Positive,
                parity: Parity::Odd
            }
        )
    }
    #[test]
    fn change_37_to_minus920_eq() {
        let mut num = Number::new(37);
        num.set_value(-920);
        assert_eq!(
            num,
            Number {
                value: -920,
                positive: false,
                sign: Sign::Negative,
                parity: Parity::Even
            }
        )
    }
    #[test]
    fn change_37_to_minus920_ne() {
        let mut num = Number::new(37);
        let old = num.clone();
        num.set_value(-920);
        assert_ne!(num, old)
    }
    #[test]
    fn turn_i32_to_number_minus397() {
        assert_eq!(
            Number::from(-397),
            Number {
                value: -397,
                positive: false,
                sign: Sign::Negative,
                parity: Parity::Odd
            }
        )
    }

    #[test]
    #[cfg(feature = "random_tests")]
    fn test_1000000_random_i32() {
        let mut rng = thread_rng();
        for _ in 0..1000000 {
            let value = rng.gen::<i32>();
            let odd_even = if value.rem_euclid(2) == 1 {
                Parity::Odd
            } else {
                Parity::Even
            };
            let positive = value >= 0;
            let num = Number::new(value);
            assert_eq!(
                num,
                Number {
                    value,
                    positive,
                    sign: if positive {
                        Sign::Positive
                    } else {
                        Sign::Negative
                    },
                    parity: odd_even
                }
            )
        }
    }
    #[test]
    #[cfg(feature = "large_tests")]
    fn test_every_i32() {
        for value in i32::MIN..=i32::MAX {
            let odd_even = if value.rem_euclid(2) == 1 {
                Parity::Odd
            } else {
                Parity::Even
            };
            let positive = value >= 0;
            let num = Number::new(value);
            assert_eq!(
                num,
                Number {
                    value,
                    positive,
                    sign: if positive {
                        Sign::Positive
                    } else {
                        Sign::Negative
                    },
                    parity: odd_even
                }
            )
        }
    }
    #[test]
    #[cfg(feature = "prime_tests")]
    fn test_primes_i32_to_100k() {
        use std::collections::HashSet;
        use std::fs::File;
        use std::io::{prelude::*, BufReader};

        let file = File::open("src/number/primes_100k.txt").unwrap();
        let reader = BufReader::new(file);

        let mut primes = HashSet::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let value = line.parse::<i32>().unwrap();
            primes.insert(value);
        }

        for value in 0..100_000 {
            let num = Number::new(value);
            assert_eq!(num.prime(), primes.contains(&value));
        }
    }
}

#[cfg(test)]
mod u64_tests {
    use super::super::*;

    #[test]
    fn new_for_u64_max_eq() {
        assert_eq!(
            Number::new(u64::MAX),
            Number {
                value: 18_446_744_073_709_551_615,
                positive: true,
                sign: Sign::Positive,
                parity: Parity::Odd
            }
        )
    }
}
