use std::process::Command;

fn run(args: &[&str]) -> String {
    let bin = env!("CARGO_BIN_EXE_aoc22");
    let output = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run binary");
    assert!(
        output.status.success(),
        "binary exited with {:?}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("non-utf8 stdout")
}

fn assert_contains(stdout: &str, needle: &str) {
    assert!(
        stdout.contains(needle),
        "expected stdout to contain {:?}, got:\n{}",
        needle,
        stdout
    );
}

#[test]
fn day1() {
    let out = run(&["day1", "task1", "input/day01/test.txt"]);
    assert_contains(&out, "The maximum calorie count is 24000");
    let out = run(&["day1", "task2", "input/day01/test.txt"]);
    assert_contains(&out, "The top 3 calorie counts sum up to 45000");
}

#[test]
fn day2() {
    let out = run(&["day2", "task1", "input/day02/test.txt"]);
    assert_contains(&out, "Total score is 15");
    let out = run(&["day2", "task2", "input/day02/test.txt"]);
    assert_contains(&out, "Total fixed score is 12");
}

#[test]
fn day3() {
    let out = run(&["day3", "task1", "input/day03/test.txt"]);
    assert_contains(&out, "The priority sum of all duplicates is 157");
    let out = run(&["day3", "task2", "input/day03/test.txt"]);
    assert_contains(&out, "The priority sum of all the badges is 70");
}

#[test]
fn day4() {
    let out = run(&["day4", "task1", "input/day04/test.txt"]);
    assert_contains(&out, "Found 2 matching pairs");
    let out = run(&["day4", "task2", "input/day04/test.txt"]);
    assert_contains(&out, "Found 4 overlapping pairs");
}

#[test]
fn day5() {
    let out = run(&["day5", "task1", "input/day05/test.txt"]);
    assert_contains(&out, "After reordering the top crates are CMZ");
    let out = run(&["day5", "task2", "input/day05/test.txt"]);
    assert_contains(&out, "After reordering the top crates are MCD");
}

#[test]
fn day6() {
    let out = run(&["day6", "task1", "input/day06/test.txt"]);
    assert_contains(
        &out,
        "distinct 4 character substring for buffer at position 0 starts at 7",
    );
    assert_contains(
        &out,
        "distinct 4 character substring for buffer at position 1 starts at 5",
    );
    assert_contains(
        &out,
        "distinct 4 character substring for buffer at position 2 starts at 6",
    );
    assert_contains(
        &out,
        "distinct 4 character substring for buffer at position 3 starts at 10",
    );
    assert_contains(
        &out,
        "distinct 4 character substring for buffer at position 4 starts at 11",
    );
    let out = run(&["day6", "task2", "input/day06/test.txt"]);
    assert_contains(
        &out,
        "distinct 14 character substring for buffer at position 0 starts at 19",
    );
    assert_contains(
        &out,
        "distinct 14 character substring for buffer at position 1 starts at 23",
    );
    assert_contains(
        &out,
        "distinct 14 character substring for buffer at position 2 starts at 23",
    );
    assert_contains(
        &out,
        "distinct 14 character substring for buffer at position 3 starts at 29",
    );
    assert_contains(
        &out,
        "distinct 14 character substring for buffer at position 4 starts at 26",
    );
}

#[test]
fn day7() {
    let out = run(&["day7", "task1", "input/day07/test.txt"]);
    assert_contains(
        &out,
        "size sum of all directories whose size is under 100000 is 95437",
    );
    let out = run(&["day7", "task2", "input/day07/test.txt"]);
    assert_contains(&out, "Freed up 24933642 to prepare for update");
}

#[test]
fn day8() {
    let out = run(&["day8", "task1", "input/day08/test.txt"]);
    assert_contains(&out, "21 trees are visible from outside");
    let out = run(&["day8", "task2", "input/day08/test.txt"]);
    assert_contains(&out, "Best cover score amongst the trees is 8");
}

#[test]
fn day9() {
    let out = run(&["day9", "task1", "input/day09/test1.txt"]);
    assert_contains(&out, "The tail visited 13 places");
    let out = run(&["day9", "task2", "input/day09/test2.txt"]);
    assert_contains(&out, "The tail visited 36 places");
}

#[test]
fn day10() {
    let out = run(&["day10", "task1", "input/day10/test.txt"]);
    assert_contains(&out, "Sum of cycles 13140");
    let out = run(&["day10", "task2", "input/day10/test.txt"]);
    assert_contains(&out, "##..##..##..##..##..##..##..##..##..##..");
    assert_contains(&out, "###...###...###...###...###...###...###.");
    assert_contains(&out, "####....####....####....####....####....");
    assert_contains(&out, "#####.....#####.....#####.....#####.....");
    assert_contains(&out, "######......######......######......####");
    assert_contains(&out, "#######.......#######.......#######.....");
}

#[test]
fn day11() {
    let out = run(&["day11", "task1", "input/day11/test.txt"]);
    assert_contains(&out, "Monkey business level is 10605");
    let out = run(&["day11", "task2", "input/day11/test.txt"]);
    assert_contains(&out, "Monkey business level is 2713310158");
}

#[test]
fn day12() {
    let out = run(&["day12", "task1", "input/day12/test.txt"]);
    assert_contains(&out, "Shortest path is 31");
    let out = run(&["day12", "task2", "input/day12/test.txt"]);
    assert_contains(&out, "Shortest path is 29");
}

#[test]
fn day13() {
    let out = run(&["day13", "task1", "input/day13/test.txt"]);
    assert_contains(&out, "Sum of indices of ordered signals is 13");
    let out = run(&["day13", "task2", "input/day13/test.txt"]);
    assert_contains(&out, "Decode key is 140");
}

#[test]
fn day14() {
    let out = run(&["day14", "task1", "input/day14/test.txt"]);
    assert_contains(&out, "The cave is filled with 24 sand granules");
    let out = run(&["day14", "task2", "input/day14/test.txt"]);
    assert_contains(&out, "The cave is filled with 93 sand granules");
}

#[test]
fn day15() {
    let out = run(&["day15", "task1", "input/day15/test.txt", "10"]);
    assert_contains(&out, "Row 10 has 26 positions that cannot contain a beacon");
    let out = run(&["day15", "task2", "input/day15/test.txt", "20"]);
    assert_contains(&out, "Tuning frequency of the missing beacon is 56000011");
}

#[test]
fn day16() {
    let out = run(&["day16", "task1", "input/day16/test.txt", "30"]);
    assert_contains(&out, "Released 1651 pressure in 30 minutes");
    let out = run(&["day16", "task2", "input/day16/test.txt", "26"]);
    assert_contains(
        &out,
        "Released 1707 pressure in 26 minutes when working with 1 elephant",
    );
}
