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

#[test]
fn day1() {
    let input = read("input/day01/test.txt");
    assert_contains(
        &day1::part1(&input).unwrap(),
        "The maximum calorie count is 24000",
    );
    assert_contains(
        &day1::part2(&input).unwrap(),
        "The top 3 calorie counts sum up to 45000",
    );
}

#[test]
fn day2() {
    let input = read("input/day02/test.txt");
    assert_contains(&day2::part1(&input).unwrap(), "Total score is 15");
    assert_contains(&day2::part2(&input).unwrap(), "Total fixed score is 12");
}

#[test]
fn day3() {
    let input = read("input/day03/test.txt");
    assert_contains(
        &day3::part1(&input).unwrap(),
        "The priority sum of all duplicates is 157",
    );
    assert_contains(
        &day3::part2(&input).unwrap(),
        "The priority sum of all the badges is 70",
    );
}

#[test]
fn day4() {
    let input = read("input/day04/test.txt");
    assert_contains(&day4::part1(&input).unwrap(), "Found 2 matching pairs");
    assert_contains(&day4::part2(&input).unwrap(), "Found 4 overlapping pairs");
}

#[test]
fn day5() {
    let input = read("input/day05/test.txt");
    assert_contains(
        &day5::part1(&input).unwrap(),
        "After reordering the top crates are CMZ",
    );
    assert_contains(
        &day5::part2(&input).unwrap(),
        "After reordering the top crates are MCD",
    );
}

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
fn day7() {
    let input = read("input/day07/test.txt");
    assert_contains(
        &day7::part1(&input).unwrap(),
        "size sum of all directories whose size is under 100000 is 95437",
    );
    assert_contains(
        &day7::part2(&input).unwrap(),
        "Freed up 24933642 to prepare for update",
    );
}

#[test]
fn day8() {
    let input = read("input/day08/test.txt");
    assert_contains(
        &day8::part1(&input).unwrap(),
        "21 trees are visible from outside",
    );
    assert_contains(
        &day8::part2(&input).unwrap(),
        "Best cover score amongst the trees is 8",
    );
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
fn day11() {
    let input = read("input/day11/test.txt");
    assert_contains(
        &day11::part1(&input).unwrap(),
        "Monkey business level is 10605",
    );
    assert_contains(
        &day11::part2(&input).unwrap(),
        "Monkey business level is 2713310158",
    );
}

#[test]
fn day12() {
    let input = read("input/day12/test.txt");
    assert_contains(&day12::part1(&input).unwrap(), "Shortest path is 31");
    assert_contains(&day12::part2(&input).unwrap(), "Shortest path is 29");
}

#[test]
fn day13() {
    let input = read("input/day13/test.txt");
    assert_contains(
        &day13::part1(&input).unwrap(),
        "Sum of indices of ordered signals is 13",
    );
    assert_contains(&day13::part2(&input).unwrap(), "Decode key is 140");
}

#[test]
fn day14() {
    let input = read("input/day14/test.txt");
    assert_contains(
        &day14::part1(&input).unwrap(),
        "The cave is filled with 24 sand granules",
    );
    assert_contains(
        &day14::part2(&input).unwrap(),
        "The cave is filled with 93 sand granules",
    );
}

#[test]
fn day15() {
    let input = read("input/day15/test.txt");
    assert_contains(
        &day15::part1(&input, 10).unwrap(),
        "Row 10 has 26 positions that cannot contain a beacon",
    );
    assert_contains(
        &day15::part2(&input, 20).unwrap(),
        "Tuning frequency of the missing beacon is 56000011",
    );
}

#[test]
fn day16() {
    let input = read("input/day16/test.txt");
    assert_contains(
        &day16::part1(&input, 30).unwrap(),
        "Released 1651 pressure in 30 minutes",
    );
    assert_contains(
        &day16::part2(&input, 26).unwrap(),
        "Released 1707 pressure in 26 minutes when working with 1 elephant",
    );
}

#[test]
fn day17() {
    let input = read("input/day17/test.txt");
    assert_contains(
        &day17::part1(&input, 2022).unwrap(),
        "Tower height after 2022 rocks is 3068",
    );
    assert_contains(
        &day17::part2(&input, 1_000_000_000_000).unwrap(),
        "Tower height after 1000000000000 rocks is 1514285714288",
    );
}

#[test]
fn day18() {
    let input = read("input/day18/test.txt");
    assert_contains(&day18::part1(&input).unwrap(), "Total surface area is 64");
    assert_contains(
        &day18::part2(&input).unwrap(),
        "Exterior surface area is 58",
    );
}
