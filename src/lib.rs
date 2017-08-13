#[derive(Debug, PartialEq, Eq)]
pub struct GitStatus {
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
}

impl GitStatus {
    pub fn new(status_txt: &str) -> Result<GitStatus, &'static str> {
        let mut s = GitStatus {
            oid: String::new(),
            branch: String::new(),
            upstream: None,
            ahead: false,
            behind: false,
            staged: false,
            unstaged: false,
            untracked: false,
            unmerged: false,
            ignored: false,
        };
        let err = "Bad input";
        for line in status_txt.lines() {
            let mut words = line.split(' ');
            match words.next() {
                Some("#") => {
                    let key = words.next().ok_or(err)?;
                    let value = words.next().ok_or(err)?;
                    match key {
                        "branch.oid" => s.oid = String::from(value),
                        "branch.head" => s.branch = String::from(value),
                        "branch.upstream" => s.upstream = Some(String::from(value)),
                        "branch.ab" => {
                            if value != "+0" {
                                s.ahead = true;
                            }
                            if words.next().ok_or(err)? != "-0" {
                                s.behind = true;
                            }
                        },
                        _ => return Err(err),
                    }
                },
                Some("1") | Some("2") => {
                    let changes = words.next().ok_or(err)?;
                    let mut changes = changes.chars();
                    if changes.next().ok_or(err)? != '.' {
                        s.staged = true;
                    }
                    if changes.next().ok_or(err)? != '.' {
                        s.unstaged = true;
                    }
                },
                Some("u") => s.unmerged = true,
                Some("?") => s.untracked = true,
                Some("!") => s.ignored = true,
                Some("") | None => {},
                _ => return Err(err),
            }
        }
        Ok(s)
    }

    pub fn to_line(&self) -> String {
        let mut symbols = String::new();
        if self.ahead {
            symbols.push('A');
        }
        if self.behind {
            symbols.push('B');
        }
        if self.staged {
            symbols.push('+');
        }
        if self.unstaged {
            symbols.push('*');
        }
        if self.untracked {
            symbols.push('?');
        }
        if self.ignored {
            symbols.push('!');
        }
        let short_oid = if self.oid == "(initial)" {
            &self.oid
        } else {
            &self.oid[..7]
        };
        if symbols.is_empty() {
            format!("[{} {}]", self.branch, short_oid)
        } else {
            format!("[{} {} {}]", self.branch, short_oid, symbols)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_clean() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
";
        assert_eq!(
            GitStatus::new(test_status).unwrap(),
            GitStatus {
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
            }
        );
    }

    #[test]
    fn parse_clean_upstream() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +1 -1
";
        assert_eq!(
            GitStatus::new(test_status).unwrap(),
            GitStatus {
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
            }
        );
    }

    #[test]
    fn parse_staged_only() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 M. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md
";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(!s.unstaged);
    }

    #[test]
    fn parse_unstaged_only() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 .M N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 README.md
";
        let s = GitStatus::new(test_status).unwrap();
        assert!(!s.staged);
        assert!(s.unstaged);
    }

    #[test]
    fn parse_both_staged_and_unstaged() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 MM N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md
";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(s.unstaged);
    }

    #[test]
    fn parse_deleted() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
1 D. N... 100644 000000 000000 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 0000000000000000000000000000000000000000 README.md
";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(!s.unstaged);
    }

    #[test]
    fn parse_renamed() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
2 R. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 R100 README	README.md
";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(!s.unstaged);
    }

    #[test]
    fn parse_unmerged() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
u UU N... 100644 100644 100644 100644 8fb20c5f0b7da31f56f74f0a98e1fadb13e4c2a0 801dd97d4dace6780f9eca5a99dbee77d6e05a95 cbf6eb8db76897842f3b77d1d2b95dbd422c180d README.md
";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.unmerged);
    }

    #[test]
    fn parse_untracked() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
? foo.txt
";
        assert!(GitStatus::new(test_status).unwrap().untracked);
    }

    #[test]
    fn parse_ignored() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
! node_modules/
";
        assert!(GitStatus::new(test_status).unwrap().ignored);
    }

    #[test]
    fn parse_very_dirty() {
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
            GitStatus::new(test_status).unwrap(),
            GitStatus {
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
            }
        );
    }

    #[test]
    fn parse_bad_input() {
        assert_eq!(GitStatus::new("foo"), Err("Bad input"));
    }

    #[test]
    fn test_to_line_clean() {
        let s = GitStatus {
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
        };
        assert_eq!(s.to_line(), String::from("[master 3845e7a]"));
    }

    #[test]
    fn test_to_line_dirty() {
        let s = GitStatus {
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
        };
        assert_eq!(s.to_line(), String::from("[master 3845e7a AB+*?!]"));
    }

    #[test]
    fn test_to_line_initial_commit() {
        let s = GitStatus {
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
        };
        assert_eq!(s.to_line(), String::from("[master (initial)]"));
    }
}
