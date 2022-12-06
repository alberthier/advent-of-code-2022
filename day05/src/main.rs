use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use once_cell::sync::Lazy;
use regex::Regex;


type Crate = char;


pub fn crate_from_str(s: &str) -> Option<Crate> {
    static CRATE_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
        Regex::new(r"\[(?P<id>\w)\]\s*")
            .expect("Unable to create the crates regex")
    });
    let id = CRATE_REGEX.captures(s)?.name("id")?.as_str();
    if id.len() != 1 { return None };
    Some(id.chars().next()?)
}


type CratesStack = Vec<Crate>;

struct CratesStock {

    stock: Vec<CratesStack>
}

impl CratesStock {

    pub fn from(schema_buffer: &Vec::<String>) -> Option<CratesStock> {
        let mut it = schema_buffer.iter();
        let stacks_count: usize = it.next()
            ?.split(' ')
            .filter_map(|x| {
                x.parse().ok()
            })
            .last()?;

        let mut stock = Vec::<CratesStack>::new();
        for _ in 0..stacks_count {
            stock.push(CratesStack::new());
        }

        for line in it {
            let mut current = line.as_str();
            let mut index = 0;
            while !current.is_empty() {
                let (chunk, rest) = current.split_at(std::cmp::min(4, current.len()));
                if let Some(crate_) = crate_from_str(chunk) {
                    stock[index].push(crate_);
                }
                index += 1;
                current = rest;
            }
        }

        Some(CratesStock { stock })
    }

    pub fn _dump(&self) {
        for stack in self.stock.iter() {
            for crate_ in stack {
                print!("{}", crate_);
            }
            println!("")
        }
    }

    pub fn top_of_stacks(&self) -> String {
        self.stock
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect()
    }

}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl Move {

    pub fn from(line: String) -> Option<Move> {
        static MOVE_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
            Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)")
                .expect("Unable to create the move regex")
        });

        let captures = MOVE_REGEX.captures(line.as_str())?;
        let count = captures.name("count")?.as_str().parse::<usize>().ok()?;
        let from = captures.name("from")?.as_str().parse::<usize>().ok()? - 1;
        let to = captures.name("to")?.as_str().parse::<usize>().ok()? - 1;

        Some(Move { count, from, to })
    }
}


struct CraneMoves {
    moves: Vec<Move>,
}

impl CraneMoves {

    pub fn from(lines: impl Iterator<Item = String>) -> Option<CraneMoves> {
        let mut moves = Vec::<Move>::new();
        for line in lines {
            let move_ = Move::from(line)?;
            moves.push(move_);
        }
        Some(CraneMoves { moves })
    }
}

trait Crane {
    fn execute(&self, moves: &CraneMoves, crates_stock: &mut CratesStock);
}

struct CrateMover9000 {
}

impl CrateMover9000 {

    pub fn new() -> CrateMover9000 {
        CrateMover9000 {}
    }
}

impl Crane for CrateMover9000 {

    fn execute(&self, crane_moves: &CraneMoves, crates_stock: &mut CratesStock) {
        for move_ in crane_moves.moves.iter() {
            for _ in 0..move_.count {
                let crate_ = crates_stock.stock[move_.from].pop().expect("Invalid move");
                crates_stock.stock[move_.to].push(crate_);
            }
        }
    }
}


struct CrateMover9001 {
}

impl CrateMover9001 {

    pub fn new() -> CrateMover9001 {
        CrateMover9001 {}
    }
}

impl Crane for CrateMover9001 {

    fn execute(&self, crane_moves: &CraneMoves, crates_stock: &mut CratesStock) {
        for move_ in crane_moves.moves.iter() {
            let dest_index = crates_stock.stock[move_.to].len();
            for _ in 0..move_.count {
                let crate_ = crates_stock.stock[move_.from].pop().expect("Invalid move");
                crates_stock.stock[move_.to].insert(dest_index, crate_);
            }
        }
    }
}

fn execute(path: &str, crane: &dyn Crane) -> Result<(), &'static str> {
    let Ok(mut lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let mut schema_buffer = Vec::<String>::new();
    loop {
        let line = lines.next().ok_or("Unable to find end of schema")?;
        if line.is_empty() { break; }
        schema_buffer.insert(0, line);
    }

    let Some(mut crates_stock) = CratesStock::from(&schema_buffer) else {
        return Err("Unable to create crates stock");
    };
    let crane_moves = CraneMoves::from(lines).ok_or("Unable to parse crane instructions")?;

    crane.execute(&crane_moves, &mut crates_stock);
    let result = crates_stock.top_of_stacks();
    println!("Top of stacks: {}", result);

    Ok(())
}

fn main() -> Result<(), &'static str> {
    let stage = std::env::args().nth(1).expect("Expecting puzzle stage");
    let path = std::env::args().nth(2).expect("Expecting a file name");

    match stage.as_str() {
        "stage1" => execute(path.as_str(), &CrateMover9000::new()),
        "stage2" => execute(path.as_str(), &CrateMover9001::new()),
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