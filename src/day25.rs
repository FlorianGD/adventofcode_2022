fn snafu_to_int(s: &str) -> isize {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5isize.pow(i as u32)
                * match c {
                    '0' => 0isize,
                    '1' => 1,
                    '2' => 2,
                    '-' => -1,
                    '=' => -2,
                    _ => unreachable!(),
                }
        })
        .sum()
}

fn int_to_snafu(val: isize) -> String {
    let mut s = vec![];
    let n = (val as f32).log(5f32).round() as u32;
    let mut carry = 0;
    for i in 0..=n {
        let mut r = (val / 5isize.pow(i)) % 5;
        r += carry;

        s.push(match r {
            0..=2 => {
                carry = 0;
                format!("{r}")
            }
            3 => {
                carry = 1;
                "=".into()
            }
            4 => {
                carry = 1;
                "-".into()
            }
            5 => {
                carry = 1;
                "0".into()
            }
            _ => unreachable!(),
        });
    }
    s.into_iter().rev().collect()
}

pub fn parse_input(input: &str) -> Vec<isize> {
    input.lines().map(snafu_to_int).collect()
}

pub fn part1(input: Vec<isize>) -> String {
    let val = input.iter().sum();
    int_to_snafu(val)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_snafu_to_int() {
        assert_eq!(snafu_to_int("1-0---0"), 12345);
        assert_eq!(snafu_to_int("1121-1110-1=0"), 314159265);
    }

    #[test]
    fn test_int_to_snafu() {
        assert_eq!(int_to_snafu(1), "1".to_string());
        assert_eq!(int_to_snafu(3), "1=".to_string());
        assert_eq!(int_to_snafu(4), "1-".to_string());
        assert_eq!(int_to_snafu(5), "10".to_string());
        assert_eq!(int_to_snafu(6), "11".to_string());
        assert_eq!(int_to_snafu(7), "12".to_string());
        assert_eq!(int_to_snafu(8), "2=".to_string());
        assert_eq!(int_to_snafu(9), "2-".to_string());
        assert_eq!(int_to_snafu(10), "20".to_string());
        assert_eq!(int_to_snafu(15), "1=0".to_string());
        assert_eq!(int_to_snafu(20), "1-0".to_string());
        assert_eq!(int_to_snafu(2022), "1=11-2".to_string());
        assert_eq!(int_to_snafu(12345), "1-0---0".to_string());
        assert_eq!(int_to_snafu(314159265), "1121-1110-1=0".to_string());
    }
}
