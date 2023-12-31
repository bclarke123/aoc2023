#[derive(Debug)]
struct Part {
    num: i32,
    len: usize,
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Match {
    row: usize,
    col: usize,
}

fn parse_parts(lines: &[&str]) -> Vec<Part> {
    let mut parts = vec![];
    for (i, line) in lines.iter().enumerate() {
        let mut index = 0;

        loop {
            if index > line.len() {
                break;
            }

            let remaining = &line[index..];
            let next = remaining.find(|c: char| c.is_ascii_digit());

            if next.is_none() {
                break;
            }

            let next = next.unwrap();
            let next_end = next
                + remaining[next..]
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(remaining.len() - next);
            let num = &remaining[next..next_end];
            let row = i;
            let col = index + next;

            parts.push(Part {
                num: num.parse::<i32>().unwrap(),
                len: num.len(),
                row,
                col,
            });

            index += next_end;
        }
    }
    parts
}

fn search_near(p: &Part, lines: &Vec<&str>, check: fn(char) -> bool) -> Option<Match> {
    let start_col = p.col.max(1) - 1;
    let end_col = (p.col + p.len + 1).min(lines[0].len() - 1);

    if p.row > 0 {
        let chars = &lines[p.row - 1][start_col..end_col];

        if let Some(i) = chars.find(check) {
            return Some(Match {
                row: p.row - 1,
                col: start_col + i,
            });
        }
    }

    if p.row < lines.len() - 1 {
        let chars = &lines[p.row + 1][start_col..end_col];

        if let Some(i) = chars.find(check) {
            return Some(Match {
                row: p.row + 1,
                col: start_col + i,
            });
        }
    }

    let mut row = lines[p.row][start_col..end_col].chars();

    if p.col > 0 {
        let first = row.next().unwrap();
        if check(first) {
            return Some(Match {
                row: p.row,
                col: start_col,
            });
        }
    }

    if p.col < lines[0].len() - 1 {
        let last = row.last().unwrap();
        if check(last) {
            return Some(Match {
                row: p.row,
                col: start_col + p.len + 1,
            });
        }
    }

    None
}

fn do_part1(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let parts = parse_parts(&lines);

    fn is_symbol(c: char) -> bool {
        !(c.is_ascii_digit() || c == '.')
    }

    let sum = parts
        .iter()
        .filter(|&p| search_near(p, &lines, is_symbol).is_some())
        .fold(0, |acc, p| acc + p.num);

    sum
}

fn do_part2(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let parts = parse_parts(&lines);

    fn is_gear(c: char) -> bool {
        c == '*'
    }

    let gear_parts = parts
        .iter()
        .filter_map(|p| search_near(p, &lines, is_gear).map(|m| (m, p.num)))
        .collect::<Vec<_>>();

    let mut matches = vec![];
    for x in &gear_parts {
        if !matches.contains(&x.0) {
            matches.push(x.0);
        }
    }

    matches
        .iter()
        .map(|m| {
            gear_parts
                .iter()
                .filter(|x| x.0 == *m)
                .map(|x| x.1)
                .collect::<Vec<_>>()
        })
        .filter(|x| x.len() == 2)
        .map(|x| x.iter().product::<i32>())
        .sum()
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 3 part 1: {}", output);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let output = do_part2(input);
    println!("Day 3 part 2: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 4361);
}

#[test]
fn test_part2() {
    let input = include_str!("sample.txt");
    let output = do_part2(input);
    assert_eq!(output, 467835);
}
