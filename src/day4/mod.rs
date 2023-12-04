#[derive(Debug, Clone)]
struct Card {
    matches: usize,
    score: i32,
}

impl Card {
    fn new(wins: Vec<i32>, have: Vec<i32>) -> Self {
        let matches = have.iter().filter(|x| wins.contains(x)).count();
        let score = (2_u32 as f32).powi((matches as i32) - 1) as i32;

        Self { matches, score }
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

    Card::new(wins, have)
}

fn get_copies(cards: &Vec<Card>, index: usize, score: usize) -> usize {
    let card = &cards[index];
    let matches = card.matches;
    let mut ret = score + 1;

    for i in 0..matches {
        let idx = index + i + 1;
        ret += get_copies(cards, idx, score);
    }

    ret
}

fn do_part1(input: &str) -> i32 {
    input.lines().map(|l| parse_card(l).score).sum()
}

fn do_part2(input: &str) -> usize {
    let cards = input.lines().map(|l| parse_card(l)).collect::<Vec<_>>();

    let mut ret = 0;
    for i in 0..cards.len() {
        ret += get_copies(&cards, i, 0);
    }

    ret
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 4 part 1: {}", output);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let output = do_part2(input);
    println!("Day 4 part 2: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 13);
}

#[test]
fn test_part2() {
    let input = include_str!("sample.txt");
    let output = do_part2(input);
    assert_eq!(output, 30);
}
