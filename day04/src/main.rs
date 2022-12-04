use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;
use once_cell::sync::Lazy;
use regex::Regex;


type SectionID = i32;
type SectionRange = Range<SectionID>;


pub fn are_section_ranges_fully_overlapping(lhs: &SectionRange, rhs: &SectionRange) -> bool {
    lhs.start >= rhs.start && lhs.end <= rhs.end ||
    rhs.start >= lhs.start && rhs.end <= lhs.end
}

pub fn are_section_ranges_partially_overlapping(lhs: &SectionRange, rhs: &SectionRange) -> bool {
    are_section_ranges_fully_overlapping(lhs, rhs) ||
    rhs.contains(&lhs.start) || rhs.contains(&lhs.end) ||
    lhs.contains(&rhs.start) || lhs.contains(&rhs.end)
}

pub struct ElvesPair {
    first: SectionRange,
    second: SectionRange,
}

impl ElvesPair {

    pub fn from(line: String) -> Option<ElvesPair> {
        static SECTIONS_LINE_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
            Regex::new(r"(?P<s1start>\d+)-(?P<s1end>\d+),(?P<s2start>\d+)-(?P<s2end>\d+)")
                .expect("Unable to create the sections line regex")
        });

        let Some(captures) = SECTIONS_LINE_REGEX.captures(&line) else { return None };
        let s1start = captures.name("s1start")?.as_str().parse().ok()?;
        let s1end = captures.name("s1end")?.as_str().parse().ok()?;
        let s2start = captures.name("s2start")?.as_str().parse().ok()?;
        let s2end = captures.name("s2end")?.as_str().parse().ok()?;

        let first = s1start..s1end;
        let second = s2start..s2end;

        Some(ElvesPair { first, second })
    }

    pub fn is_fully_overlapping(&self) -> bool {
        are_section_ranges_fully_overlapping(&self.first, &self.second)
    }

    pub fn is_partially_overlapping(&self) -> bool {
        are_section_ranges_partially_overlapping(&self.first, &self.second)
    }
}


struct ElvesColony {
    groups: Vec<ElvesPair>
}

impl ElvesColony {

    pub fn from(lines: impl Iterator<Item = String>) -> Option<ElvesColony> {
        let mut groups = Vec::new();
        for line in lines {
            groups.push(ElvesPair::from(line)?);
        }
        Some(ElvesColony { groups })
    }

    pub fn fully_overlapping_sections_count(&self) -> usize {
        self.groups
            .iter()
            .filter_map(|p| p.is_fully_overlapping().then_some(()))
            .count()
    }

    pub fn partially_overlapping_sections_count(&self) -> usize {
        self.groups
            .iter()
            .filter_map(|p| p.is_partially_overlapping().then_some(()))
            .count()
    }
}


fn execute(path: &str, fully_overlapping: bool) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let colony = ElvesColony::from(lines).ok_or("Unable to read the input")?;
    if fully_overlapping {
        let count = colony.fully_overlapping_sections_count();
        println!("The number of groups with fully overlapping sections is {count}");
    } else {
        let count = colony.partially_overlapping_sections_count();
        println!("The number of groups with partially overlapping sections is {count}");
    }

    Ok(())
}

fn main() -> Result<(), &'static str> {
    let stage = std::env::args().nth(1).expect("Expecting puzzle stage");
    let path = std::env::args().nth(2).expect("Expecting a file name");

    match stage.as_str() {
        "stage1" => execute(path.as_str(), true),
        "stage2" => execute(path.as_str(), false),
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