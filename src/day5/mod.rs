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
}

fn parse_header(input: &str) -> Vec<u64> {
    input
        .split(' ')
        .skip(1)
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<_>>()
}

fn parse_body(lines: &[&str]) -> Vec<Mapping> {
    let mut str_mappings = vec![];

    let mut remaining = lines; // Remove header and blank line
    loop {
        if remaining.is_empty() {
            break;
        }

        let end = match remaining.iter().position(|s| s.is_empty()) {
            Some(x) => x,
            None => remaining.len(),
        };

        let next = &remaining[0..end];

        str_mappings.push(next);

        remaining = &remaining[(end + 1).min(remaining.len())..];
    }

    str_mappings
        .iter()
        .map(|m| Mapping::parse(m))
        .collect::<Vec<_>>()
}

fn split_range(range: Vec<u64>, mapping: &Mapping) -> Vec<Vec<u64>> {
    let lo = range[0];
    let hi = range[1];

    for map_range in &mapping.ranges {
        let m_dst = map_range[0];
        let m_lo = map_range[1];
        let m_hi = m_lo + map_range[2];

        // if our range is completely outside this range
        if lo >= m_hi || hi < m_lo {
            continue;
        }

        // if our range is completely within this range
        if lo >= m_lo && hi < m_hi {
            return vec![vec![lo - m_lo + m_dst, hi - m_lo + m_dst]];
        }

        // if our range starts below this range but ends within it
        if lo < m_lo && hi > m_lo && hi < m_hi {
            let mut ret = split_range(vec![lo, m_lo], mapping);
            ret.append(&mut split_range(vec![m_lo, hi], mapping));

            return ret;
        }

        // if our range starts within this range and ends outside of it
        if lo >= m_lo && lo < m_hi && hi >= m_hi {
            let mut ret = split_range(vec![lo, m_hi - 1], mapping);
            ret.append(&mut split_range(vec![m_hi, hi], mapping));

            return ret;
        }
    }

    // println!("No mapping for range {:?}", range);
    vec![range]
}

fn filter_range(input: Vec<Vec<u64>>, mappings: Vec<Mapping>) -> Vec<Vec<u64>> {
    let mut map_name = "seed";
    let mut updated = input;

    loop {
        if map_name == "location" {
            break;
        }

        let mapping = mappings.iter().find(|m| m.from == map_name).unwrap();
        updated = updated
            .iter()
            .flat_map(|x| split_range(x.to_vec(), mapping))
            .collect::<Vec<_>>();

        map_name = &mapping.to;
    }

    updated
}

fn do_part(input: &str, cb: fn(Vec<u64>) -> Vec<Vec<u64>>) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();
    let header = parse_header(lines.first().unwrap());
    let mappings = parse_body(&lines[2..]);

    let ranges = cb(header);
    let filtered = filter_range(ranges, mappings);
    *filtered.iter().flatten().min().unwrap()
}

fn do_part1(input: &str) -> u64 {
    do_part(input, |header| {
        header.iter().map(|x| vec![*x, *x]).collect::<Vec<_>>()
    })
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);
    println!("Day 5 Part 1: {}", output);
}

fn do_part2(input: &str) -> u64 {
    do_part(input, |header| {
        header
            .chunks(2)
            .map(|c| vec![c[0], c[0] + c[1]])
            .collect::<Vec<_>>()
    })
}

pub fn part2() {
    let input = include_str!("input.txt");
    let output = do_part2(input);
    println!("Day 5 Part 2: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);
    assert_eq!(output, 35);
}

#[test]
fn test_part2() {
    let input = include_str!("sample.txt");
    let output = do_part2(input);
    assert_eq!(output, 46);
}
