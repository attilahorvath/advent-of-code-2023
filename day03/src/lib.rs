use std::error::Error;
use std::io::{self, BufRead, Read};

fn has_symbol(line: &Option<Vec<char>>, index: usize, width: usize, current: bool) -> bool {
    if let Some(l) = line {
        if (index > 0 && !l[index - 1].is_digit(10) && l[index - 1] != '.')
            || (!current && !l[index].is_digit(10) && l[index] != '.')
            || (index < width - 1 && !l[index + 1].is_digit(10) && l[index + 1] != '.')
        {
            return true;
        }
    }

    false
}

pub fn sum_part_numbers(input: impl Read) -> Result<u32, Box<dyn Error>> {
    let mut lines = io::BufReader::new(input).lines();

    let mut prev_line: Option<Vec<char>> = None;
    let mut curr_line: Option<Vec<char>> = None;
    let mut next_line: Option<Vec<char>>;

    let mut width = 0;
    let mut number = 0;
    let mut part_number = false;

    let mut sum = 0;

    loop {
        if let Some(line) = lines.next() {
            next_line = Some(line?.chars().collect::<Vec<_>>());
        } else {
            next_line = None;
        }

        if let Some(ref line) = curr_line {
            if width == 0 {
                width = line.len();
            }

            for (index, char) in line.iter().enumerate() {
                if char.is_digit(10) {
                    number *= 10;
                    number += char.to_digit(10).unwrap_or_default();

                    part_number = part_number
                        || has_symbol(&prev_line, index, width, false)
                        || has_symbol(&curr_line, index, width, true)
                        || has_symbol(&next_line, index, width, false);
                }

                if !char.is_digit(10) || index == width - 1 {
                    if part_number {
                        sum += number;
                    }

                    number = 0;
                    part_number = false;
                }
            }

            number = 0;
            part_number = false;
        }

        if next_line.is_none() {
            break;
        }

        prev_line = curr_line;
        curr_line = next_line;
    }

    Ok(sum)
}

fn number_at(line: &Option<Vec<char>>, index: usize, width: usize) -> Option<u32> {
    if let Some(l) = line {
        if !l[index].is_digit(10) {
            return None;
        }

        let mut i = index;

        loop {
            if i == 0 {
                break;
            }

            if l[i - 1].is_digit(10) {
                i -= 1;
            } else {
                break;
            }
        }

        let mut number = 0;

        while i <= width - 1 && l[i].is_digit(10) {
            number *= 10;
            number += l[i].to_digit(10).unwrap_or_default();

            i += 1;
        }

        return Some(number);
    }

    None
}

fn numbers(line: &Option<Vec<char>>, index: usize, width: usize, current: bool) -> (u32, u32) {
    if let Some(l) = line {
        let mut product = 1;
        let mut count = 0;

        if index > 0 {
            if let Some(l) = number_at(line, index - 1, width) {
                product *= l;
                count += 1;
            }
        }

        if !current && (index == 0 || !l[index - 1].is_digit(10)) {
            if let Some(m) = number_at(line, index, width) {
                product *= m;
                count += 1;
            }
        }

        if index < width - 1 && !l[index].is_digit(10) {
            if let Some(r) = number_at(line, index + 1, width) {
                product *= r;
                count += 1;
            }
        }

        return (product, count);
    }

    (1, 0)
}

pub fn sum_gear_ratios(input: impl Read) -> Result<u32, Box<dyn Error>> {
    let mut lines = io::BufReader::new(input).lines();

    let mut prev_line: Option<Vec<char>> = None;
    let mut curr_line: Option<Vec<char>> = None;
    let mut next_line: Option<Vec<char>>;

    let mut width = 0;
    let mut sum = 0;

    loop {
        if let Some(line) = lines.next() {
            next_line = Some(line?.chars().collect::<Vec<_>>());
        } else {
            next_line = None;
        }

        if let Some(ref line) = curr_line {
            if width == 0 {
                width = line.len();
            }

            for (index, &char) in line.iter().enumerate() {
                if char == '*' {
                    let prev_numbers = numbers(&prev_line, index, width, false);
                    let curr_numbers = numbers(&curr_line, index, width, true);
                    let next_numbers = numbers(&next_line, index, width, false);

                    if prev_numbers.1 + curr_numbers.1 + next_numbers.1 == 2 {
                        sum += prev_numbers.0 * curr_numbers.0 * next_numbers.0;
                    }
                }
            }
        }

        if next_line.is_none() {
            break;
        }

        prev_line = curr_line;
        curr_line = next_line;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_part_numbers() -> Result<(), Box<dyn Error>> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(4361, sum_part_numbers(input.as_bytes())?);

        Ok(())
    }

    #[test]
    fn total_gear_ratios() -> Result<(), Box<dyn Error>> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(467835, sum_gear_ratios(input.as_bytes())?);

        Ok(())
    }
}
