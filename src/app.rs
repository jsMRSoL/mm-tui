use super::bobbles::Bobble;
use super::bobbles::{make_secret, parse_guess};
use crate::colours::{make_bobbles_span, HEADER_STYLE};
use tui::text::{Span, Spans, Text};

#[derive(PartialEq, Eq)]
pub enum GameState {
    Win,
    Loss,
    InProgress,
}

pub struct App<'app> {
    pub header: Text<'static>,
    pub board: Vec<Text<'static>>,
    pub input: Vec<char>,
    pub input_cursor: usize,
    pub secret: Vec<Bobble>,
    pub turn: u8,
    pub game_state: GameState,
    pub guesses: Vec<Spans<'app>>,
}

impl<'app> App<'app> {
    pub fn new() -> App<'app> {
        let mut header = Text::styled("Welcome to Mastermind Trainer!\n", *HEADER_STYLE);
        header.extend(Text::from("Red:  R/r, Green: G/g, Blue: B/b, Purple: P,"));
        header.extend(Text::from("Pink: p, Yellow: Y/y, White: W/w, Orange: O/o."));

        return App {
            header,
            board: Vec::new(),
            input: Vec::new(),
            input_cursor: 0,
            secret: make_secret(),
            turn: 0,
            game_state: GameState::InProgress,
            guesses: Vec::new(),
        };
    }

    pub fn store(&mut self, c: char) {
        self.input.push(c)
    }

    pub fn input(&self) -> &[char] {
        &self.input
    }

    pub fn input_write(&mut self, character: char) {
        self.input.insert(self.input_cursor, character);
        self.input_cursor += 1;
    }

    pub fn input_remove(&mut self) {
        if self.input_cursor < self.input.len() {
            self.input.remove(self.input_cursor);
        }
    }

    pub fn input_remove_previous(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
            self.input.remove(self.input_cursor);
        }
    }

    pub fn left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.input_cursor < self.input.len() {
            self.input_cursor += 1;
        }
    }

    pub fn submit_guess(&mut self) {
        if self.input.len() > 0 {
            self.turn += 1;
            let raw_guess = self.input.drain(..).collect::<String>();
            self.input_cursor = 0;
            // TODO: Error handling
            let guess_vec = parse_guess(raw_guess).unwrap();
            let mut right_col_right_pos: usize = 0;
            let mut right_col: usize = 0;
            let mut rest: usize = 5;
            for b in 0..5 {
                if self.secret[b] == guess_vec[b] {
                    right_col_right_pos += 1;
                }
                if self.secret.contains(&guess_vec[b]) {
                    right_col += 1;
                }
            }

            if right_col_right_pos == 5 {
                self.game_state = GameState::Win;
                return;
            }

            rest = rest - right_col;
            right_col = right_col - right_col_right_pos;

            let mut guess_assessment = vec![
                Span::raw(format!(" Guess {}: ", self.turn)),
            ];

            let bobble_span = make_bobbles_span(&guess_vec);
            guess_assessment.extend(bobble_span);
            guess_assessment.push(
                Span::raw(format!(
                    " Marks: {}{}{}",
                    "Y".repeat(right_col_right_pos),
                    "y".repeat(right_col),
                    "-".repeat(rest)
                )));

            let guess_assessment = Spans::from(guess_assessment);

            self.guesses.push(guess_assessment);
        }
    }
}
