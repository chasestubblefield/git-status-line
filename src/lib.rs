#[derive(Debug, PartialEq, Eq)]
struct GitStatus {
    branch: Option<BranchInfo>,
    staged: bool,
    unstaged: bool,
    untracked: bool,
    ignored: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct BranchInfo {
    object_id: String,
    name: Option<String>,
    upstream: Option<UpstreamInfo>,
}

#[derive(Debug, PartialEq, Eq)]
struct UpstreamInfo {
    name: String,
    ahead: bool,
    behind: bool,
}

impl GitStatus {
    fn new(status_txt: &str) -> Result<GitStatus, &'static str> {
        let mut s = GitStatus {
            branch: None,
            staged: false,
            unstaged: false,
            untracked: false,
            ignored: false,
        };
        let mut object_id: Option<String> = None;
        let mut branch_name: Option<String> = None;
        let mut upstream_name: Option<String> = None;
        let mut ahead = false;
        let mut behind = false;
        for line in status_txt.lines() {
            let mut words = line.split(' ');
            match words.next() {
                Some("#") => {
                    match words.next() {
                        Some("branch.oid") => {
                            if let Some(oid) = words.next() {
                                object_id = Some(String::from(oid));
                            }
                        },
                        Some("branch.head") => {
                            if let Some(head) = words.next() {
                                branch_name = Some(String::from(head));
                            }
                        }
                        Some("branch.upstream") => {
                            if let Some(upstream) = words.next() {
                                upstream_name = Some(String::from(upstream));
                            }
                        },
                        Some("branch.ab") => {
                            match words.next() {
                                Some("+0") | None => {},
                                Some(_) => ahead = true,
                            }
                            match words.next() {
                                Some("-0") | None => {},
                                Some(_) => behind = true,
                            }
                        },
                        _ => {},
                    }
                },
                Some("1") | Some("2") => {
                    if let Some(changes) = words.next() {
                        let mut changes = changes.chars();
                        match changes.next() {
                            Some('.') | None => {},
                            Some(_) => s.staged = true,
                        }
                        match changes.next() {
                            Some('.') | None => {},
                            Some(_) => s.unstaged = true,
                        }
                    }
                },
                Some("?") => s.untracked = true,
                Some("!") => s.ignored = true,
                Some("") | None => {},
                _ => return Err("Unrecognized line in status"),
            }
        }
        if branch_name == Some(String::from("(detached)")) {
            branch_name = None;
        }
        if let Some(oid) = object_id {
            let mut b_info = BranchInfo {
                object_id: oid,
                name: branch_name,
                upstream: None,
            };
            if let Some(up) = upstream_name {
                b_info.upstream = Some(UpstreamInfo {
                    name: up,
                    ahead: ahead,
                    behind: behind,
                });
            }
            s.branch = Some(b_info);
        }
        Ok(s)
    }

    fn to_line(&self) -> String {
        let mut line = String::from("(");
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
        line.push_str(") ");
        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_clean() {
        let test_status = "\n";
        assert_eq!(
            GitStatus::new(test_status).unwrap(),
            GitStatus {
                branch: None,
                staged: false,
                unstaged: false,
                untracked: false,
                ignored: false,
            }
        );
    }

    #[test]
    fn parse_branch() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
";
        assert_eq!(
            GitStatus::new(test_status).unwrap().branch,
            Some(BranchInfo {
                object_id: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                name: Some(String::from("master")),
                upstream: None,

            })
        );
    }

    #[test]
    fn parse_detached_head() {
        let test_status = "\
# branch.oid 2a7bb916bba69a5fc9d428acc80b1bce64e3e0bc
# branch.head (detached)
";
        assert_eq!(
            GitStatus::new(test_status).unwrap().branch,
            Some(BranchInfo {
                object_id: String::from("2a7bb916bba69a5fc9d428acc80b1bce64e3e0bc"),
                name: None,
                upstream: None,
            })
        );
    }

    #[test]
    fn parse_upstream() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +1 -1
";
        assert_eq!(
            GitStatus::new(test_status).unwrap().branch,
            Some(BranchInfo {
                object_id: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                name: Some(String::from("master")),
                upstream: Some(UpstreamInfo {
                    name: String::from("origin/master"),
                    ahead: true,
                    behind: true,
                }),
            })
        );
    }

    #[test]
    fn parse_staged_only() {
        let test_status = "1 M. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md\n";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(!s.unstaged);
    }

    #[test]
    fn parse_unstaged_only() {
        let test_status = "1 .M N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 README.md\n";
        let s = GitStatus::new(test_status).unwrap();
        assert!(!s.staged);
        assert!(s.unstaged);
    }

    #[test]
    fn parse_both_staged_and_unstaged() {
        let test_status = "1 MM N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md\n";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(s.unstaged);
    }

    #[test]
    fn parse_deleted() {
        let test_status = "1 D. N... 100644 000000 000000 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 0000000000000000000000000000000000000000 README.md\n";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(!s.unstaged);
    }

    #[test]
    fn parse_renamed() {
        let test_status = "2 R. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 R100 README	README.md\n";
        let s = GitStatus::new(test_status).unwrap();
        assert!(s.staged);
        assert!(!s.unstaged);
    }

    #[test]
    fn parse_untracked() {
        let test_status = "? foo.txt\n";
        assert!(GitStatus::new(test_status).unwrap().untracked);
    }

    #[test]
    fn parse_ignored() {
        let test_status = "! node_modules/\n";
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
                branch: Some(BranchInfo {
                    object_id: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                    name: Some(String::from("master")),
                    upstream: Some(UpstreamInfo {
                        name: String::from("origin/master"),
                        ahead: true,
                        behind: true,
                    }),
                }),
                staged: true,
                unstaged: true,
                untracked: true,
                ignored: true,
            }
        );
    }

    #[test]
    fn parse_improper_status() {
        assert_eq!(GitStatus::new("foo"), Err("Unrecognized line in status"));
    }

    #[test]
    fn test_to_line_clean_no_branch() {
        let s = GitStatus {
            branch: None,
            staged: false,
            unstaged: false,
            untracked: false,
            ignored: false,
        };
        assert_eq!(s.to_line(), String::from("() "));
    }

    #[test]
    fn test_to_line_clean_with_branch() {
        let s = GitStatus {
            branch: Some(BranchInfo {
                object_id: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                name: Some(String::from("master")),
                upstream: None,
            }),
            staged: false,
            unstaged: false,
            untracked: false,
            ignored: false,
        };
        assert_eq!(s.to_line(), String::from("(master) "));
    }

    #[test]
    fn test_to_line_dirty() {
        let s = GitStatus {
            branch: Some(BranchInfo {
                object_id: String::from("3845e7a3c3aadaaebb2d1b261bf07a9357d35a79"),
                name: Some(String::from("master")),
                upstream: Some(UpstreamInfo {
                    name: String::from("origin/master"),
                    ahead: true,
                    behind: true,
                }),
            }),
            staged: true,
            unstaged: true,
            untracked: true,
            ignored: true,
        };
        assert_eq!(s.to_line(), String::from("(master AB+*?!) "));
    }
}
