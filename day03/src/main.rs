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

#[derive(Clone)]
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

#[derive(Clone)]
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

    pub fn iter_all_items(&self) -> impl Iterator<Item = Item> + '_ {
        self.compartment[0].content.chars().chain(self.compartment[1].content.chars())
    }
}

pub struct ElfGroup {
    rucksacks: Vec<Rucksack>
}

impl ElfGroup {

    pub fn new(rucksacks: Vec<Rucksack>) -> ElfGroup {
        ElfGroup { rucksacks }
    }

    pub fn find_common_item(&self) -> Option<Item> {
        for item in self.rucksacks[0].iter_all_items() {
            let is_common = self.rucksacks.iter().map(|rucksack| {
                rucksack.iter_all_items()
                    .find(|other_item| *other_item == item)
                    .is_some()
            }).all(|x| x);
            if is_common { return Some(item) }
        };
        None
    }
}

pub struct ElvesColony {
    elf_groups: Vec<ElfGroup>,
}

impl ElvesColony {

    pub fn from(lines: impl Iterator<Item = String>, group_size: usize) -> Option<ElvesColony> {
        let mut instance = ElvesColony {
            elf_groups: Vec::new(),
        };
        let mut rucksacks = Vec::new();
        for line in lines {
            let Some(rucksack) = Rucksack::from(&line) else { return None };
            rucksacks.push(rucksack)
        }

        for group_rucksacks in rucksacks.chunks(group_size).into_iter() {
            let elf_group = ElfGroup::new(group_rucksacks.to_owned().to_vec());
            instance.elf_groups.push(elf_group);
        }
        println!("count {}", instance.elf_groups.len());

        Some(instance)
    }

    pub fn common_items_sum(&self) -> u32 {
        let mut result = 0;
        for elf_group in self.elf_groups.iter() {
            for rucksack in elf_group.rucksacks.iter() {
                if let Some(item) = rucksack.find_common_item() {
                    let item_val = item_value(item);
                    // println!("{item} = {item_val}");
                    result += item_val;
                }
            }
        }
        result
    }

    pub fn badges_sum(&self) -> Option<u32> {
        self.elf_groups.iter().try_fold(0, |acc, elf_group| {
            let Some(item) = elf_group.find_common_item() else { return None };
            let value = item_value(item);
            Some(acc + value)
        })
    }
}


fn stage1(path: &str) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let Some(rucksack_group) = ElvesColony::from(lines, 1) else {
        return Err("Unable to load rucksack")
    };
    let sum = rucksack_group.common_items_sum();
    println!("All common items in rucksacks values sum {sum}");

    Ok(())
}


fn stage2(path: &str) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let Some(rucksack_group) = ElvesColony::from(lines, 3) else {
        return Err("Unable to load rucksack")
    };
    let Some(sum) = rucksack_group.badges_sum() else {
        return Err("Unable to find a common item in all groups");
    };
    println!("All groups badges sum {sum}");

    Ok(())
}


fn main() -> Result<(), &'static str> {
    let stage = std::env::args().nth(1).expect("Expecting puzzle stage");
    let path = std::env::args().nth(2).expect("Expecting a file name");

    match stage.as_str() {
        "stage1" => stage1(path.as_str()),
        "stage2" => stage2(path.as_str()),
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