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
    ops::Range,
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

pub fn to_range_touple<N>(lines: Lines<BufReader<File>>) -> Vec<(Range<N>, Range<N>)>
where N: FromStr + Copy {
    lines.fold(Vec::new(), |mut acc, line| {
        if let Ok(itm) = line {
            let ranges: Vec<&str> = itm.split(",").collect();
            let first_range: Vec<&str> = ranges[0].split("-").collect();
            let mut res: Vec<N> = Vec::new();
            if let Ok(idx) = first_range[0].parse::<N>() {
                res.push(idx);
            }
            if let Ok(idx) = first_range[1].parse::<N>() {
                res.push(idx);
            }
            let second_range: Vec<&str> = ranges[1].split("-").collect();
            if let Ok(idx) = second_range[0].parse::<N>() {
                res.push(idx);
            }
            if let Ok(idx) = second_range[1].parse::<N>() {
                res.push(idx);
            }
            acc.push(((res[0]..res[1]), (res[2]..res[3])));
        }
        acc
    })
}
