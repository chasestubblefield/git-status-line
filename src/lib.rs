use std::fmt;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Status {
    oid: String,
    branch: String,
    upstream: Option<String>,
    ahead: bool,
    behind: bool,
    staged: bool,
    unstaged: bool,
    unmerged: bool,
    untracked: bool,
    ignored: bool,
    merge_state: Option<MergeState>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum MergeState {
    Merge,
    Rebase,
}

impl Status {
    pub fn new(input: &str) -> Result<Status, &'static str> {
        let mut branch_info = HashMap::new();
        let mut ahead = false;
        let mut behind = false;
        let mut staged = false;
        let mut unstaged = false;
        let mut unmerged = false;
        let mut untracked = false;
        let mut ignored = false;
        let err = "Unexpected input";

        for line in input.lines() {
            let mut words = line.split(' ');
            match words.next() {
                Some("#") => {
                    let key = words.next().ok_or(err)?;
                    let value = words.next().ok_or(err)?;
                    match key {
                        "branch.oid" | "branch.head" | "branch.upstream" => {
                            branch_info.insert(key, value);
                        },
                        "branch.ab" => {
                            let ahead_count = value;
                            let behind_count = words.next().ok_or(err)?;

                            if ahead_count != "+0" {
                                ahead = true;
                            }
                            if behind_count != "-0" {
                                behind = true;
                            }
                        },
                        _ => return Err(err),
                    }
                },
                Some("1") | Some("2") => {
                    let changes = words.next().ok_or(err)?;
                    let mut changes = changes.chars();
                    let staged_change = changes.next().ok_or(err)?;
                    let unstaged_change = changes.next().ok_or(err)?;
                    if staged_change != '.' {
                        staged = true;
                    }
                    if unstaged_change != '.' {
                        unstaged = true;
                    }
                },
                Some("u") => unmerged = true,
                Some("?") => untracked = true,
                Some("!") => ignored = true,
                Some("") => {},
                _ => return Err(err),
            }
        }

        let oid = branch_info.get("branch.oid").ok_or(err)?.to_string();
        let branch = branch_info.get("branch.head").ok_or(err)?.to_string();
        let upstream = branch_info.get("branch.upstream").map(|s| s.to_string());

        Ok(Status {
            oid,
            branch,
            upstream,
            ahead,
            behind,
            staged,
            unstaged,
            unmerged,
            untracked,
            ignored,
            merge_state: None,
        })
    }

    pub fn set_merge_state(&mut self, m: MergeState) {
        self.merge_state = Some(m);
    }

    pub fn is_dirty(&self) -> bool {
        self.ahead || self.behind || self.staged || self.unstaged || self.unmerged || self.untracked || self.ignored
    }

    fn to_string(&self) -> String {
        let mut result = String::new();

        result.push('[');

        result.push_str(&self.branch);

        result.push(' ');

        if self.oid == "(initial)" {
            result.push_str(&self.oid);
        } else {
            result.push_str(&self.oid[..7]);
        };

        match self.merge_state {
            Some(MergeState::Merge) => result.push_str(" (merge)"),
            Some(MergeState::Rebase) => result.push_str(" (rebase)"),
            _ => {},
        }

        if self.is_dirty() {
            result.push(' ');
            if self.ahead {
                result.push('A');
            }
            if self.behind {
                result.push('B');
            }
            if self.staged {
                result.push('+');
            }
            if self.unstaged {
                result.push('*');
            }
            if self.unmerged {
                result.push('%');
            }
            if self.untracked {
                result.push('?');
            }
            if self.ignored {
                result.push('!');
            }
        }

        result.push(']');

        result
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clean() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
";
        assert_eq!(
            Status::new(input).unwrap(),
            Status {
                oid: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                branch: String::from("master"),
                upstream: None,
                ahead: false,
                behind: false,
                staged: false,
                unstaged: false,
                unmerged: false,
                untracked: false,
                ignored: false,
                merge_state: None,
            }
        );
    }

    #[test]
    fn new_clean_upstream() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +1 -1
";
        assert_eq!(
            Status::new(input).unwrap(),
            Status {
                oid: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                branch: String::from("master"),
                upstream: Some(String::from("origin/master")),
                ahead: true,
                behind: true,
                staged: false,
                unstaged: false,
                unmerged: false,
                untracked: false,
                ignored: false,
                merge_state: None,
            }
        );
    }

    #[test]
    fn new_staged_only() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 M. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md
