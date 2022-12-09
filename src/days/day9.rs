use crate::utils::file;
use log::{debug, info};

#[derive(PartialEq)]
#[derive(PartialOrd)]
#[derive(Ord)]
#[derive(Eq)]
struct Position {
    x: i32,
    y: i32,
}

fn update_rest_of_rope(rope: &mut Vec<Position>) {
    if rope.len() < 3 {
        return;
    }
    for i in 2..rope.len() {
        if rope[i-1] == rope[i] {
            continue;
        }
        if rope[i-1].x == rope[i].x {
            if rope[i-1].y - rope[i].y > 1 {
                rope[i].y += 1;
            } else if rope[i-1].y - rope[i].y < -1 {
                rope[i].y -= 1;
            }
        } else if rope[i-1].y == rope[i].y {
            if rope[i-1].x - rope[i].x > 1 {
                rope[i].x += 1;
            } else if rope[i-1].x - rope[i].x < -1 {
                rope[i].x -= 1;
            }
        } else {
            if rope[i-1].y - rope[i].y > 1 && rope[i-1].x - rope[i].x > 1 {
                rope[i].y += 1;
                rope[i].x += 1;
            } else if rope[i-1].y - rope[i].y < -1 && rope[i-1].x - rope[i].x < -1 {
                rope[i].y -= 1;
                rope[i].x -= 1;
            } else if rope[i-1].y - rope[i].y > 1 && rope[i-1].x - rope[i].x < -1 {
                rope[i].y += 1;
                rope[i].x -= 1;
            } else if rope[i-1].y - rope[i].y < -1 && rope[i-1].x - rope[i].x > 1 {
                rope[i].y -= 1;
                rope[i].x += 1;
            } else if rope[i-1].y - rope[i].y > 1 {
                rope[i].y += 1;
                if rope[i-1].x != rope[i].x {
                    rope[i].x = rope[i-1].x;
                }
            } else if rope[i-1].y - rope[i].y < -1 {
                rope[i].y -= 1;
                if rope[i-1].x != rope[i].x {
                    rope[i].x = rope[i-1].x;
                }
            } else if rope[i-1].x - rope[i].x > 1 {
                rope[i].x += 1;
                if rope[i-1].y != rope[i].y {
                    rope[i].y = rope[i-1].y;
                }
            } else if rope[i-1].x - rope[i].x < -1 {
                rope[i].x -= 1;
                if rope[i-1].y != rope[i].y {
                    rope[i].y = rope[i-1].y;
                }
            }
        }
    }
}

fn simulate_tail_movements(movements: &Vec<file::Direction>, curr_movement: usize, rope: &mut Vec<Position>) -> Vec<Position> {
    if curr_movement == movements.len() {
        return Vec::new();
    }

    let movement = movements[curr_movement];
    match movement {
        file::Direction::Down => {
            rope[0].y -= 1;
            if rope[1].y - rope[0].y > 1 {
                rope[1].y -= 1;
                if rope[1].x != rope[0].x {
                    rope[1].x = rope[0].x;
                }
            }
            update_rest_of_rope(rope);
        },
        file::Direction::Up => {
            rope[0].y += 1;
            if rope[0].y - rope[1].y > 1 {
                rope[1].y += 1;
                if rope[1].x != rope[0].x {
                    rope[1].x = rope[0].x;
                }
            }
            update_rest_of_rope(rope);
        },
        file::Direction::Left => {
            rope[0].x -= 1;
            if rope[1].x - rope[0].x > 1 {
                rope[1].x -= 1;
                if rope[1].y != rope[0].y {
                    rope[1].y = rope[0].y;
                }
            }
            update_rest_of_rope(rope);
        },
        file::Direction::Right => {
            rope[0].x += 1;
            if rope[0].x - rope[1].x > 1 {
                rope[1].x += 1;
                if rope[1].y != rope[0].y {
                    rope[1].y = rope[0].y;
                }
            }
            update_rest_of_rope(rope);
        },
    }
    let mut visited = vec![Position{x: rope[rope.len() -1].x, y: rope[rope.len() -1].y}];
    visited.extend(simulate_tail_movements(movements, curr_movement + 1, rope));
    visited
}

pub fn task1(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let movements = file::to_movements(lines);
        debug!("Found {} rope movements", movements.len());
        let mut rope = vec![Position{x:0, y:0}, Position{x:0, y:0}];
        let mut pos = simulate_tail_movements(&movements, 0, &mut rope);
        pos.sort();
        pos.dedup();
        info!("The tail visited {} places", pos.len());
    }
}

pub fn task2(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let movements = file::to_movements(lines);
        debug!("Found {} rope movements", movements.len());
        let mut rope = vec![Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}, Position{x: 0, y: 0}];
        let mut pos = simulate_tail_movements(&movements, 0, &mut rope);
        pos.sort();
        pos.dedup();
        info!("The tail visited {} places", pos.len());
    } 
}
