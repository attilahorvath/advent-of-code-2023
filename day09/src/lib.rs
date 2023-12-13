use std::error::Error;
use std::io::{self, BufRead, Read};

fn extrapolate(values: &[i64], backwards: bool) -> i64 {
    if values.iter().all(|&value| value == values[0]) {
        return values[0];
    }

    let differences = values
        .windows(2)
        .map(|items| {
            if backwards {
                items[0] - items[1]
            } else {
                items[1] - items[0]
            }
        })
        .collect::<Vec<_>>();

    let difference = extrapolate(&differences, backwards);

    values[if backwards { 0 } else { values.len() - 1 }] + difference
}

pub fn sum_values(input: impl Read, backwards: bool) -> Result<i64, Box<dyn Error>> {
    let mut sum = 0;

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        let values = l
            .split_ascii_whitespace()
            .map(|value| value.parse().unwrap_or_default())
            .collect::<Vec<_>>();

        sum += extrapolate(&values, backwards);
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn sum_values_forward() -> Result<(), Box<dyn Error>> {
        assert_eq!(114, sum_values(INPUT.as_bytes(), false)?);

        Ok(())
    }

    #[test]
    fn sum_values_backwards() -> Result<(), Box<dyn Error>> {
        assert_eq!(2, sum_values(INPUT.as_bytes(), true)?);

        Ok(())
    }
}
