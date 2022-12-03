use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        Result,
        Lines,
    },
    path::Path,
    option::Option,
    str::FromStr,
};

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn lines_to_numbers<N>(lines: Lines<BufReader<File>>) -> Vec<Option<N>>
where N: FromStr {
    lines.map(|line| {
        match line {
            Err(_) => None,
            Ok(itm) => {
                match itm.parse::<N>() {
                    Err(_) => None,
                    Ok(num) => Some(num),
                }
            },
        }
    }).collect()
}

pub fn to_groups(lines: Lines<BufReader<File>>, size: usize) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    groups.push(Vec::new());
    lines.fold(groups, |mut groups, line| {
        if let Ok(itm) = line {
            if let Some(last) = groups.last() {
                if last.len() >= size {
                    groups.push(Vec::new());
                }
            }
            if let Some(last) = groups.last_mut() {
                last.push(itm);
            }
        }
        groups
    }) 
}
