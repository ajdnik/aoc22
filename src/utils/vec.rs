use std::ops::Add;

pub fn find_duplicate_chars(strings: &[String]) -> String {
    let mut duplicates = String::from("");
    if let Some((first, rest)) = strings.split_first() {
        for char in first.chars() {
            let mut count_found = 0;
            for itm in rest {
                if itm.find(char).is_some() {
                    count_found += 1;
                }
            }
            if count_found == rest.len() && duplicates.find(char).is_none() {
                duplicates += &char.to_string();
            }
        }
    }
    duplicates
}

pub fn find_first_distinct_substring(buffer: String, substring_size: usize) -> u32 {
    let (start, rest) = buffer.split_at(substring_size - 1);
    let mut start_string = String::from(start);
    let mut position = substring_size as u32 - 1;
    for char in rest.chars() {
        position += 1;
        start_string.push(char);
        let dups = find_duplicate_chars(&[start_string.to_owned(), start_string.to_owned()]);
        if dups.len() == substring_size {
            break;
        }
        let mut chars = start_string.chars();
        chars.next();
        start_string = chars.as_str().to_string();
    }
    position
}

pub fn matrix_to_mask<N, T>(
    matrix: &[Vec<N>],
    handler: fn(usize, usize, &[Vec<N>]) -> T,
) -> Vec<Vec<T>> {
    let mut mask = Vec::new();
    for y in 0..matrix.len() {
        let mut row = Vec::new();
        for x in 0..matrix[y].len() {
            row.push(handler(x, y, matrix));
        }
        mask.push(row);
    }
    mask
}

pub fn matrix_rotate180<N>(matrix: &[Vec<N>]) -> Vec<Vec<N>>
where
    N: Copy,
{
    let mut rotated = Vec::new();
    for y in (0..matrix.len()).rev() {
        let mut row = Vec::new();
        for x in (0..matrix[y].len()).rev() {
            row.push(matrix[y][x]);
        }
        rotated.push(row);
    }
    rotated
}

pub fn matrix_merge<N>(mat_a: &[Vec<N>], mat_b: &[Vec<N>]) -> Vec<Vec<N>>
where
    N: Copy + Add<Output = N>,
{
    let mut merged = Vec::new();
    assert!(mat_a.len() == mat_b.len());
    for y in 0..mat_a.len() {
        assert!(mat_a[y].len() == mat_b[y].len());
        let mut row = Vec::new();
        for x in 0..mat_a[y].len() {
            row.push(mat_a[y][x] + mat_b[y][x]);
        }
        merged.push(row);
    }
    merged
}
