use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


type Item = char;
const ITEM_LOWER_BASE: u32 = 97 - 1;
const ITEM_UPPER_BASE: u32 = 65 - 27;

pub fn item_value(item: Item) -> u32 {
    let codepoint: u32 = item.into();
    return if item.is_lowercase() {
        codepoint - ITEM_LOWER_BASE
    } else {
        codepoint - ITEM_UPPER_BASE
    }
}

pub struct Compartment {
    content: String,
}

impl Compartment {

    pub fn new(content: &str) -> Compartment {
        Compartment { content: String::from(content) }
    }

    pub fn find_common_item(&self, other: &Compartment) -> Option<Item> {
        for c in self.content.chars() {
            if other.content.contains(c) {
                return Some(c);
            }
        }
        None
    }
}

pub struct Rucksack {
    compartment: [Compartment; 2],
}

impl Rucksack {

    pub fn from(content: &String) -> Option<Rucksack> {
        let items_count = content.chars().count();
        if !(items_count % 2) == 0 {
            return None;
        };
        let half_items_count = items_count / 2;
        let rucksack = Rucksack {
            compartment: [
                Compartment::new(&content[0..half_items_count]),
                Compartment::new(&content[half_items_count..]),
            ],
        };
        Some(rucksack)
    }

    pub fn find_common_item(&self) -> Option<Item> {
        self.compartment[0].find_common_item(&self.compartment[1])
    }
}

pub struct RucksackGroup {
    rucksacks: Vec<Rucksack>
}

impl RucksackGroup {

    pub fn from(lines: impl Iterator<Item = String>) -> Option<RucksackGroup> {
        let mut rucksacks = Vec::new();
        for line in lines {
            let Some(rucksack) = Rucksack::from(&line) else { return None };
            rucksacks.push(rucksack)
        }
        let instance = RucksackGroup { rucksacks };
        Some(instance)
    }

    pub fn common_items_sum(&self) -> u32 {
        let mut result = 0;
        for rucksack in self.rucksacks.iter() {
            if let Some(item) = rucksack.find_common_item() {
                let item_val = item_value(item);
                // println!("{item} = {item_val}");
                result += item_val;
            }
        }
        result
    }
}


fn stage1(path: &str) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let Some(rucksack_group) = RucksackGroup::from(lines) else {
        return Err("Unable to load rucksack")
    };
    let sum = rucksack_group.common_items_sum();
    println!("All common items in rucksacks values sum {sum}");

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