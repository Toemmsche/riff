pub mod riff {
    use std::cmp::max;
    use std::fs::read_to_string;
    use std::path::Path;

    /// The number of unchanged lines (if they exist) at the start and end
    /// of a hunk that cannot be part of any other hunk.
    pub const HUNK_RADIUS: usize = 3;

    /// A wrapper for an index pair. For two sequences A and B, the items at
    /// the respective indices should be (partially) equal.
    #[derive(Debug)]
    pub struct LcsItem {
        pub index_a: usize,
        pub index_b: usize,
    }

    #[derive(Debug)]
    pub enum EditType {
        INSERT,
        DELETE,
        NIL,
    }

    /// An potentially changed line.
    pub struct DeltaLine {
        pub edit_type: EditType,
        pub line: String,
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

    /// Read a file and transform its content into a sequence of lines.
    /// A line ends whenever the newline character is found and can be empty.
    pub fn lines(path: &Path) -> Vec<String> {
        // Ignore newline at the end for now...
        read_to_string(path)
            .expect("Unable to read file")
            .lines()
            .map(|str| String::from(str).replace("\t", ""))
            .collect::<Vec<String>>()
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
                res.push(LcsItem { index_a: r - 1, index_b: c - 1 });
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

    impl DeltaLine {
        pub fn to_string(&self) -> String {
            let mut prefix = match self.edit_type {
                EditType::DELETE => "-".to_string(),
                EditType::INSERT => "+".to_string(),
                EditType::NIL => " ".to_string(),
            };
            prefix.push_str(&self.line);
            prefix
        }
    }

    /// Create a diff from two string sequences. The diff uses the LCS among the sequences and
    /// marks inserted and deleted lines.
    pub fn diff(seq_a: &Vec<String>, seq_b: &Vec<String>) -> Vec<DeltaLine> {
        let mut lcs = lcs(seq_a, seq_b);
        let mut res = Vec::new();

        // insert dummy at the end
        lcs.push(LcsItem { index_a: seq_a.len(), index_b: seq_b.len() });

        let mut next_a = 0;
        let mut next_b = 0;
        for i in 0..lcs.len() {
            if lcs[i].index_a != next_a {
                // Deletions
                for line in &seq_a[next_a..lcs[i].index_a] {
                    res.push(DeltaLine {
                        edit_type: EditType::DELETE,
                        line: line.clone(),
                    })
                }
            }
            if lcs[i].index_b != next_b {
                // Deletions
                for line in &seq_b[next_b..lcs[i].index_b] {
                    res.push(DeltaLine {
                        edit_type: EditType::INSERT,
                        line: line.clone(),
                    })
                }
            }
            if i < lcs.len() - 1 {
                res.push(DeltaLine {
                    edit_type: EditType::NIL,
                    line: seq_a[lcs[i].index_a].clone(),
                });
            }
            next_a = lcs[i].index_a + 1;
            next_b = lcs[i].index_b + 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {

    use crate::riff;

    #[test]
    fn standard_lines() {
        riff::lines("test_files/std_A.txt".as_ref());
        riff::lines("test_files/std_B.txt".as_ref());
    }

    #[test]
    pub fn small_lcs() {
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


        let lcs = riff::lcs(&seq_a, &seq_b);
        assert_eq!(lcs.len(), 4);

        for lcs_item in lcs {
            assert_eq!(seq_a[lcs_item.index_a], seq_b[lcs_item.index_b]);
        }
    }

    #[test]
    pub fn small_diff() {
        let seq_a = riff::lines("test_files/std_A.txt".as_ref());
        let seq_b = riff::lines("test_files/std_B.txt".as_ref());

        let out = riff::diff(&seq_a, &seq_b).iter().map(|x| x.to_string()).collect::<Vec<String>>();
        assert_eq!(out.len(), 35);
    }
}
