use anyhow::Result;
use std::{io::BufRead, path::Path};

pub fn get_line_from_memmap(pid: usize, elf_path: &Path) -> Result<Option<(usize, usize)>> {
    let maps = std::fs::File::open(format!("/proc/{pid}/maps"))?;
    let maps = std::io::BufReader::new(maps);
    
    let mut ranges = Vec::new();
    for line in maps.lines() {
        let line = line?;
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        // if line[line.len() - 1] == elf_path.to_str().unwrap() && line[1].contains('w') {
        if line[line.len() - 1] == elf_path.to_str().unwrap() {
            // println!("{line:#?}");
            let range = get_range(line[0])?;
            ranges.push(range);
        }
    }

    let range = ranges.iter().min_by_key(|v| v.0).copied();
    Ok(range)
}

fn get_range(line: &str) -> Result<(usize, usize)> {
    let addrs: Vec<_> = line.split('-').collect();
    let start = usize::from_str_radix(addrs[0], 16)?;
    let end = usize::from_str_radix(addrs[1], 16)?;
    Ok((start, end))
}