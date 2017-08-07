#[derive(Debug, PartialEq, Eq)]
struct GitStatus {
    branch: Option<BranchInfo>,
    upstream: Option<UpstreamInfo>,
    staged: bool,
    unstaged: bool,
    untracked: bool,
    ignored: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct BranchInfo {
    object_id: String,
    branch: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct UpstreamInfo {
    upstream: String,
    ahead: bool,
    behind: bool,
}

impl GitStatus {
    fn new(status_txt: &str) -> Result<GitStatus, &'static str> {
        let mut s = GitStatus {
            branch: None,
            upstream: None,
            staged: false,
            unstaged: false,
            untracked: false,
            ignored: false,
        };
        for line in status_txt.lines() {
            let mut words = line.split(' ');
            match words.next() {
                Some("#") => {
                },
                Some("1") | Some("2") => {
                    if let Some(changes) = words.next() {
                        let mut changes = changes.chars();
                        match changes.next() {
                            Some('.') => {},
                            Some(_) => s.staged = true,
                            _ => {},
                        }
                        match changes.next() {
                            Some('.') => {},
                            Some(_) => s.unstaged = true,
                            _ => {},
                        }
                    }
                },
                Some("?") => s.untracked = true,
                Some("!") => s.ignored = true,
                _ => {},
            }
        }
        Ok(s)
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
                upstream: None,
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
                branch: Some(String::from("master")),
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
                branch: None
            })
        );
    }

    #[test]
    fn parse_upstream() {
        let test_status = "\
# branch.upstream origin/master
# branch.ab +1 -1
";
        assert_eq!(
            GitStatus::new(test_status).unwrap().upstream,
            Some(UpstreamInfo {
                upstream: String::from("origin/master"),
                ahead: true,
                behind: true,
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
                    branch: Some(String::from("master")),
                }),
                upstream: Some(UpstreamInfo {
                    upstream: String::from("origin/master"),
                    ahead: true,
                    behind: true,
                }),
                staged: true,
                unstaged: true,
                untracked: true,
                ignored: true,
            }
        );
    }
}
