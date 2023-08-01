use anyhow::Result;
use std::{io::BufRead, path::PathBuf};

pub fn get_line_from_memmap(pid: usize, elf_path: &PathBuf) -> Result<Option<(usize, usize)>> {
    let maps = std::fs::File::open(format!("/proc/{}/maps", pid))?;
    let maps = std::io::BufReader::new(maps);
    for line in maps.lines() {
        let line = line?;
        let line: Vec<_> = line.split_ascii_whitespace().collect();
        if line[line.len()-1] == elf_path.to_str().unwrap() && line[1].contains('w') {
            println!("{:#?}", line);
            let addrs: Vec<_> = line[0].split('-').collect();
            let start = usize::from_str_radix(addrs[0], 16)?;
            let end = usize::from_str_radix(addrs[1], 16)?;
            return Ok(Some((start, end)))
        }
    }
    Ok(None)
}