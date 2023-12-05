#[derive(Debug, Clone)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<Vec<u64>>,
}

impl Mapping {
    fn parse_ranges(input: &[&str]) -> Vec<Vec<u64>> {
        input
            .iter()
            .map(|l| {
                l.split(' ')
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn parse(input: &[&str]) -> Self {
        let header = input.first().unwrap();
        let space = header.find(' ').unwrap();
        let (name, _) = header.split_at(space);
        let words = name.split('-').collect::<Vec<_>>();

        Self {
            from: words[0].to_string(),
            to: words[2].to_string(),
            ranges: Self::parse_ranges(&input[1..]),
        }
    }
    fn get_value(&self, input: u64) -> u64 {
        let mapping = self
            .ranges
            .iter()
            .find(|r| input >= r[1] && input < r[1] + r[2]);
        match mapping {
            Some(r) => r[0] + (input - r[1]),
            None => input,
        }
    }
}

fn parse_header(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .skip(1)
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>()
}

fn do_part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let header = parse_header(lines.first().unwrap());
    let mut str_mappings = vec![];

    let mut remaining = &lines[2..]; // Remove header and blank line
    loop {
        if remaining.len() == 0 {
            break;
        }

        let end = match remaining.iter().position(|s| *s == "") {
            Some(x) => x,
            None => remaining.len(),
        };

        let next = &remaining[0..end];

        str_mappings.push(next);

        remaining = &remaining[(end + 1).min(remaining.len())..];
    }

    let mappings = str_mappings
        .iter()
        .map(|m| Mapping::parse(m))
        .collect::<Vec<_>>();

    header
        .iter()
        .map(|x| {
            let mut to = "seed";
            let mut val = *x;
            loop {
                let mapping = mappings.iter().find(|m| m.from == to).unwrap();
                let new_val = mapping.get_value(val);
                val = new_val;
                to = &mapping.to;

                if to == "location" {
                    break;
                }
            }
            val
        })
        .min()
        .unwrap()
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 5 Part 1: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 35);
}
