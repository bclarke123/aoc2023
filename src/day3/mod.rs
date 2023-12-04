#[derive(Debug)]
struct Part {
    num: i32,
    len: usize,
    row: usize,
    col: usize,
}

fn parse_parts(lines: &Vec<&str>) -> Vec<Part> {
    let mut parts = vec![];
    for i in 0..lines.len() {
        let line = lines[i];
        let mut index = 0;

        loop {
            if index > line.len() {
                break;
            }

            let remaining = &line[index..];
            let next = remaining.find(|c: char| c.is_digit(10));

            if next.is_none() {
                break;
            }

            let next = next.unwrap();
            let next_end = next
                + remaining[next..]
                    .find(|c: char| !c.is_digit(10))
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

fn do_part1(input: &str) -> i32 {
    let lines = input.lines().collect::<Vec<_>>();
    let parts = parse_parts(&lines);

    fn is_symbol(c: char) -> bool {
        !(c.is_digit(10) || c == '.')
    }

    let sum = parts
        .iter()
        .filter(|&p| {
            let start_col = p.col.max(1) - 1;
            let end_col = (p.col + p.len + 1).min(lines[0].len() - 1);

            if p.row > 0 {
                let chars = &lines[p.row - 1][start_col..end_col];
                if chars.find(|c: char| is_symbol(c)).is_some() {
                    return true;
                }
            }

            if p.row < lines.len() - 1 {
                let chars = &lines[p.row + 1][start_col..end_col];
                if chars.find(|c: char| is_symbol(c)).is_some() {
                    return true;
                }
            }

            let mut row = lines[p.row][start_col..end_col].chars();

            if p.col > 0 {
                let first = row.next().unwrap();
                if is_symbol(first) {
                    return true;
                }
            }

            if p.col < lines[0].len() - 1 {
                let last = row.last().unwrap();
                if is_symbol(last) {
                    return true;
                }
            }

            false
        })
        .fold(0, |acc, p| acc + p.num);

    sum
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 3 part 1: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 4361);
}
