use crate::utils::{file, vec};
use anyhow::{ensure, Result};

fn is_visible(x: usize, y: usize, matrix: &[Vec<u8>]) -> u8 {
    if y == 0 || y == matrix.len() - 1 {
        return 1;
    }
    if x == 0 || x == matrix[y].len() - 1 {
        return 1;
    }
    let smaller_x = (0..x).rev().all(|idx| matrix[y][idx] < matrix[y][x]);
    if smaller_x {
        return 1;
    }
    let smaller_y = (0..y).rev().all(|idx| matrix[idx][x] < matrix[y][x]);
    if smaller_y {
        return 1;
    }
    0
}

fn calc_cover(x: usize, y: usize, matrix: &[Vec<u8>]) -> usize {
    if y == 0 || y == matrix.len() - 1 {
        return 0;
    }
    if x == 0 || x == matrix[y].len() - 1 {
        return 0;
    }

    let mut up = 0;
    for z in (0..y).rev() {
        up += 1;
        if matrix[z][x] >= matrix[y][x] {
            break;
        }
    }
    let mut left = 0;
    for z in (0..x).rev() {
        left += 1;
        if matrix[y][z] >= matrix[y][x] {
            break;
        }
    }
    let mut right = 0;
    for z in x + 1..matrix[y].len() {
        right += 1;
        if matrix[y][z] >= matrix[y][x] {
            break;
        }
    }
    let mut down = 0;
    for z in y + 1..matrix.len() {
        down += 1;
        if matrix[z][x] >= matrix[y][x] {
            break;
        }
    }
    up * left * right * down
}

pub fn part1(input: &str) -> Result<String> {
    let forest = file::to_matrix::<u8, _>(file::lines_of(input));
    ensure!(!forest.is_empty(), "empty forest");
    let visible_top_left = vec::matrix_to_mask(&forest, is_visible);
    let reversed = vec::matrix_rotate180(&forest);
    let visible_bottom_right = vec::matrix_to_mask(&reversed, is_visible);
    let all_visible = vec::matrix_merge(
        &visible_top_left,
        &vec::matrix_rotate180(&visible_bottom_right),
    );
    let total_seen: u32 = all_visible
        .iter()
        .flat_map(|row| row.iter())
        .map(|val| if *val > 0 { 1 } else { 0 })
        .sum();
    Ok(format!("{} trees are visible from outside", total_seen))
}

pub fn part2(input: &str) -> Result<String> {
    let forest = file::to_matrix::<u8, _>(file::lines_of(input));
    ensure!(!forest.is_empty(), "empty forest");
    let cover_scores = vec::matrix_to_mask(&forest, calc_cover);
    let max_score = cover_scores
        .iter()
        .flat_map(|row| row.iter())
        .copied()
        .max()
        .unwrap_or(0);
    Ok(format!(
        "Best cover score amongst the trees is {}",
        max_score
    ))
}
