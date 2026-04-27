// Real-input regressions. Skipped by default — task.txt files may be private
// per-user inputs and the puzzle answers below are the author's solutions.
//
//   cargo test --release -- --ignored
//
// If you fork this repo with your own task.txt files, update the expected
// strings to match your puzzle outputs (or delete this file entirely).

use aoc22::days::*;

fn read(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("failed to read {path}: {e}"))
}

fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "expected output to contain {needle:?}, got:\n{haystack}"
    );
}

macro_rules! day_real {
    ($name:ident, $module:ident, $path:expr, $p1:expr, $p2:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let input = read($path);
            assert_contains(&$module::part1(&input).unwrap(), $p1);
            assert_contains(&$module::part2(&input).unwrap(), $p2);
        }
    };
}

macro_rules! day_real_extra {
    ($name:ident, $module:ident, $path:expr, $e1:expr, $p1:expr, $e2:expr, $p2:expr) => {
        #[test]
        #[ignore]
        fn $name() {
            let input = read($path);
            assert_contains(&$module::part1(&input, $e1).unwrap(), $p1);
            assert_contains(&$module::part2(&input, $e2).unwrap(), $p2);
        }
    };
}

day_real!(
    day1,
    day1,
    "input/day01/task.txt",
    "The maximum calorie count is 67633",
    "The top 3 calorie counts sum up to 199628"
);
day_real!(
    day2,
    day2,
    "input/day02/task.txt",
    "Total score is 12586",
    "Total fixed score is 13193"
);
day_real!(
    day3,
    day3,
    "input/day03/task.txt",
    "The priority sum of all duplicates is 8039",
    "The priority sum of all the badges is 2510"
);
day_real!(
    day4,
    day4,
    "input/day04/task.txt",
    "Found 536 matching pairs",
    "Found 845 overlapping pairs"
);
day_real!(
    day5,
    day5,
    "input/day05/task.txt",
    "After reordering the top crates are TPGVQPFDH",
    "After reordering the top crates are DMRDFRHHH"
);
day_real!(
    day7,
    day7,
    "input/day07/task.txt",
    "size sum of all directories whose size is under 100000 is 1307902",
    "Freed up 7068748 to prepare for update"
);
day_real!(
    day8,
    day8,
    "input/day08/task.txt",
    "1647 trees are visible from outside",
    "Best cover score amongst the trees is 392080"
);
day_real!(
    day10,
    day10,
    "input/day10/task.txt",
    "Sum of cycles 15680",
    "CRT Output:"
);
day_real!(
    day11,
    day11,
    "input/day11/task.txt",
    "Monkey business level is 99840",
    "Monkey business level is 20683044837"
);
day_real!(
    day12,
    day12,
    "input/day12/task.txt",
    "Shortest path is 425",
    "Shortest path is 418"
);
day_real!(
    day13,
    day13,
    "input/day13/task.txt",
    "Sum of indices of ordered signals is 6240",
    "Decode key is 23142"
);
day_real!(
    day14,
    day14,
    "input/day14/task.txt",
    "The cave is filled with 779 sand granules",
    "The cave is filled with 27426 sand granules"
);
day_real!(
    day18,
    day18,
    "input/day18/task.txt",
    "Total surface area is 4282",
    "Exterior surface area is 2452"
);
day_real!(
    day19,
    day19,
    "input/day19/task.txt",
    "Sum of quality levels is 1427",
    "Product of geodes from first 3 blueprints is 4400"
);
day_real!(
    day20,
    day20,
    "input/day20/task.txt",
    "Grove coordinates sum is 7278",
    "Grove coordinates sum is 14375678667089"
);
day_real!(
    day21,
    day21,
    "input/day21/task.txt",
    "Root yells 256997859093114",
    "humn must yell 3952288690726"
);
day_real!(
    day22,
    day22,
    "input/day22/task.txt",
    "Final password is 136054",
    "Final password is 122153"
);
day_real!(
    day23,
    day23,
    "input/day23/task.txt",
    "Empty tiles in bounding rectangle: 3966",
    "First round with no movement: 933"
);
day_real!(
    day24,
    day24,
    "input/day24/task.txt",
    "Fastest path to goal takes 240 minutes",
    "Round trip with snack takes 717 minutes"
);

day_real_extra!(
    day15,
    day15,
    "input/day15/task.txt",
    2_000_000,
    "Row 2000000 has 6425133 positions that cannot contain a beacon",
    4_000_000,
    "Tuning frequency of the missing beacon is 10996191429555"
);
day_real_extra!(
    day16,
    day16,
    "input/day16/task.txt",
    30,
    "Released 2253 pressure in 30 minutes",
    26,
    "Released 2838 pressure in 26 minutes when working with 1 elephant"
);
day_real_extra!(
    day17,
    day17,
    "input/day17/task.txt",
    2022,
    "Tower height after 2022 rocks is 3109",
    1_000_000_000_000,
    "Tower height after 1000000000000 rocks is 1541449275365"
);

#[test]
#[ignore]
fn day6() {
    let input = read("input/day06/task.txt");
    assert_contains(
        &day6::part1(&input).unwrap(),
        "4 character substring for buffer at position 0 starts at 1655",
    );
    assert_contains(
        &day6::part2(&input).unwrap(),
        "14 character substring for buffer at position 0 starts at 2665",
    );
}

#[test]
#[ignore]
fn day9() {
    let input = read("input/day09/task.txt");
    assert_contains(&day9::part1(&input).unwrap(), "The tail visited 6175 places");
    assert_contains(&day9::part2(&input).unwrap(), "The tail visited 2578 places");
}

#[test]
#[ignore]
fn day25() {
    let input = read("input/day25/task.txt");
    assert_contains(
        &day25::part1(&input).unwrap(),
        "Fuel requirement in SNAFU is 2=0=02-0----2-=02-10",
    );
}
