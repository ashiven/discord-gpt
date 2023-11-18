use std::fs;
use std::io::prelude::*;
use std::io::{BufReader, Result};

const FILE_PATH: &str = "C:/Users/janni/OneDrive/Dokumente/Projects/Python/scripts/test.py";

#[derive(Debug)]
struct Docstrings {
    count: usize,
    docstrings: Vec<Docstring>,
}

impl Docstrings {
    fn new() -> Self {
        Self {
            docstrings: vec![],
            count: 0,
        }
    }
}

#[derive(Debug)]
struct Docstring {
    start: usize,
    end: usize,
    content: String,
}

fn main() -> Result<()> {
    let file = fs::File::open(FILE_PATH)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let mut line_number = 0;

    let mut docstrings = Docstrings::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes_read) => {
                if line.contains("\"\"\"") || line.contains("\'\'\'") {
                    let mut docstring = Docstring {
                        start: line_number,
                        end: 0,
                        content: line.clone(),
                    };

                    let occurences = line.match_indices("\"\"\"").count();
                    let occurences2 = line.match_indices("\'\'\'").count();

                    if occurences == 2 || occurences2 == 2 {
                        docstring.end = line_number;
                        docstrings.docstrings.push(docstring);
                        docstrings.count += 1;
                        line.clear();
                        line_number += 1;
                        continue;
                    }

                    loop {
                        line.clear();
                        reader.read_line(&mut line)?;
                        line_number += 1;

                        docstring.content.push_str(&line);

                        if line.contains("\"\"\"") || line.contains("\'\'\'") {
                            docstring.end = line_number;
                            docstrings.docstrings.push(docstring);
                            docstrings.count += 1;

                            line.clear();
                            break;
                        }
                    }
                } else if bytes_read == 0 {
                    break;
                } else {
                    line.clear();
                }
                line_number += 1;
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }

    println!("{:#?}", docstrings);

    Ok(())
}
