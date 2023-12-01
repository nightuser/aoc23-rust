use std::env;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

static NUMBERS: [(&str, char); 9] = [
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn read_lines<P: AsRef<Path>>(path: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let fh = File::open(path)?;
    let reader = io::BufReader::new(fh);
    Ok(reader.lines())
}

fn process_file<P, F>(path: P, digit_extractor: F) -> io::Result<i32>
where
    P: AsRef<Path>,
    F: Fn(String) -> String,
{
    let lines = read_lines(path)?;
    lines
        .map(|line| {
            line.map(|line| {
                let digits: String = digit_extractor(line);
                let first = digits.chars().next().unwrap_or('0');
                let last = digits.chars().last().unwrap_or('0');
                let number_str: String = [first, last].into_iter().collect();
                number_str.parse().expect("Must be a number")
            })
        })
        .try_fold(0, |acc, number| number.map(|x: i32| acc + x))
}

fn part1_extractor(line: String) -> String {
    line.chars().filter(char::is_ascii_digit).collect()
}

fn part2_extractor(line: String) -> String {
    line.char_indices()
        .map(|(pos, _)| &line[pos..])
        .filter_map(|suf| {
            NUMBERS
                .iter()
                .find_map(|(word, c)| suf.starts_with(*word).then(|| *c))
                .or_else(|| suf.chars().next().filter(char::is_ascii_digit))
        })
        .collect()
}

fn print_result<T, P, F>(part: &str, path: P, result: F)
where
    T: Display,
    P: AsRef<Path>,
    F: Fn(&Path) -> io::Result<T>,
{
    match result(path.as_ref()) {
        Ok(result) => println!("{} = {}", part, result),
        Err(err) => log::error!(
            "Error while processing `{}` for `{}`: {}",
            path.as_ref().display(),
            part,
            err
        ),
    }
}

fn main() {
    env_logger::init();
    let mut args = env::args();
    if args.len() != 2 {
        println!("Usage: day1 FILENAME");
        return;
    }
    let fname = PathBuf::from(args.nth(1).expect("Second argument exists"));
    print_result("part1", &fname, |path| process_file(path, part1_extractor));
    print_result("part2", &fname, |path| process_file(path, part2_extractor));
}
