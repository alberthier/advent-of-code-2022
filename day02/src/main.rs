use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


pub enum GameMode {
    Simple,
    ExpectedResult
}


pub enum RoundResult {
    Defeat,
    Draw,
    Victory
}

impl RoundResult {

    pub fn from(id: char) -> Option<RoundResult> {
        if id == RoundResult::Defeat.id() { return Some(RoundResult::Defeat); };
        if id == RoundResult::Draw.id() { return Some(RoundResult::Draw); };
        if id == RoundResult::Victory.id() { return Some(RoundResult::Victory); };
        None
    }

    pub fn id(&self) -> char {
        match self {
            Self::Defeat => 'X',
            Self::Draw => 'Y',
            Self::Victory => 'Z',
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::Defeat => 0,
            Self::Draw => 3,
            Self::Victory => 6
        }
    }
}


#[derive(PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {

    pub fn from(id: char) -> Option<Move> {
        if id == Self::Rock.self_id() { return Some(Self::Rock) }
        if id == Self::Paper.self_id() { return Some(Self::Paper) }
        if id == Self::Scissors.self_id() { return Some(Self::Scissors) }
        if id == Self::Rock.opponent_id() { return Some(Self::Rock) }
        if id == Self::Paper.opponent_id() { return Some(Self::Paper) }
        if id == Self::Scissors.opponent_id() { return Some(Self::Scissors) }
        None
    }

    pub fn for_expected_result(opponent_move: &Move, expected_result: RoundResult) -> Move {
        match (opponent_move, expected_result) {
            (Self::Rock, RoundResult::Defeat) => Self::Scissors,
            (Self::Rock, RoundResult::Draw) => Self::Rock,
            (Self::Rock, RoundResult::Victory) => Self::Paper,
            (Self::Paper, RoundResult::Defeat) => Self::Rock,
            (Self::Paper, RoundResult::Draw) => Self::Paper,
            (Self::Paper, RoundResult::Victory) => Self::Scissors,
            (Self::Scissors, RoundResult::Defeat) => Self::Paper,
            (Self::Scissors, RoundResult::Draw) => Self::Scissors,
            (Self::Scissors, RoundResult::Victory) => Self::Rock,
        }
    }

    pub fn opponent_id(&self) -> char {
        match self {
            Self::Rock => 'A',
            Self::Paper => 'B',
            Self::Scissors => 'C',
        }
    }

    pub fn self_id(&self) -> char {
        match self {
            Self::Rock => 'X',
            Self::Paper => 'Y',
            Self::Scissors => 'Z',
        }
    }

    pub fn inherent_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn confrontation_result(&self, opponent_move: &Self) -> RoundResult {
        match self.cmp(opponent_move) {
            Ordering::Less => RoundResult::Defeat,
            Ordering::Equal => RoundResult::Draw,
            Ordering::Greater => RoundResult::Victory
        }
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Rock, Self::Rock) => Ordering::Equal,
            (Self::Rock, Self::Paper) => Ordering::Less,
            (Self::Rock, Self::Scissors) => Ordering::Greater,
            (Self::Paper, Self::Rock) => Ordering::Greater,
            (Self::Paper, Self::Paper) => Ordering::Equal,
            (Self::Paper, Self::Scissors) => Ordering::Less,
            (Self::Scissors, Self::Rock) => Ordering::Less,
            (Self::Scissors, Self::Paper) => Ordering::Greater,
            (Self::Scissors, Self::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub struct Round {
    opponent_move: Move,
    my_move: Move,
}

impl Round {

    pub fn new(opponent_move: Move, my_move: Move) -> Round {
        Round { opponent_move, my_move }
    }

    pub fn from_line(line: &str, game_mode: &GameMode) -> Option<Round> {
        let mut move_ids = line.split(' ').map(|s| {
            s.chars().next()
        });
        let Some(opponent_move_id) = move_ids.next()? else { return None };
        let Some(my_id) = move_ids.next()? else { return None };

        let opponent_move = Move::from(opponent_move_id)?;
        let my_move = match game_mode {
            GameMode::Simple => {
                Move::from(my_id)?
            }
            GameMode::ExpectedResult =>
                Move::for_expected_result(&opponent_move, RoundResult::from(my_id)?),
        };
        Some(Round::new(opponent_move, my_move))
    }

    pub fn score(&self) -> u32 {
        self.my_move.inherent_score() +
            self.my_move.confrontation_result(&self.opponent_move).value()
    }
}


struct Game {
    rounds: Vec<Round>,
}

impl Game {

    fn from_lines(lines: impl Iterator<Item = String>, game_mode: &GameMode) -> Option<Game> {
        let mut game = Game { rounds: Vec::new() };
        for line in lines {
            let round = Round::from_line(line.as_str(), game_mode)?;
            game.rounds.push(round);
        }
        Some(game)
    }

    pub fn total_score(&self) -> u32{
        self.rounds
            .iter()
            .fold(0, |acc, round|
                acc + round.score()
            )
    }
}


fn play_game(path: &str, game_mode: &GameMode) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let Some(game) = Game::from_lines(lines, game_mode) else {
        return Err("Unable to load the game")
    };

    let total = game.total_score();

    println!("Total score of the game: {total}");

    Ok(())
}


fn main() -> Result<(), &'static str> {
    let stage = std::env::args().nth(1).expect("Expecting puzzle stage");
    let path = std::env::args().nth(2).expect("Expecting a file name");

    match stage.as_str() {
        "stage1" => play_game(path.as_str(), &GameMode::Simple),
        "stage2" => play_game(path.as_str(), &GameMode::ExpectedResult),
        _ => Err("Unknown stage")
    }
}


fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>
{
    let file = File::open(filename)?;
    let it = io::BufReader::new(file)
        .lines()
        .map(|a| a.expect("Bad line"));
    Ok(it)
}