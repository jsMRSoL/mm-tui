use rand::Rng;
use super::error::ParseBobbleStrError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Bobble {
    Red,
    Green,
    Blue,
    Purple,
    Pink,
    Yellow,
    White,
    Orange,
}

impl FromStr for Bobble {
    type Err = ParseBobbleStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Bobble::*;
        match s {
            "R" | "r" => Ok(Red),
            "G" | "g" => Ok(Green),
            "B" | "b" => Ok(Blue),
            "P" => Ok(Purple),
            "p" => Ok(Pink),
            "Y" | "y" => Ok(Yellow),
            "W" | "w" => Ok(White),
            "O" | "o" => Ok(Orange),
            _ => Err(ParseBobbleStrError(s.to_owned())),
        }
    }
}

pub fn make_secret() -> Vec<Bobble> {
    use Bobble::*;
    let mut options = vec![Red, Green, Blue, Purple, Pink, Yellow, White, Orange];
    let mut chosen: Vec<Bobble> = vec![];
    let mut rng = rand::thread_rng();
    for _ in 0..5 {
        let idx = rng.gen_range(0..options.len());
        let choice = options.swap_remove(idx);
        chosen.push(choice);
    }
    chosen
}

pub fn parse_guess(guess: String) -> Result<Vec<Bobble>, ParseBobbleStrError> {
    let mut guess_vec = guess
        .split(" ")
        .map(|c| c.trim().parse::<Bobble>())
        .collect::<Result<Vec<Bobble>, _>>()?;
    if guess_vec.len() > 5 {
        guess_vec.truncate(5);
    }
    Ok(guess_vec)
}
