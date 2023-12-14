use std::error::Error;
use std::io::{self, BufRead, Read};

pub fn sum_lengths(input: impl Read, rate: u64) -> Result<u64, Box<dyn Error>> {
    let mut sum = 0;

    let mut galaxies = vec![];
    let mut y = 0u64;

    for line in io::BufReader::new(input).lines() {
        let l = line?;

        for (x, char) in l.chars().enumerate() {
            if char == '#' {
                galaxies.push((x as u64, y));
            }
        }

        y += 1;
    }

    let mut non_empty_rows = galaxies.iter().map(|galaxy| galaxy.1).collect::<Vec<_>>();
    let mut non_empty_cols = galaxies.iter().map(|galaxy| galaxy.0).collect::<Vec<_>>();

    non_empty_rows.sort();
    non_empty_rows.dedup();

    non_empty_cols.sort();
    non_empty_cols.dedup();

    let rate = rate - 1;

    for (i, galaxy_a) in galaxies.iter().enumerate() {
        for galaxy_b in galaxies[(i + 1)..].iter() {
            let min_x;
            let max_x;
            let min_y;
            let max_y;

            if galaxy_a.0 < galaxy_b.0 {
                min_x = galaxy_a.0;
                max_x = galaxy_b.0;
            } else {
                min_x = galaxy_b.0;
                max_x = galaxy_a.0;
            }

            if galaxy_a.1 < galaxy_b.1 {
                min_y = galaxy_a.1;
                max_y = galaxy_b.1;
            } else {
                min_y = galaxy_b.1;
                max_y = galaxy_a.1;
            }

            sum += (max_x - min_x)
                + ((max_x - min_x)
                    - (min_x..max_x)
                        .filter(|x| non_empty_cols.contains(&x))
                        .count() as u64)
                    * rate;

            sum += (max_y - min_y)
                + ((max_y - min_y)
                    - (min_y..max_y)
                        .filter(|y| non_empty_rows.contains(&y))
                        .count() as u64)
                    * rate;
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn sum_lengths_base_rate() -> Result<(), Box<dyn Error>> {
        assert_eq!(374, sum_lengths(INPUT.as_bytes(), 2)?);

        Ok(())
    }

    #[test]
    fn sum_lengths_rate_of_10() -> Result<(), Box<dyn Error>> {
        assert_eq!(1030, sum_lengths(INPUT.as_bytes(), 10)?);

        Ok(())
    }

    #[test]
    fn sum_lengths_rate_of_100() -> Result<(), Box<dyn Error>> {
        assert_eq!(8410, sum_lengths(INPUT.as_bytes(), 100)?);

        Ok(())
    }
}
