extern crate util;

use std::collections::HashSet;
use std::str;

use util::sequences::triangle;

// need to figure out a better way to handle files.
const CONTENT: &'static [u8] = include_bytes!("../dat/words.txt");

// WordScore calculates the score of a word
trait WordScore {
    fn scores(&self) -> Vec<u64>;

    fn score(&self) -> u64 {
        self.scores().iter().fold(0, |mut sum, i| { sum += i; sum })
    }
}

// Implementation of WordScore for a &str. Only scores is needed since the default
// method for score will suffice.
impl<'a> WordScore for &'a str {
    fn scores(&self) -> Vec<u64> {
        self.chars().map( |c| c as u64 - 64 ).collect()
    }
}

// Experimenting with using a result here with the `?` operator.
fn get_result() -> Result<u64, String> {
    let trimmed_content = str::from_utf8(CONTENT).map_err(|e| e.to_string())?.trim();

    // trim off quotes from the strings (the first and last character)
    // a pretty naïve approach
    let words: Vec<&str> = trimmed_content.split(",").map( |s| &s[1..s.len()-1] ).collect();

    let scores: Vec<_> = words.iter().map(|w| w.score()).collect();

    let max_triangle = *scores.iter().max().unwrap_or(&0u64);

    // T(max_triangle) is unnecessarily high, but the take_while will catch the overflow here, 
    let triangles: HashSet<_> = (0..max_triangle).map(|i| triangle(i)).take_while(|&i| i <= max_triangle ).collect();

    let result = scores.iter().filter(|i| triangles.contains(i)).count() as u64;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_result() {
        assert_eq!(Ok(162), get_result());
    }
}

fn main() {
    let result = get_result();
    println!("{:?}", result.unwrap());
}
