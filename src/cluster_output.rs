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

pub struct ClusterOutput<W: Write> {
    write: W,
    ofs: FieldSeperator,
    ors: RecordSeperator,
}

impl<W> ClusterOutput<W>
where
    W: Write,
{
    pub fn new(write: W, ofs: FieldSeperator, ors: RecordSeperator) -> ClusterOutput<W> {
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
