use crate::Block;
use cgmath::Vector2;

#[derive(Clone, Debug, Copy)]
pub enum BlockType {
    Solid,
    Breakable,
}

#[derive(Debug, Clone)]
pub struct Level {
    pub level: u8,
    pub blocks: Vec<Vec<BlockType>>,
    pub width: u8,
    pub height: u8,
}

pub fn parse_file_to_level(filename: &str) -> Result<Level, String> {
    let file_contents = std::fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Could not read file {}", filename));

    let mut blocks: Vec<Vec<BlockType>> = vec![];
    let mut level: Option<u8> = None;
    let mut width: Option<u8> = None;
    let mut height: Option<u8> = None;
    for line in file_contents.lines() {
        let mut row: Vec<BlockType> = vec![];
        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() {
            continue;
        }

        if words[0] == "level" {
            level = Some(words[1].parse().expect("Failed to parse level information"));
            continue;
        }

        if words[0] == "dimensions" {
            let mut dimensions = words[1].split_terminator('x');
            width = Some(dimensions.next().unwrap().parse().unwrap());
            height = Some(dimensions.next().unwrap().parse().unwrap());
            continue;
        }

        for word in words {
            match word {
                "s" => {
                    row.push(BlockType::Solid);
                }
                "b" => {
                    row.push(BlockType::Breakable);
                }
                _ => {
                    panic!("Failed to parse token {}", word)
                }
            }
        }
        blocks.push(row);
    }

    if let Some(level) = level {
        Ok(Level {
            level,
            blocks,
            width: width.expect("Failed to parse width"),
            height: height.expect("Failed to parse height"),
        })
    } else {
        Err(String::from("Failed to parse level information"))
    }
}
