#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::str;

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

    for (a, c) in (alpha[idx..idx + chars.len()]).chars().zip(chars.iter()) {
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

            if !even.is_empty() && !odd.is_empty() {
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
    for item in arr {
        if *item != 0 {
            v[j] = *item;
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

    match g_cnt.cmp(&e_cnt) {
        Ordering::Greater => String::from("Battle Result: Good triumphs over Evil"),
        Ordering::Less => String::from("Battle Result: Evil eradicates all trace of Good"),
        Ordering::Equal => String::from("Battle Result: No victor on this battle field"),
    }
}

fn multiplication_table(len: usize) -> Vec<Vec<usize>> {
    let mut table = vec![vec![0; len]; len];

    for (i, row) in table.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = (j + 1) * (i + 1);
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
            } else if acc.is_empty() {
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

fn revrot(s: &str, c: usize) -> String {
    if c == 0 || s.is_empty() || c > s.len() {
        return String::new();
    }

    let mut ret = String::with_capacity(s.len());
    for chunk in s.as_bytes().chunks(c) {
        if chunk.len() < c {
            continue;
        }

        let sum_cubed_digits: u32 = chunk.iter().fold(0, |acc, x| {
            if let Some(d) = (*x as char).to_digit(10) {
                acc + d.pow(3)
            } else {
                acc
            }
        });

        if sum_cubed_digits % 2 == 0 {
            // reverse
            for &byte in chunk.iter().rev() {
                ret.push(byte as char);
            }
        } else {
            // rotate
            for &byte in chunk.iter().skip(1) {
                ret.push(byte as char);
            }
            ret.push(*(chunk.first().unwrap()) as char);
        }
    }

    ret
}

fn camel_case(str: &str) -> String {
    str.split(' ').fold(String::new(), |mut acc, word| {
        acc.extend(word.chars().enumerate().map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().next().unwrap()
            } else {
                c
            }
        }));

        acc
    })
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // naive approach: triple nested for loop that explicitly checks the condition
    // runtime: O(n^3)

    // can be improved by iterating through with a double for-loop and calculating
    // sums of pairs, then iterating once more for the third index
    // ^ gonna try this implementation
    // tried the implementation and it didn't work; didn't test for duplicates.

    // Actual solution (I didn't come up with this):
    // sort the vec, loop through it and have two pointers that look at the rest of the array
    // whenever a triplet is found, add it to a set

    use std::collections::HashSet;
    let mut nums = nums;
    nums.sort();

    let mut sol = HashSet::new();
    let mut j = nums.iter().enumerate().skip(1).peekable(); // n guaranteed to be at least length 3
    let mut k = nums.iter().enumerate().rev().peekable();

    nums.iter().enumerate().fold(&mut sol, |acc, (i, num)| {
        if let (Some((ji, jn)), Some((ki, kn))) = (j.peek(), k.peek()) {
            if (i != *ji && i != *ki && *ji != *ki) && (*num + **jn + **kn) == 0 {
                acc.insert(vec![*num, **jn, **kn]);
                j.next();
                k.next();
            }
        }

        acc
    });

    let len = sol.len();
    sol.drain().fold(Vec::with_capacity(len), |mut acc, v| {
        acc.push(v);
        acc
    })
}

fn get_submatrix(ind: usize, matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    assert!(ind < matrix.len());

    let mut x: Vec<Vec<i64>> = Vec::with_capacity(matrix.len() - 1);

    for row in matrix.iter().skip(1) {
        x.push(
            row.iter()
                .enumerate()
                .filter(|(i, _)| *i != ind)
                .map(|(_, x)| *x)
                .collect::<Vec<i64>>(),
        )
    }

    return x;
}

fn print_matrix(matrix: &[Vec<i64>]) {
    for row in matrix {
        println!("{:?}", &row);
    }
}

fn determinant(matrix: &[Vec<i64>]) -> i64 {
    if matrix.len() == 1 {
        return matrix[0][0];
    }

    let mut det = 0;
    let mut polarity: i8 = 1;
    for (i, cell) in matrix[0].iter().enumerate() {
        det += (polarity as i64) * (*cell * determinant(&get_submatrix(i, matrix)));
        polarity *= -1;
    }

    det
}

#[cfg(test)]
mod tests {
    use super::*;

    const DET_ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn det_dotest(a: &[Vec<i64>], expected: i64) {
        assert_eq!(determinant(a), expected, "{DET_ERR_MSG}")
    }

    #[test]
    fn determinant_test() {
        det_dotest(&[vec![1]], 1);
        det_dotest(&[vec![1, 3], vec![2, 5]], -1);
        det_dotest(&[vec![2, 5, 3], vec![1, -2, -1], vec![1, 3, 4]], -20);
    }

    #[test]
    fn camel_case_sample_test() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }

    fn testing(s: &str, c: usize, exp: &str) -> () {
        assert_eq!(&revrot(s, c), exp)
    }
    #[test]
    fn basics_revrot() {
        testing("1234", 0, "");
        testing("", 0, "");
        testing("1234", 5, "");
        let s = "733049910872815764";
        testing(s, 5, "330479108928157");
        let s = "73304991087281576455176044327690580265896";
        testing(s, 8, "1994033775182780067155464327690480265895");
    }

    #[test]
    fn sample_test() {
        assert_eq!(
            unique_in_order("AAAABBBCCDAABBB".chars()),
            vec!['A', 'B', 'C', 'D', 'A', 'B']
        );
    }

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
    let x = three_sum(vec![-1, 0, 1, 2, -1, -4]);

    println!("{:?}", x);
}
