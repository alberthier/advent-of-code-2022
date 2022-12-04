use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;
use once_cell::sync::Lazy;
use regex::Regex;

type SectionID = i32;
type SectionRange = Range<SectionID>;

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

    pub fn fully_overlapping_sections_sum(&self) -> usize {
        self.groups.iter().filter_map(|p| {
            let fully_overlapping =
                p.first.start >= p.second.start && p.first.end <= p.second.end ||
                p.second.start >= p.first.start && p.second.end <= p.first.end;
            fully_overlapping.then_some(())
        }).count()
    }
}

fn stage1(path: &str) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    let colony = ElvesColony::from(lines).ok_or("Unable to read the input")?;
    let sum = colony.fully_overlapping_sections_sum();

    println!("The number of groups with fully overlapping sections is {sum}");

    Ok(())
}


fn stage2(path: &str) -> Result<(), &'static str> {
    let Ok(_lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

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