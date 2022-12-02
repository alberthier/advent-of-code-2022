use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Elf {
    food: Vec<u32>,
}

impl Elf {

    pub fn new() -> Elf {
        Elf {
            food: Vec::new()
        }
    }

    pub fn add_calories(&mut self, calories: u32) {
        self.food.push(calories);
    }

    pub fn total_calories(&self) -> u32 {
        self.food.iter().sum()
    }
}

pub struct ElfGroup {
    elves: Vec<Elf>,
}

impl ElfGroup<> {

    pub fn new() -> ElfGroup {
        ElfGroup {
            elves: Vec::new()
        }
    }

    pub fn load(&mut self, it: impl Iterator<Item = String>) {
        let mut elf = Elf::new();
        for line in it {
            match line.parse() {
                Ok(calories) => {
                    elf.add_calories(calories);
                }
                Err(_error) => {
                    self.elves.push(elf);
                    elf = Elf::new();
                }
            }
        }
    }

    pub fn elves_count(&self) -> usize {
        self.elves.len()
    }

    pub fn get_max_calories(&self) -> Option<u32> {
        let Some(elf) = self.elves.iter().max_by(|x, y|
            x.total_calories().cmp(&y.total_calories())
        ) else {
            return None;
        };
        Some(elf.total_calories())
    }
}

fn stage1(path: &str) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let mut elf_group = ElfGroup::new();
    elf_group.load(lines);

    let Some(max_calories) = elf_group.get_max_calories() else {
        return Err("No elves in the list");
    };
    println!("The input has {} elves", elf_group.elves_count());
    println!("The elf with the carrying the most calories has {} calories", max_calories);

    Ok(())
}

fn main() -> Result<(), &'static str> {
    let stage = std::env::args().nth(1).expect("Expecting puzzle stage");
    let path = std::env::args().nth(2).expect("Expecting a file name");

    match stage.as_str() {
        "stage1" => stage1(path.as_str()),
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