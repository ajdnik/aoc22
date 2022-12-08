use crate::utils::{
    file,
    vec,
};
use log::{debug, info};

fn is_visible(x: usize, y: usize, matrix: &Vec<Vec<u8>>) -> u8 {
    if y == 0 || y == matrix.len() - 1 {
        return 1;
    }
    if x == 0 || x == matrix[y].len() - 1 {
        return 1;
    }
    let smaller_x = (0..x).rev().fold(true, |mut smaller, idx| {
        if matrix[y][idx] >= matrix[y][x] {
            smaller = false;
        }
        smaller
    });
    if smaller_x {
        return 1;
    }
    let smaller_y = (0..y).rev().fold(true, |mut smaller, idx| {
        if matrix[idx][x] >= matrix[y][x] {
            smaller = false;
        }
        smaller
    });
    if smaller_y {
        return 1;
    }
    0
}

fn calc_cover(x: usize, y: usize, matrix: &Vec<Vec<u8>>) -> usize {
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
    for z in x+1..matrix[y].len() {
        right += 1;
        if matrix[y][z] >= matrix[y][x] {
            break;
        }

    }
    let mut down = 0;
    for z in y+1..matrix.len() {
        down += 1;
        if matrix[z][x] >= matrix[y][x] {
            break;
        }

    }
    up * left * right * down
}

pub fn task1(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let forest = file::to_matrix::<u8>(lines);
        assert!(forest.len() > 0);
        debug!("Loaded {} x {} forest", forest.len(), forest[0].len());
        let visible_top_left = vec::matrix_to_mask(&forest, is_visible);
        let reversed_forest = vec::matrix_rotate180(&forest);
        let visible_bottom_right = vec::matrix_to_mask(&reversed_forest, is_visible);
        let all_visible = vec::matrix_merge(&visible_top_left, &vec::matrix_rotate180(&visible_bottom_right));
        let total_seen = all_visible.iter().fold(0, |mut sum, row| {
            sum += row.iter().fold(0, |sum, val| {
                if *val > 0 {
                    return sum + 1;
                }
                sum
            });
            sum
        });
        info!("{} trees are visible from outside", total_seen);
    }
}

pub fn task2(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let forest = file::to_matrix::<u8>(lines);
        assert!(forest.len() > 0);
        debug!("Loaded {} x {} forest", forest.len(), forest[0].len());
        let cover_scores = vec::matrix_to_mask(&forest, calc_cover);
        let max_score = cover_scores.iter().fold(0_usize, |max, row| {
            let row_max = row.iter().max().unwrap_or(&0);
            if *row_max > max {
                return *row_max;
            }
            max
        });
        info!("Best cover score amongst the trees is {}", max_score);
    } 
}
