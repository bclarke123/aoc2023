fn get_sum(input: &str) {
    let sum = input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .fold(0, |acc, cl| {
            acc + cl.first().unwrap() * 10 + cl.last().unwrap()
        });

    println!("{:?}", sum);
}

pub fn part1() {
    let input = include_str!("input.txt");
    get_sum(input);
}

pub fn get_sum2(input: &str) -> u32 {
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut idx = 0;

    let chars = input.chars().collect::<Vec<_>>();
    let mut numbers = vec![];

    loop {
        if idx >= input.len() {
            break;
        }

        let next = chars[idx];
        if next.is_digit(10) {
            numbers.push(next.to_digit(10).unwrap());
            idx += 1;
            continue;
        }

        let sub = &input[idx..];

        for i in 0..words.len() {
            if sub.starts_with(&words[i]) {
                numbers.push(i as u32);
                break;
            }
        }

        idx += 1;
    }

    let first = numbers[0];
    let last = numbers[numbers.len() - 1];

    return first * 10 + last;
}

pub fn part2() {
    let str = include_str!("input.txt");
    let mut total = 0;

    for line in str.lines() {
        total += get_sum2(&line);
    }

    println!("{}", total);
}
