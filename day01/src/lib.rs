use std::error::Error;
use std::io::{self, BufRead, Read};

const DIGIT_STRINGS: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn calibrate(input: impl Read, include_strings: bool) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let mut first_digit = None;
        let mut last_digit = 0;

        let line = line?;

        for mut i in 0..line.len() {
            let mut digit = None;

            let first_char = line[i..].chars().next().unwrap_or_default();

            if first_char.is_digit(10) {
                digit = Some(first_char.to_digit(10).unwrap_or_default());
            } else if include_strings {
                for (index, &string) in DIGIT_STRINGS.iter().enumerate() {
                    if line[i..].starts_with(string) {
                        digit = Some(index as u32);
                        i += string.len() - 1;
                        break;
                    }
                }
            }

            if let Some(d) = digit {
                if first_digit.is_none() {
                    first_digit = Some(d);
                }
                last_digit = d;
            }
        }

        sum += first_digit.unwrap_or(0) * 10 + last_digit;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calibrate_without_strings() -> Result<(), Box<dyn Error>> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(142, calibrate(input.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn calibrate_with_strings() -> Result<(), Box<dyn Error>> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(281, calibrate(input.as_bytes(), true)?);

        Ok(())
    }
}
