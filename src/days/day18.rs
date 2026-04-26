use crate::utils::file;
use anyhow::Result;
use std::collections::{HashSet, VecDeque};

type Voxel = (i32, i32, i32);

const NEIGHBORS: [(i32, i32, i32); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn neighbors_of((x, y, z): Voxel) -> impl Iterator<Item = Voxel> {
    NEIGHBORS
        .iter()
        .map(move |&(dx, dy, dz)| (x + dx, y + dy, z + dz))
}

fn surface_area(cubes: &HashSet<Voxel>) -> u32 {
    cubes
        .iter()
        .map(|&c| neighbors_of(c).filter(|n| !cubes.contains(n)).count() as u32)
        .sum()
}

fn bounds(cubes: &HashSet<Voxel>) -> (Voxel, Voxel) {
    let mut min = (i32::MAX, i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN, i32::MIN);
    for &(x, y, z) in cubes {
        min.0 = min.0.min(x);
        min.1 = min.1.min(y);
        min.2 = min.2.min(z);
        max.0 = max.0.max(x);
        max.1 = max.1.max(y);
        max.2 = max.2.max(z);
    }
    (
        (min.0 - 1, min.1 - 1, min.2 - 1),
        (max.0 + 1, max.1 + 1, max.2 + 1),
    )
}

fn exterior_surface_area(cubes: &HashSet<Voxel>) -> u32 {
    let (lo, hi) = bounds(cubes);
    let mut exterior: HashSet<Voxel> = HashSet::new();
    let mut queue: VecDeque<Voxel> = VecDeque::new();
    queue.push_back(lo);
    exterior.insert(lo);
    let mut faces = 0u32;
    while let Some(cur) = queue.pop_front() {
        for n @ (x, y, z) in neighbors_of(cur) {
            if x < lo.0 || y < lo.1 || z < lo.2 || x > hi.0 || y > hi.1 || z > hi.2 {
                continue;
            }
            if cubes.contains(&n) {
                faces += 1;
                continue;
            }
            if exterior.insert(n) {
                queue.push_back(n);
            }
        }
    }
    faces
}

pub fn part1(input: &str) -> Result<String> {
    let voxels = file::to_voxels::<i32, _>(file::lines_of(input))?;
    let cubes: HashSet<Voxel> = voxels.into_iter().collect();
    let area = surface_area(&cubes);
    Ok(format!("Total surface area is {area}"))
}

pub fn part2(input: &str) -> Result<String> {
    let voxels = file::to_voxels::<i32, _>(file::lines_of(input))?;
    let cubes: HashSet<Voxel> = voxels.into_iter().collect();
    let area = exterior_surface_area(&cubes);
    Ok(format!("Exterior surface area is {area}"))
}
