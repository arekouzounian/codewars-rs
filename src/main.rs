use std::{ascii::AsciiExt, fmt::Display};

fn array_diff<T: PartialEq + Display>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let a: Vec<T> = a.into_iter().filter(|x| !b.contains(x)).collect();
    a
}

fn create_phone_number(numbers: &[u8]) -> String {
    let mut ret = String::new();

    ret.push('(');
    for (i, x) in (1..=numbers.len()).zip(numbers.iter()) {
        ret.push(char::from_digit(*x as u32, 10).expect("Invalid number."));

        if i == 3 {
            ret.push_str(") ");
        } else if i == 6 {
            ret.push('-');
        }
    }

    ret
}

fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }

    let upper: i64 = (x as f64).sqrt() as i64;
    for i in 2..=upper {
        if x % i == 0 {
            return false;
        }
    }

    true
}

fn find_next_square(sq: u64) -> Option<u64> {
    let sqrt = (sq as f64).sqrt();

    if sqrt != sqrt.floor() {
        return None;
    }

    let sqrt = sqrt + 1.0;

    Option::Some(sqrt.powi(2) as u64)
}

fn find_missing_letter(chars: &[char]) -> char {
    let mut alpha = String::from("abcdefghijklmnopqrstuvwxyz");
    if chars[0].is_ascii_uppercase() {
        alpha = alpha.to_ascii_uppercase();
    }

    let idx = match alpha.find(chars[0]) {
        Some(i) => i,
        None => return 'a',
    };

    for (a, c) in (&alpha[idx..idx + chars.len()]).chars().zip(chars.iter()) {
        if a != *c {
            return a;
        }
    }

    'a'
}

fn disemvowel(s: &str) -> String {
    const vowels: &str = "aeiou";

    s.chars()
        .filter(|c| vowels.find(c.to_ascii_lowercase()).is_none())
        .collect()
}

fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|d| {
            d.to_digit(10)
                .expect("unidentified digit!")
                .pow(2)
                .to_string()
        })
        .collect::<String>()
        .parse::<u64>()
        .expect("invalid format")
}

fn is_square(n: i64) -> bool {
    let sqrt = (n as f64).sqrt();

    sqrt.ceil() == sqrt
}

fn find_outlier(values: &[i32]) -> i32 {
    *values
        .windows(3)
        .find_map(|w| {
            let even: Vec<&i32> = w.iter().filter(|&&x| x % 2 == 0).collect();
            let odd: Vec<&i32> = w.iter().filter(|&&x| x % 2 != 0).collect();

            if even.len() > 0 && odd.len() > 0 {
                return Some(if even.len() > odd.len() {
                    odd[0]
                } else {
                    even[0]
                });
            }
            None
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }

    #[test]
    fn fixed_tests() {
        assert_eq!(
            is_square(-1),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(0),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(3),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(4),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(25),
            true,
            "\nYour answer (left) is not the expected answer (right)."
        );
        assert_eq!(
            is_square(26),
            false,
            "\nYour answer (left) is not the expected answer (right)."
        );
    }

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }

    #[test]
    fn example_test() {
        assert_eq!(
            disemvowel("This website is for losers LOL!"),
            "Ths wbst s fr lsrs LL!"
        );
    }

    #[test]
    fn example_tests() {
        assert_eq!(find_missing_letter(&['a', 'b', 'c', 'd', 'f']), 'e');
        assert_eq!(find_missing_letter(&['O', 'Q', 'R', 'S']), 'P');
    }

    #[test]
    fn sample_tests() {
        assert_eq!(find_next_square(121), Some(144));
        assert_eq!(find_next_square(625), Some(676));
        assert_eq!(find_next_square(319_225), Some(320_356));
        assert_eq!(find_next_square(15_241_383_936), Some(15_241_630_849));
        assert_eq!(find_next_square(155), None);
        assert_eq!(find_next_square(342_786_627), None);
    }

    #[test]
    fn array_diff_returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
        assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
    }

    #[test]
    fn create_phone_number_returns_expected() {
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
            "(123) 456-7890"
        );
        assert_eq!(
            create_phone_number(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            "(111) 111-1111"
        );
        assert_eq!(
            create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]),
            "(123) 456-7899"
        );
    }

    #[test]
    fn basic_tests() {
        assert!(!is_prime(0), "0 is not prime");
        assert!(!is_prime(1), "1 is not prime");
        assert!(is_prime(2), "2 is prime");
        assert!(is_prime(73), "73 is prime");
        assert!(!is_prime(75), "75 is not prime");
        assert!(!is_prime(-1), "-1 is not prime");
    }

    #[test]
    fn prime_tests() {
        assert!(is_prime(3), "3 is prime");
        assert!(is_prime(5), "5 is prime");
        assert!(is_prime(7), "7 is prime");
        assert!(is_prime(41), "41 is prime");
        assert!(is_prime(5099), "5099 is prime");
    }

    #[test]
    fn not_prime_tests() {
        assert!(!is_prime(4), "4 is not prime");
        assert!(!is_prime(6), "6 is not prime");
        assert!(!is_prime(8), "8 is not prime");
        assert!(!is_prime(9), "9 is not prime");
        assert!(!is_prime(45), "45 is not prime");
        assert!(!is_prime(-5), "-5 is not prime");
        assert!(!is_prime(-8), "-8 is not prime");
        assert!(!is_prime(-41), "-41 is not prime");
    }
}

fn main() {
    println!("Run cargo test!");
}
