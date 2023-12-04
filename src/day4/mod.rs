#[derive(Debug, Clone)]
struct Card {
    wins: Vec<i32>,
    have: Vec<i32>,
}

impl Card {
    fn score(&self) -> i32 {
        let ret = self.have.iter().filter(|x| self.wins.contains(x)).count() as i32;
        (2_u32 as f32).powi(ret - 1) as i32
    }
}

fn parse_list(input: &str) -> Vec<i32> {
    input
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<_>>()
}

fn parse_card(line: &str) -> Card {
    let colon = line.find(": ").unwrap() + 2;
    let numbers = &line[colon..];
    let pipe = numbers.find('|').unwrap();
    let (wins, have) = numbers.split_at(pipe);

    let wins = parse_list(&wins[0..wins.len() - 1]);
    let have = parse_list(&have[2..]);

    Card { wins, have }
}

fn do_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|l| parse_card(l))
        .map(|c| c.score())
        .sum()
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 4 part 1: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 13);
}
