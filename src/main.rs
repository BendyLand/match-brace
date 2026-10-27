use std::str::Lines;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        let starting_line = get_starting_line_number(&args);
        let file_contents = get_file_contents(&args);
        let lines = file_contents.lines();
        let trimmed = trim_to_starting_line(lines, starting_line);
        let verbose = check_for_flags(&args);
        begin_walk(trimmed, starting_line as i16, verbose);
    }
    else {
        print_usage();
    }
}

fn print_usage() {
    eprintln!("Usage: match-brace <file> <starting-line-number>")
}

fn check_for_flags(args: &[String]) -> bool {
    let flags = {
        args
            .iter()
            .find(|arg| arg.as_str() == "-v" || arg.as_str() == "--verbose")
    };
    return match flags {
        Some(_) => true,
        None => false,
    };
}

fn begin_walk(lines: Lines, starting_line: i16, verbose: bool) {
    let mut count: i16 = 0;
    let mut line_count = starting_line;
    for line in lines {
        let open_count = line.matches("{").collect::<Vec<&str>>().len();
        count += open_count as i16;
        let closed_count = line.matches("}").collect::<Vec<&str>>().len();
        count -= closed_count as i16;
        if verbose { println!("{}: {}", &line_count, &line) }
        if count <= 0 { break; }
        line_count += 1;
    }
    println!("Open brace on: {}\nClosing brace on: {}", starting_line, line_count);
}

fn trim_to_starting_line(lines: Lines, line: usize) -> Lines {
    let mut lines = lines;
    if line > 0 {
        lines.nth(line - 2);
    }
    return lines;
}

fn get_file_contents(args: &[String]) -> String {
    let content = {
        args
            .iter()
            .find_map(|path| fs::read_to_string(path).ok())
    };
    return match content {
        Some(text) => text,
        None => {
            eprintln!("No valid or readable file found in arguments.");
            print_usage();
            std::process::exit(1);
        }
    };
}

fn get_starting_line_number(args: &[String]) -> usize {
    let first_number: Option<usize> = {
        args
            .into_iter()
            .skip(1)
            .find_map(|arg| arg.parse::<usize>().ok())
    };
    return match first_number {
        Some(num) => num,
        None => {
            eprintln!("Error: must provide a starting line number.");
            print_usage();
            std::process::exit(1);
        },
    };
}

