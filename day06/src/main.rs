use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use itertools::Itertools;

pub fn is_marker_valid(marker: &[char]) -> bool {
    for couple in marker.iter().combinations(2) {
        if couple[0] == couple[1] {
            return false;
        }
    }
    return true
}

pub fn data_start_index(raw_buffer: &str, start_marker_size: usize) -> Option<usize> {
    let mut marker = vec![raw_buffer.chars().next()?; start_marker_size];

    for (i, c) in raw_buffer.chars().enumerate() {
        marker.rotate_left(1);
        marker[start_marker_size - 1] = c;
        if is_marker_valid(&marker) {
            return Some(i + 1);
        }
    }

    None
}

fn execute(path: &str, packet_marker_size: usize, message_marker_size: Option<usize>) -> Result<(), &'static str> {
    let Ok(lines) = read_lines(path) else {
        return Err("Unable to read the file")
    };

    for (line_index, line) in lines.enumerate() {
        let packet_start_index = match data_start_index(line.as_str(), packet_marker_size) {
            Some(start_index) => {
                println!("Line #{} packet start index: {}", line_index + 1, start_index);
                start_index
            },
            None => {
                println!("Line #{} has no packet start index", line_index + 1);
                return Ok(());
            },
        };
        let Some(msg_start_idx) = message_marker_size else { return Ok(()); };
        let packet = &line[packet_start_index..];
        match data_start_index(packet, msg_start_idx) {
            Some(start_index) => {
                println!("Line #{} message start index: {}", line_index + 1, packet_start_index + start_index);
                start_index
            },
            None => {
                println!("Line #{} has no message start index", line_index + 1);
                return Ok(());
            },
        };
    }

    Ok(())
}

fn main() -> Result<(), &'static str> {
    let stage = std::env::args().nth(1).expect("Expecting puzzle stage");
    let path = std::env::args().nth(2).expect("Expecting a file name");

    match stage.as_str() {
        "stage1" => execute(path.as_str(), 4, None),
        "stage2" => execute(path.as_str(), 4, Some(14)),
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