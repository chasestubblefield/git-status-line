#[derive(Debug, PartialEq, Eq)]
struct GitStatus {
    oid: String,
    branch: String,
    upstream: Option<String>,
    ahead: bool,
    behind: bool,
    staged: bool,
    unstaged: bool,
    untracked: bool,
    ignored: bool,
}

impl GitStatus {
    fn new(status_txt: &str) -> Result<GitStatus, &'static str> {
        let mut s = GitStatus {
            oid: String::new(),
            branch: String::new(),
            upstream: None,
            ahead: false,
            behind: false,
            staged: false,
            unstaged: false,
            untracked: false,
            ignored: false,
        };
        for line in status_txt.lines() {
            let mut words = line.split(' ');
            match words.next() {
                Some("#") => {
                    match words.next() {
                        Some("branch.oid") => {
                            match words.next() {
                                Some(oid) => s.oid = String::from(oid),
                                None => return Err("Bad input"),
                            }
                        },
                        Some("branch.head") => {
                            match words.next() {
                                Some(branch) => s.branch = String::from(branch),
                                None => return Err("Bad input"),
                            }
                        }
                        Some("branch.upstream") => {
                            match words.next() {
                                Some(upstream) => s.upstream = Some(String::from(upstream)),
                                None => return Err("Bad input"),
                            }
                        },
                        Some("branch.ab") => {
                            match words.next() {
                                Some("+0") => {},
                                Some(_) => s.ahead = true,
                                None => return Err("Bad input"),
                            }
                            match words.next() {
                                Some("-0") => {},
                                Some(_) => s.behind = true,
                                None => return Err("Bad input"),
                            }
                        },
                        _ => return Err("Bad input"),
                    }
                },
                Some("1") | Some("2") => {
                    if let Some(changes) = words.next() {
                        let mut changes = changes.chars();
                        match changes.next() {
                            Some('.') => {},
                            Some(_) => s.staged = true,
                            None => return Err("Bad input"),
                        }
                        match changes.next() {
                            Some('.') => {},
                            Some(_) => s.unstaged = true,
                            None => return Err("Bad input"),
                        }
                    }
                },
                Some("?") => s.untracked = true,
                Some("!") => s.ignored = true,
                Some("") | None => {},
                _ => return Err("Bad input"),
            }
        }
        Ok(s)
    }

    fn to_line(&self) -> String {
        let mut line = String::from("[");
        if self.staged {
            line.push('+');
        }
        if self.unstaged {
            line.push('*');
        }
        if self.untracked {
            line.push('?');
        }
        if self.ignored {
            line.push('!');
        }
        line.push_str("] ");
        line
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
            untracked: false,
            ignored: false,
        };
        assert_eq!(s.to_line(), String::from("[master 3845e7a] "));
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
            untracked: true,
            ignored: true,
        };
        assert_eq!(s.to_line(), String::from("[master 3845e7a AB+*?!] "));
    }
}
