use crate::cluster::Cluster;
use std::io::Write;

pub enum FieldSeperator {
    Null,
    Colon,
    Line,
}

pub enum RecordSeperator {
    Null,
    Line,
    DLine,
}

impl FieldSeperator {
    fn repr(&self) -> &str {
        match self {
            FieldSeperator::Null => "\0",
            FieldSeperator::Colon => ":",
            FieldSeperator::Line => "\n",
        }
    }
}

impl RecordSeperator {
    fn repr(&self) -> &str {
        match self {
            RecordSeperator::Null => "\0",
            RecordSeperator::Line => "\n",
            RecordSeperator::DLine => "\n\n",
        }
    }
}

pub struct ClusterOutput<'w, W: Write> {
    write: &'w mut W,
    ofs: FieldSeperator,
    ors: RecordSeperator,
}

impl<'w, W> ClusterOutput<'w, W>
where
    W: Write,
{
    pub fn new(
        write: &'w mut W,
        ofs: FieldSeperator,
        ors: RecordSeperator,
    ) -> ClusterOutput<'w, W> {
        ClusterOutput { write, ofs, ors }
    }

    pub fn output<'a>(&mut self, cluster: Cluster<'a>) {
        let ofs = self.ofs.repr();
        let ors = self.ors.repr();

        let mut rsep = "";
        for set in cluster {
            let _ = write!(self.write, "{}", rsep);
            let mut fsep = "";
            for r in set {
                let _ = write!(self.write, "{}", fsep);
                let _ = write!(self.write, "{}", r);

                fsep = ofs;
            }

            rsep = ors;
        }

        let _ = write!(self.write, "\n");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use super::FieldSeperator as FS;
    use super::RecordSeperator as RS;

    macro_rules! assert_split_contains {
        ($str: expr, $split: expr, $elem: expr) => {
            assert!(
                $str.split($split)
                    .map(String::from)
                    .collect::<Vec<String>>()
                    .contains(&$elem),
                "{:?} was not in cluster {:?}",
                $elem,
                $str
            );
        };
    }

    macro_rules! run_output {
        ($cluster: expr, $fs:expr, $rs: expr) => {{
            let mut buf = Vec::new();
            let mut out = ClusterOutput::new(&mut buf, $fs, $rs);
            out.output($cluster);

            let result = String::from_utf8_lossy(&buf);

            result
                .trim_end_matches("\n")
                .split($rs.repr())
                .map(String::from)
                .collect::<Vec<String>>()
        }};
    }

    #[test]
    fn default_line_dline() {
        let a = "hello".to_string();
        let b = "world".to_string();

        let c = "sweet".to_string();
        let d = "home".to_string();

        let cluster = Cluster::pairwise(vec![(&a, &b), (&c, &d)]);
        let lines = run_output!(cluster, FS::Line, RS::DLine);

        assert_split_contains!(lines[0], "\n", &a);
        assert_split_contains!(lines[0], "\n", &b);

        assert_split_contains!(lines[1], "\n", &c);
        assert_split_contains!(lines[1], "\n", &d);
    }

    #[test]
    fn colon_line() {
        let a = "hello".to_string();
        let b = "world".to_string();

        let c = "sweet".to_string();
        let d = "home".to_string();

        let cluster = Cluster::pairwise(vec![(&a, &b), (&c, &d)]);
        let lines = run_output!(cluster, FS::Colon, RS::Line);

        assert_split_contains!(lines[0], ":", &a);
        assert_split_contains!(lines[0], ":", &b);

        assert_split_contains!(lines[1], ":", &c);
        assert_split_contains!(lines[1], ":", &d);
    }

    #[test]
    fn null_line() {
        let a = "hello".to_string();
        let b = "world".to_string();

        let c = "sweet".to_string();
        let d = "home".to_string();

        let cluster = Cluster::pairwise(vec![(&a, &b), (&c, &d)]);
        let lines = run_output!(cluster, FS::Null, RS::Line);

        assert_split_contains!(lines[0], "\0", &a);
        assert_split_contains!(lines[0], "\0", &b);

        assert_split_contains!(lines[1], "\0", &c);
        assert_split_contains!(lines[1], "\0", &d);
    }

    #[test]
    fn colon_null() {
        let a = "hello".to_string();
        let b = "world".to_string();

        let c = "sweet".to_string();
        let d = "home".to_string();

        let cluster = Cluster::pairwise(vec![(&a, &b), (&c, &d)]);
        let lines = run_output!(cluster, FS::Colon, RS::Null);

        assert_split_contains!(lines[0], ":", &a);
        assert_split_contains!(lines[0], ":", &b);

        assert_split_contains!(lines[1], ":", &c);
        assert_split_contains!(lines[1], ":", &d);
    }
}
