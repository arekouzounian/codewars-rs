#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::Display;

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
    const VOWELS: &str = "aeiou";

    s.chars()
        .filter(|c| VOWELS.find(c.to_ascii_lowercase()).is_none())
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

fn persistence(num: u64) -> u64 {
    let multiply_digits = |x: u64| -> u64 {
        let mut mult: u64 = 1;
        x.to_string()
            .chars()
            .map(|c| c.to_digit(10).expect("invalid digit") as u64)
            .for_each(|d| mult *= d);

        mult
    };

    let count_digits = |x: u64| -> u64 { x.to_string().len() as u64 };

    let mut i = 0;
    let mut num = num;
    while count_digits(num) > 1 {
        num = multiply_digits(num);
        i += 1;
    }

    i
}

fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = vec![0; arr.len()];
    let mut j = 0;
    for i in 0..arr.len() {
        if arr[i] != 0 {
            v[j] = arr[i];
            j += 1;
        }
    }

    v
}

fn good_vs_evil(good: &str, evil: &str) -> String {
    let g_vals = [1, 2, 3, 3, 4, 10];
    let e_vals = [1, 2, 2, 2, 3, 5, 10];
    let get_cnt = |strn: &str, vals: &[u32]| {
        let mut cnt = 0;
        let mut i = 0;
        for x in strn.split(' ') {
            if let Ok(d) = x.parse::<u32>() {
                cnt += vals[i] * d;
                i += 1;
            }
        }

        cnt
    };

    let g_cnt = get_cnt(good, &g_vals);
    let e_cnt = get_cnt(evil, &e_vals);

    if g_cnt > e_cnt {
        String::from("Battle Result: Good triumphs over Evil")
    } else if g_cnt < e_cnt {
        String::from("Battle Result: Evil eradicates all trace of Good")
    } else {
        String::from("Battle Result: No victor on this battle field")
    }
}

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut table = vec![vec![0; len]; len];

    for i in 0..len {
        for j in 0..len {
            table[i][j] = (j + 1) * (i + 1);
        }
    }

    table
}

fn parts_sums(ls: &[u64]) -> Vec<u64> {
    let mut ret: Vec<u64> = vec![0; ls.len() + 1];

    let s = ls.iter().sum();
    ret[0] = s;
    for i in 1..ret.len() {
        ret[i] = ret[i - 1] - ls[i - 1]
    }

    ret
}

fn parse(code: &str) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    let mut val: i32 = 0;
    for c in code.chars() {
        match c {
            'i' => {
                val += 1;
            }
            'd' => {
                val -= 1;
            }
            's' => {
                val = val.pow(2);
            }
            'o' => {
                ret.push(val);
            }
            _ => (),
        };
    }

    ret
}

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    // construct a hashmap of value to index
    // iterate through ints
    // for each int check if s - int[i] exists in hashmap
    // if it does, and the index is greater than i but less than curr, store it in tup

    let map: HashMap<&i8, usize> = ints.iter().enumerate().map(|e| (e.1, e.0)).collect();
    let mut curr: Option<(i8, i8)> = None;
    for (i, x) in ints.iter().enumerate() {
        if let Some(ind) = map.get(&(s - *x)) {
            match curr {
                Some((a, b)) => {
                    if *ind > i && ind < map.get(&b).unwrap() {
                        curr = Some((a, ints[i]));
                    }
                }
                None => curr = Some((*x, s - *x)),
            }
        }
    }

    curr
}

fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence
        .into_iter()
        .fold(vec![], |mut acc: Vec<T::Item>, x| {
            if let Some(y) = acc.last() {
                if *y != x {
                    acc.push(x);
                }
            } else if acc.len() < 1 {
                acc.push(x);
            }

            acc
        })

    /*
        use std::iter::FromIterator;
            /* ...fn declaration... */
            let mut vec = Vec::from_iter(iter);
            vec.dedup();
            vec
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }

    // #[test]
    // fn sum_pairs_returns_expected() {
    //     let l1 = [1, 4, 8, 7, 3, 15];
    //     let l2 = [1, -2, 3, 0, -6, 1];
    //     let l3 = [20, -13, 40];
    //     let l4 = [1, 2, 3, 4, 1, 0];
    //     let l5 = [10, 5, 2, 3, 7, 5];
    //     let l6 = [4, -2, 3, 3, 4];
    //     let l7 = [0, 2, 0];
    //     let l8 = [5, 9, 13, -3];
    //     assert_eq!(sum_pairs(&l1, 8), Some((1, 7)));
    //     assert_eq!(sum_pairs(&l2, -6), Some((0, -6)));
    //     assert_eq!(sum_pairs(&l3, -7), None);
    //     assert_eq!(sum_pairs(&l4, 2), Some((1, 1)));
    //     assert_eq!(sum_pairs(&l5, 10), Some((3, 7)));
    //     assert_eq!(sum_pairs(&l6, 8), Some((4, 4)));
    //     assert_eq!(sum_pairs(&l7, 0), Some((0, 0)));
    //     assert_eq!(sum_pairs(&l8, 10), Some((13, -3)));
    // }

    #[test]
    fn deadfish_sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }

    fn dotest(ls: Vec<u64>, expect: Vec<u64>) {
        let actual = parts_sums(&ls);
        assert_eq!(actual, expect);
    }

    #[test]
    fn example() {
        dotest(vec![], vec![0]);
        dotest(vec![0, 1, 3, 6, 10], vec![20, 20, 19, 16, 10, 0]);
        dotest(vec![1, 2, 3, 4, 5, 6], vec![21, 20, 18, 15, 11, 6, 0]);
        dotest(
            vec![
                744125, 935, 407, 454, 430, 90, 144, 6710213, 889, 810, 2579358,
            ],
            vec![
                10037855, 9293730, 9292795, 9292388, 9291934, 9291504, 9291414, 9291270, 2581057,
                2580168, 2579358, 0,
            ],
        );
    }

    #[test]
    fn basic() {
        assert_eq!(multiplication_table(3), [[1, 2, 3], [2, 4, 6], [3, 6, 9]]);
    }

    #[test]
    fn returns_expected() {
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
            "Battle Result: Good triumphs over Evil"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
            "Battle Result: Evil eradicates all trace of Good"
        );
        assert_eq!(
            good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
            "Battle Result: No victor on this battle field"
        );
    }

    fn mz_dotest(a: &[u8], expected: &[u8]) {
        let actual = move_zeros(a);
        assert!(
            actual == expected,
            "With arr = {a:?}\nExpected {expected:?} but got {actual:?}"
        )
    }

    #[test]
    fn move_zeros_sample_tests() {
        mz_dotest(
            &[1, 2, 0, 1, 0, 1, 0, 3, 0, 1],
            &[1, 2, 1, 1, 3, 1, 0, 0, 0, 0],
        );
        mz_dotest(
            &[9, 0, 0, 9, 1, 2, 0, 1, 0, 1, 0, 3, 0, 1, 9, 0, 0, 0, 0, 9],
            &[9, 9, 1, 2, 1, 1, 3, 1, 9, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        );
        mz_dotest(&[0, 0], &[0, 0]);
        mz_dotest(&[0], &[0]);
        mz_dotest(&[], &[]);
    }

    #[test]
    fn persistence_sample_tests() {
        assert_eq!(super::persistence(39), 3);
        assert_eq!(super::persistence(4), 0);
        assert_eq!(super::persistence(25), 2);
        assert_eq!(super::persistence(999), 4);
    }

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
