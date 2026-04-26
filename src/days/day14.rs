use crate::utils::file;
use anyhow::Result;
use std::collections::HashSet;

fn find_lowest_point(walls: &[Vec<file::Position<u32>>]) -> u32 {
    walls.iter().fold(0, |lowest_point, wall| {
        wall.iter().fold(lowest_point, |lowest_point, point| {
            if point.y > lowest_point {
                point.y
            } else {
                lowest_point
            }
        })
    })
}

fn build_cave(walls: &[Vec<file::Position<u32>>]) -> HashSet<file::Position<u32>> {
    walls.iter().fold(HashSet::new(), |mut cave, wall| {
        for idx in 1..wall.len() {
            if wall[idx - 1].x == wall[idx].x {
                let mut range = wall[idx - 1].y..wall[idx].y + 1;
                if wall[idx - 1].y > wall[idx].y {
                    range = wall[idx].y..wall[idx - 1].y + 1;
                }
                for y in range {
                    cave.insert(file::Position { x: wall[idx].x, y });
                }
            } else {
                let mut range = wall[idx - 1].x..wall[idx].x + 1;
                if wall[idx - 1].x > wall[idx].x {
                    range = wall[idx].x..wall[idx - 1].x + 1;
                }
                for x in range {
                    cave.insert(file::Position { x, y: wall[idx].y });
                }
            }
        }
        cave
    })
}

fn simulate_sand(walls: &[Vec<file::Position<u32>>], cave_floor: Option<u32>) -> u32 {
    let mut cave = build_cave(walls);
    let mut lowest_point = find_lowest_point(walls);
    if let Some(floor) = cave_floor {
        lowest_point = floor;
    }
    let mut total_granules = 0;
    loop {
        let mut sand = file::Position { x: 500, y: 0 };
        if cave.contains(&sand) {
            break;
        }
        loop {
            sand.y += 1;
            if (cave_floor.is_some() && sand.y == lowest_point) || cave.contains(&sand) {
                sand.x -= 1;
                if (cave_floor.is_some() && sand.y == lowest_point) || cave.contains(&sand) {
                    sand.x += 2;
                    if (cave_floor.is_some() && sand.y == lowest_point) || cave.contains(&sand) {
                        sand.x -= 1;
                        sand.y -= 1;
                        break;
                    }
                }
            }
            if sand.y == lowest_point {
                break;
            }
        }
        if cave_floor.is_none() && sand.y == lowest_point {
            break;
        }
        cave.insert(sand);
        total_granules += 1;
    }
    total_granules
}

pub fn part1(input: &str) -> Result<String> {
    let walls = file::to_walls::<u32, _>(file::lines_of(input))?;
    let total = simulate_sand(&walls, None);
    Ok(format!("The cave is filled with {total} sand granules"))
}

pub fn part2(input: &str) -> Result<String> {
    let walls = file::to_walls::<u32, _>(file::lines_of(input))?;
    let lowest_point = find_lowest_point(&walls);
    let total = simulate_sand(&walls, Some(lowest_point + 2));
    Ok(format!("The cave is filled with {total} sand granules"))
}
