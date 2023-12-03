struct Game {
    game_id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

fn get_max(game_id: i32, line: &str) -> Game {
    let mut ret = Game {
        game_id,
        red: 0,
        green: 0,
        blue: 0,
    };

    let mut index = 0;

    while index < line.len() {
        let remaining = &line[index..];

        let next_comma = remaining.find(',').unwrap_or(remaining.len());
        let next_semi = remaining.find(';').unwrap_or(remaining.len());
        let next = next_comma.min(next_semi);
        let words = &remaining[0..next];

        let (num, color) = words.split_at(words.find(' ').unwrap());
        let amt = num.parse::<i32>().unwrap();
        let color = &color[1..];

        if "red" == color {
            ret.red = ret.red.max(amt);
        } else if "green" == color {
            ret.green = ret.green.max(amt);
        } else if "blue" == color {
            ret.blue = ret.blue.max(amt);
        }

        index += next + 2;
    }

    ret
}

fn do_part(input: &str, cb: fn(Game) -> i32) -> i32 {
    let total = input.lines().fold(0, |acc, line| {
        let start_id = line.find(' ').unwrap();
        let end_id = line.find(':').unwrap();
        let game_id = line[start_id..end_id].trim().parse::<i32>().unwrap();
        let index = end_id + 2;

        let game = get_max(game_id, &line[index..]);
        let inc = cb(game);

        acc + inc
    });

    total
}

fn do_part1(input: &str) -> i32 {
    do_part(input, |game| {
        if game.red > 12 || game.green > 13 || game.blue > 14 {
            return 0;
        }
        return game.game_id;
    })
}

fn do_part2(input: &str) -> i32 {
    do_part(input, |game| game.red * game.green * game.blue)
}

pub fn part1() {
    let input = include_str!("input.txt");
    let output = do_part1(input);

    println!("Day 2 part 1: {}", output);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let output = do_part2(input);

    println!("Day 2 part 2: {}", output);
}

#[test]
fn test_part1() {
    let input = include_str!("sample.txt");
    let output = do_part1(input);

    assert_eq!(output, 8);
}

#[test]
fn test_part2() {
    let input = include_str!("sample.txt");
    let output = do_part2(input);

    assert_eq!(output, 2286);
}
