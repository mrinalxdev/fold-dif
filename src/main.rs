#![allow(unused)]

use file_size::fit_4;
use std::{error::Error, fs, path::PathBuf};
use walkdir::WalkDir;

const DIR: &str = "./";
const TOP_NUMS: usize = 5;

struct Entry {
    path: PathBuf,
    size: u64,
}

fn main() {
    match exec() {
        Ok(_) => (),
        Err(ex) => {
            println!("ERROR - {}", ex);
        }
    }
}

fn exec() -> Result<(), Box<dyn Error>> {
    let mut total_size: u64 = 0;
    let mut total_numbers: u32 = 0;
    let mut tops: Vec<Entry> = Vec::with_capacity(TOP_NUMS + 1);
    let mut min_of_tops = 0;

    for entry in WalkDir::new(DIR).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && !entry.path_is_symlink() {
            total_numbers += 1;
            let size = entry.metadata()?.len();
            total_size += size;

            if min_of_tops < size {
                tops.push(Entry {
                    path: entry.path().to_path_buf(),
                    size,
                });

                tops.sort_by(|a, b| b.size.cmp(&a.size));
                if tops.len() > TOP_NUMS {
                    tops.pop();
                }
                min_of_tops = tops.last().map(|e| e.size).unwrap_or(0);
            }
        }
        // println!("{}", entry.path().display());
    }

    println!(
        "Number of file {}, total size : {}",
        total_numbers,
        fit_4(total_size)
    );
    println!("Top {} biggest files", tops.len());
    for Entry { size, path } in tops.iter() {
        println!("{:<4} - {}", fit_4(*size), path.to_string_lossy());
    }
    Ok(())
}
