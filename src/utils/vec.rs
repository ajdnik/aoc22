use std::collections::HashSet;
use std::ops::Add;

pub fn find_duplicate_chars(strings: &[String]) -> String {
    let Some((first, rest)) = strings.split_first() else {
        return String::new();
    };
    let mut common: HashSet<char> = first.chars().collect();
    for s in rest {
        let chars: HashSet<char> = s.chars().collect();
        common.retain(|c| chars.contains(c));
    }
    common.into_iter().collect()
}

pub fn find_first_distinct_substring(buffer: String, substring_size: usize) -> u32 {
    let chars: Vec<char> = buffer.chars().collect();
    for i in (substring_size - 1)..chars.len() {
        let window = &chars[i + 1 - substring_size..=i];
        let set: HashSet<char> = window.iter().copied().collect();
        if set.len() == substring_size {
            return (i + 1) as u32;
        }
    }
    chars.len() as u32
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