";
        let s = Status::new(input).unwrap();
        assert_eq!(
            (s.staged, s.unstaged, s.unmerged),
            (true, false, false)
        );
    }

    #[test]
    fn new_unstaged_only() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 .M N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 README.md
";
        let s = Status::new(input).unwrap();
        assert_eq!(
            (s.staged, s.unstaged, s.unmerged),
            (false, true, false)
        );
    }

    #[test]
    fn new_both_staged_and_unstaged() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 MM N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md
";
        let s = Status::new(input).unwrap();
        assert_eq!(
            (s.staged, s.unstaged, s.unmerged),
            (true, true, false)
        );
    }

    #[test]
    fn new_deleted() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 D. N... 100644 000000 000000 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 0000000000000000000000000000000000000000 README.md
";
        let s = Status::new(input).unwrap();
        assert_eq!(
            (s.staged, s.unstaged, s.unmerged),
            (true, false, false)
        );
    }

    #[test]
    fn new_renamed() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
2 R. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 R100 README	README.md
";
        let s = Status::new(input).unwrap();
        assert_eq!(
            (s.staged, s.unstaged, s.unmerged),
            (true, false, false)
        );
    }

    #[test]
    fn new_unmerged() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
u UU N... 100644 100644 100644 100644 8fb20c5f0b7da31f56f74f0a98e1fadb13e4c2a0 801dd97d4dace6780f9eca5a99dbee77d6e05a95 cbf6eb8db76897842f3b77d1d2b95dbd422c180d README.md
";
        let s = Status::new(input).unwrap();
        assert_eq!(
            (s.staged, s.unstaged, s.unmerged),
            (false, false, true)
        );
    }

    #[test]
    fn new_untracked() {
        let input = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
? foo.txt
";
        assert!(Status::new(input).unwrap().untracked);
    }

    #[test]
    fn new_ignored() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
! node_modules/
";
        assert!(Status::new(test_status).unwrap().ignored);
    }

    #[test]
    fn new_very_dirty() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +1 -1
1 D. N... 100644 000000 000000 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 0000000000000000000000000000000000000000 README.md
1 .M N... 100644 100644 100644 5e8a8090976077ddf16252a560460a20dbbdd6a5 5e8a8090976077ddf16252a560460a20dbbdd6a5 gh-pages.sh
? foo.txt
! ignored.txt
";
        assert_eq!(
            Status::new(test_status).unwrap(),
            Status {
                oid: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                branch: String::from("master"),
                upstream: Some(String::from("origin/master")),
                ahead: true,
                behind: true,
                staged: true,
                unstaged: true,
                unmerged: false,
                untracked: true,
                ignored: true,
                merge_state: None,
            }
        );
    }

    #[test]
    fn new_unexpected_input() {
        assert_eq!(Status::new("foo"), Err("Unexpected input"));
    }

    #[test]
    fn to_string_clean() {
        let s = Status {
            oid: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
            branch: String::from("master"),
            upstream: None,
            ahead: false,
            behind: false,
            staged: false,
            unstaged: false,
            unmerged: false,
            untracked: false,
            ignored: false,
            merge_state: None,
        };
        assert_eq!(s.to_string(), "[master 3845e7a]");
    }

    #[test]
    fn to_string_dirty() {
        let s = Status {
            oid: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
            branch: String::from("master"),
            upstream: Some(String::from("origin/master")),
            ahead: true,
            behind: true,
            staged: true,
            unstaged: true,
            unmerged: true,
            untracked: true,
            ignored: true,
            merge_state: None,
        };
        assert_eq!(s.to_string(), "[master 3845e7a AB+*%?!]");
    }

    #[test]
    fn to_string_initial_commit() {
        let s = Status {
            oid: String::from("(initial)"),
            branch: String::from("master"),
            upstream: None,
            ahead: false,
            behind: false,
            staged: false,
            unstaged: false,
            unmerged: false,
            untracked: false,
            ignored: false,
            merge_state: None,
        };
        assert_eq!(s.to_string(), "[master (initial)]");
    }

    #[test]
    fn to_string_merge() {
        let s = Status {
            oid: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
            branch: String::from("master"),
            upstream: None,
            ahead: false,
            behind: false,
            staged: false,
            unstaged: false,
            unmerged: true,
            untracked: false,
            ignored: false,
            merge_state: Some(MergeState::Merge),
        };
        assert_eq!(s.to_string(), "[master 3845e7a (merge) %]");
    }
}
