use std::io::{BufRead, BufReader, Read};

pub enum RecordSeperator {
    Null,
    Line,
    Paragraph,
}

pub struct DocReader<R>
where
    R: BufRead,
{
    read: R,
    buf: Vec<u8>,
    irs: RecordSeperator,
}

impl<R> DocReader<R>
where
    R: BufRead,
{
    pub fn new(read: R, irs: RecordSeperator) -> DocReader<R> {
        DocReader {
            read: read,
            buf: Vec::with_capacity(4096),
            irs,
        }
    }

    fn consume_to(&mut self, byte: u8) -> Option<String> {
        if let Ok(n) = self.read.read_until(byte, &mut self.buf) {
            if n != 0 {
                if Some(&byte) == self.buf.last() {
                    self.buf.pop();
                }

                Some(String::from(String::from_utf8_lossy(&self.buf).to_owned()))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<R> DocReader<BufReader<R>>
where
    R: Read,
{
    pub fn with_read(read: R, irs: RecordSeperator) -> DocReader<BufReader<R>> {
        DocReader {
            read: BufReader::new(read),
            buf: Vec::with_capacity(4096),
            irs,
        }
    }
}

impl<R> Iterator for DocReader<R>
where
    R: BufRead,
{
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.buf.clear();
        match self.irs {
            RecordSeperator::Null => self.consume_to(0u8),
            RecordSeperator::Line => self.consume_to(10u8),

            RecordSeperator::Paragraph => {
                let mut lines = Vec::new();
                let mut processed = false;
                let mut last_line = None::<String>;
                while let Some(line) = self.consume_to(10u8) {
                    self.buf.clear();
                    if line.is_empty() {
                        if last_line == None {
                            continue;
                        } else {
                            break;
                        }
                    }

                    if let Some(s) = last_line {
                        lines.push(s);
                    }

                    last_line = Some(line);
                    processed = true;
                }

                if let Some(s) = last_line {
                    lines.push(s)
                }

                if processed {
                    Some(lines.join("\n"))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_iter_eq {
        ($expected: expr, $actual: expr) => {{
            let mut i = 0;
            loop {
                match ($expected.next(), $actual.next()) {
                    (Some(e), Some(a)) => assert_eq!(e, a, "found at position: {}", i),
                    (Some(e), None) => assert!(
                        false,
                        "expected more entries than actual.  Extra expected \"{:?}\"",
                        e
                    ),
                    (None, Some(a)) => assert!(
                        false,
                        "actual had more entries than expected. Extra actual \"{:?}\"",
                        a
                    ),
                    (None, None) => break,
                }

                i += 1;
            }
        }};
    }

    #[test]
    fn test_null_seperator() {
        let buffer = b"hello\0world\0null\0seperated\0docs";
        let strs = vec![
            String::from("hello"),
            String::from("world"),
            String::from("null"),
            String::from("seperated"),
            String::from("docs"),
        ];

        let read_lines =
            DocReader::new(buffer as &[u8], RecordSeperator::Null).collect::<Vec<String>>();
        let mut expected = strs.iter();
        let mut actual = read_lines.iter();

        assert_iter_eq!(expected, actual);
    }

    #[test]
    fn test_line_seperator() {
        let buffer = b"hello\nworld\nnull\nseperated\ndocs";
        let strs = vec![
            String::from("hello"),
            String::from("world"),
            String::from("null"),
            String::from("seperated"),
            String::from("docs"),
        ];

        let read_lines =
            DocReader::new(buffer as &[u8], RecordSeperator::Line).collect::<Vec<String>>();

        let mut expected = strs.iter();
        let mut actual = read_lines.iter();

        assert_iter_eq!(expected, actual);
    }

    #[test]
    fn test_paragraph_seperator() {
        let buffer = b"hello\nworld\n\nnull\nseperated\n\ndocs";
        let strs = vec![
            String::from("hello\nworld"),
            String::from("null\nseperated"),
            String::from("docs"),
        ];

        let read_lines =
            DocReader::new(buffer as &[u8], RecordSeperator::Paragraph).collect::<Vec<String>>();

        let mut expected = strs.iter();
        let mut actual = read_lines.iter();

        assert_iter_eq!(expected, actual);
    }
}
