use anyhow::Result;
use std::{io::BufRead, path::Path};

type Range = (usize, usize);

pub fn get_memmap_lines(pid: usize, elf_path: &Path) -> Result<(Option<Range>, Vec<Range>)> {
    let maps = std::fs::File::open(format!("/proc/{pid}/maps"))?;
    let maps = std::io::BufReader::new(maps);
    
    let mut ranges = Vec::new();
    let mut write_range = None;
    for line in maps.lines() {
        let line = line?;
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        // if line[line.len() - 1] == elf_path.to_str().unwrap() && line[1].contains('w') {
        if line[line.len() - 1] == elf_path.to_str().unwrap() {
            // println!("{line:#?}");
            let range = get_range(line[0])?;
            ranges.push(range);
            if line[1].contains('w') {
                write_range = Some(range);
            }
        }
    }

    Ok((write_range, ranges))
}

fn get_range(line: &str) -> Result<Range> {
    let addrs: Vec<_> = line.split('-').collect();
    let start = usize::from_str_radix(addrs[0], 16)?;
    let end = usize::from_str_radix(addrs[1], 16)?;
    Ok((start, end))
}