fn parse_row(row: &str) -> Vec<u64> {
    row.split(' ')
        .skip(1)
        .filter_map(|x| x.parse::<u64>().ok())
        .collect()
}

fn parse_row_p2(row: &str) -> u64 {
    row.split(' ')
        .skip(1)
        .map(|x| x.trim())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

fn get_races(times: Vec<u64>, records: Vec<u64>) -> Vec<(u64, u64)> {
    times
        .iter()
        .enumerate()
        .map(|(i, r)| (*r, records[i]))
        .collect::<Vec<_>>()
}

fn ways_to_win(race: (u64, u64)) -> u64 {
    let mut ret = 0;
    for i in 0..race.0 {
        let remaining = race.0 - i;
        let speed = i;

        if speed * remaining > race.1 {
            ret += 1;
        }
    }
    ret
}

fn do_part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let times = parse_row(lines[0]);
    let records = parse_row(lines[1]);
    let races = get_races(times, records);

    let ret = races.iter().map(|r| ways_to_win(*r)).product();

    ret
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 6 part 1: {}", output);
}

fn do_part2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let time = parse_row_p2(lines[0]);
    let record = parse_row_p2(lines[1]);

    ways_to_win((time, record))
}

pub fn part2() {
    let input = include_str!("input.txt");
    let output = do_part2(input);
    println!("Day 6 part 2: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 288);
}

#[test]
fn test_part2() {
    let input = include_str!("sample.txt");
    let output = do_part2(input);
    assert_eq!(output, 71503);
}
