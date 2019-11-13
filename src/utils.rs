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

pub struct Combinations<'a, T> {
    vec: &'a Vec<T>,
    tortise: usize,
    hare: usize,
}

impl<'a, T> Combinations<'a, T> {
    pub fn new(vec: &'a Vec<T>) -> Combinations<'a, T> {
        let tortise = 0;
        let hare = 1;
        Combinations { vec, tortise, hare }
    }
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<(&'a T, &'a T)> {
        if self.tortise < self.vec.len() {
            if self.hare < self.vec.len() {
                let first = &self.vec[self.tortise];
                let second = &self.vec[self.hare];
                self.hare += 1;
                Some((first, second))
            } else {
                self.tortise += 1;
                self.hare = self.tortise + 1;
                if self.tortise < self.vec.len() && self.hare < self.vec.len() {
                    let first = &self.vec[self.tortise];
                    let second = &self.vec[self.hare];
                    self.hare += 1;
                    Some((first, second))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

#[test]
fn test_combinations() {
    let vec = vec![1, 2, 3, 4, 5];
    let combs = Combinations::new(&vec).collect::<Vec<(&i32, &i32)>>();

    assert_eq!(
        combs,
        vec![
            (&1, &2),
            (&1, &3),
            (&1, &4),
            (&1, &5),
            (&2, &3),
            (&2, &4),
            (&2, &5),
            (&3, &4),
            (&3, &5),
            (&4, &5),
        ]
    )
}
