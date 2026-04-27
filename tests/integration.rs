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

macro_rules! day_test {
    ($name:ident, $module:ident, $path:expr, $p1:expr, $p2:expr) => {
        #[test]
        fn $name() {
            let input = read($path);
            assert_contains(&$module::part1(&input).unwrap(), $p1);
            assert_contains(&$module::part2(&input).unwrap(), $p2);
        }
    };
}

macro_rules! day_test_extra {
    ($name:ident, $module:ident, $path:expr, $e1:expr, $p1:expr, $e2:expr, $p2:expr) => {
        #[test]
        fn $name() {
            let input = read($path);
            assert_contains(&$module::part1(&input, $e1).unwrap(), $p1);
            assert_contains(&$module::part2(&input, $e2).unwrap(), $p2);
        }
    };
}

day_test!(
    day1,
    day1,
    "input/day01/test.txt",
    "The maximum calorie count is 24000",
    "The top 3 calorie counts sum up to 45000"
);
day_test!(
    day2,
    day2,
    "input/day02/test.txt",
    "Total score is 15",
    "Total fixed score is 12"
);
day_test!(
    day3,
    day3,
    "input/day03/test.txt",
    "The priority sum of all duplicates is 157",
    "The priority sum of all the badges is 70"
);
day_test!(
    day4,
    day4,
    "input/day04/test.txt",
    "Found 2 matching pairs",
    "Found 4 overlapping pairs"
);
day_test!(
    day5,
    day5,
    "input/day05/test.txt",
    "After reordering the top crates are CMZ",
    "After reordering the top crates are MCD"
);
day_test!(
    day7,
    day7,
    "input/day07/test.txt",
    "size sum of all directories whose size is under 100000 is 95437",
    "Freed up 24933642 to prepare for update"
);
day_test!(
    day8,
    day8,
    "input/day08/test.txt",
    "21 trees are visible from outside",
    "Best cover score amongst the trees is 8"
);
day_test!(
    day11,
    day11,
    "input/day11/test.txt",
    "Monkey business level is 10605",
    "Monkey business level is 2713310158"
);
day_test!(
    day12,
    day12,
    "input/day12/test.txt",
    "Shortest path is 31",
    "Shortest path is 29"
);
day_test!(
    day13,
    day13,
    "input/day13/test.txt",
    "Sum of indices of ordered signals is 13",
    "Decode key is 140"
);
day_test!(
    day14,
    day14,
    "input/day14/test.txt",
    "The cave is filled with 24 sand granules",
    "The cave is filled with 93 sand granules"
);
day_test!(
    day18,
    day18,
    "input/day18/test.txt",
    "Total surface area is 64",
    "Exterior surface area is 58"
);
day_test!(
    day19,
    day19,
    "input/day19/test.txt",
    "Sum of quality levels is 33",
    "Product of geodes from first 3 blueprints is 3472"
);
day_test!(
    day20,
    day20,
    "input/day20/test.txt",
    "Grove coordinates sum is 3",
    "Grove coordinates sum is 1623178306"
);
day_test!(
    day21,
    day21,
    "input/day21/test.txt",
    "Root yells 152",
    "humn must yell 301"
);
day_test!(
    day22,
    day22,
    "input/day22/test.txt",
    "Final password is 6032",
    "Final password is 5031"
);
day_test!(
    day23,
    day23,
    "input/day23/test.txt",
    "Empty tiles in bounding rectangle: 110",
    "First round with no movement: 20"
);
day_test!(
    day24,
    day24,
    "input/day24/test.txt",
    "Fastest path to goal takes 18 minutes",
    "Round trip with snack takes 54 minutes"
);

day_test_extra!(
    day15,
    day15,
    "input/day15/test.txt",
    10,
    "Row 10 has 26 positions that cannot contain a beacon",
    20,
    "Tuning frequency of the missing beacon is 56000011"
);
day_test_extra!(
    day16,
    day16,
    "input/day16/test.txt",
    30,
    "Released 1651 pressure in 30 minutes",
    26,
    "Released 1707 pressure in 26 minutes when working with 1 elephant"
);
day_test_extra!(
    day17,
    day17,
    "input/day17/test.txt",
    2022,
    "Tower height after 2022 rocks is 3068",
    1_000_000_000_000,
    "Tower height after 1000000000000 rocks is 1514285714288"
);

#[test]
fn day6() {
    let input = read("input/day06/test.txt");
    let out1 = day6::part1(&input).unwrap();
    for (idx, expected) in [(0, 7), (1, 5), (2, 6), (3, 10), (4, 11)] {
        assert_contains(
            &out1,
            &format!(
                "distinct 4 character substring for buffer at position {idx} starts at {expected}"
            ),
        );
    }
    let out2 = day6::part2(&input).unwrap();
    for (idx, expected) in [(0, 19), (1, 23), (2, 23), (3, 29), (4, 26)] {
        assert_contains(
            &out2,
            &format!(
                "distinct 14 character substring for buffer at position {idx} starts at {expected}"
            ),
        );
    }
}

#[test]
fn day9() {
    let input1 = read("input/day09/test1.txt");
    let input2 = read("input/day09/test2.txt");
    assert_contains(&day9::part1(&input1).unwrap(), "The tail visited 13 places");
    assert_contains(&day9::part2(&input2).unwrap(), "The tail visited 36 places");
}

#[test]
fn day10() {
    let input = read("input/day10/test.txt");
    assert_contains(&day10::part1(&input).unwrap(), "Sum of cycles 13140");
    let out2 = day10::part2(&input).unwrap();
    for line in [
        "##..##..##..##..##..##..##..##..##..##..",
        "###...###...###...###...###...###...###.",
        "####....####....####....####....####....",
        "#####.....#####.....#####.....#####.....",
        "######......######......######......####",
        "#######.......#######.......#######.....",
    ] {
        assert_contains(&out2, line);
    }
}

#[test]
fn day25() {
    let input = read("input/day25/test.txt");
    assert_contains(
        &day25::part1(&input).unwrap(),
        "Fuel requirement in SNAFU is 2=-1=0",
    );
}
