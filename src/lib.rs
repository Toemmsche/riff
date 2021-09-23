pub mod io {
    use std::path::Path;
    use std::fs::read_to_string;

    /// The newline character that signals the end of a line.
    const NEWLINE: &str = "\n";

    /// Read a file and transform its content into a sequence of lines.
    /// A line ends whenever the newline character is found and can be empty.
    pub fn lines(path: &Path) -> Vec<String> {
        // Ignore newline at the end for now...
        read_to_string(path)
            .expect("Unable to read file")
            .split(NEWLINE)
            .map(|str| String::from(str))
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod io_tests {
    use crate::io;

    #[test]
    fn test_dummy() {
        assert_eq!(4, 2 * 2);
    }


    #[test]
    fn standard_lines() {
        io::lines("test_files/std_A.txt".as_ref());
        io::lines("test_files/std_B.txt".as_ref());
    }
}

#[cfg(test)]
pub mod lcs_tests {
    use crate::diff;

    #[test]
    pub fn small() {
        let seq_a = "science"
            .split("")
            .map(|str| String::from(str))
            .filter(|str| !str.is_empty())
            .collect::<Vec<String>>();
        let seq_b = "incentive"
            .split("")
            .map(|str| String::from(str))
            .filter(|str| !str.is_empty())
            .collect::<Vec<String>>();


        let lcs = diff::lcs(&seq_a, &seq_b);
        assert_eq!(lcs.len(), 4);

        for lcs_item in lcs {
            assert_eq!(seq_a[lcs_item.index_a as usize], seq_b[lcs_item.index_b as usize]);
        }
    }
}

pub mod diff {
    use std::cmp::max;

    /// A wrapper for an index pair. For two sequences A and B, the items at
    /// the respective indices should be (partially) equal.
    #[derive(Debug)]
    pub struct LcsItem {
        pub index_a: i32,
        pub index_b: i32,
    }

    impl PartialEq for LcsItem {
        fn eq(&self, other: &LcsItem) -> bool {
            return self.index_a == other.index_a
                && self.index_b == other.index_b;
        }

        fn ne(&self, other: &LcsItem) -> bool {
            return !(self == other);
        }
    }

    /// Compute the longest common subsequence between two string sequences A and B.
    /// Equality between elements is determined by the "==" operator.
    pub fn lcs(seq_a: &Vec<String>, seq_b: &Vec<String>) -> Vec<LcsItem> {
        let len_a = seq_a.len();
        let len_b = seq_b.len();

        let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

        for i in 1..len_a + 1 {
            for j in 1..len_b + 1 {
                if seq_a[i - 1] == seq_b[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }

        // Extract lcs
        let mut r = len_a;
        let mut c = len_b;

        let mut res = Vec::new();
        while r > 0 && c > 0 {
            if seq_a[r - 1] == seq_b[c - 1] {
                res.push(LcsItem { index_a: (r - 1) as i32, index_b: (c - 1) as i32 });
                r -= 1;
                c -= 1;
            } else if dp[r][c] == dp[r - 1][c] {
                r -= 1;
            } else {
                c -= 1;
            }
        }
        res.reverse();
        res
    }

    #[derive(Debug)]
    pub enum EditType {
        INSERT,
        DELETE,
        NIL,
    }

    /// An potentially changed line.
    #[derive(Debug)]
    pub struct DeltaLine {
        pub edit_type: EditType,
        pub line: String,
    }

    impl DeltaLine {
        pub fn to_string(&self) -> String {
            // TODO
            "".to_string()
        }
    }

    pub fn diff(seq_a: &Vec<String>, seq_b: &Vec<String>) -> Vec<DeltaLine> {
        let mut lcs = lcs(seq_a, seq_b);
        let mut res = Vec::new();

        // insert dummies
        lcs.insert(0, LcsItem { index_a: -1, index_b: -1 });

        lcs.push(LcsItem { index_a: seq_a.len() as i32, index_b: seq_b.len() as i32 });

        println!("{:?}", lcs);
        for i in 1..lcs.len() {
            if lcs[i].index_a != lcs[i - 1].index_a {
                // Deletions
                for line in &seq_a[((lcs[i - 1].index_a + 1) as usize)..(lcs[i].index_a as usize)] {
                    res.push(DeltaLine {
                        edit_type: EditType::DELETE,
                        line: line.clone(),
                    })
                }
            }
            if lcs[i].index_b != lcs[i - 1].index_b {
                // Deletions
                for line in &seq_b[((lcs[i - 1].index_b + 1) as usize)..(lcs[i].index_b as usize)] {
                    res.push(DeltaLine {
                        edit_type: EditType::INSERT,
                        line: line.clone(),
                    })
                }
            }
            if(i < lcs.len() - 1) {
                res.push(DeltaLine {
                    edit_type: EditType::NIL,
                    line: seq_a[lcs[i].index_a as usize].clone(),
                });
            }
        }
        res
    }

    /// A hunk that groups multiple (changed) lines that are though to be part
    /// the same edit process.
    #[derive(Debug)]
    pub struct Hunk {
        begin_a: usize,
        count_a: usize,
        begin_b: usize,
        count_b: usize,
    }

    /// The number of unchanged lines (if they exist) at the start and end
    /// of a hunk that cannot be part of any other hunk.
    const HUNK_RADIUS: usize = 3;
}
