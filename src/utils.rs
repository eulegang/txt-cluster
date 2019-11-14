use std::io::{stdin, BufRead, BufReader};

pub fn lines(path: Option<&str>) -> Vec<String> {
    match path {
        None => {
            let sin = stdin();
            process_lines(sin.lock())
        }
        Some(path) => match std::fs::File::open(path) {
            Ok(file) => {
                let buf = BufReader::new(file);
                process_lines(buf)
            }
            Err(err) => {
                eprintln!("Error opening '{}': {}", path, err);
                std::process::exit(1);
            }
        },
    }
}

fn process_lines<B: BufRead>(buf: B) -> Vec<String> {
    let mut lines = Vec::new();
    let mut iter = buf.lines();
    while let Some(Ok(line)) = iter.next() {
        lines.push(line)
    }

    lines
}
