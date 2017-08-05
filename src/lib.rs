#[derive(Debug, PartialEq, Eq)]
struct Status {
    untracked: bool,
    unstaged: bool,
    staged: bool,
    ahead: bool,
    behind: bool,
    branch: String,
    upstream: String,
}

impl Status {
    fn new(status_txt: &str) -> Result<Status, &'static str> {
        let mut s = Status {
            untracked: false,
            unstaged: false,
            staged: false,
            ahead: false,
            behind: false,
            branch: String::new(),
            upstream: String::new(),
        };
        for line in status_txt.lines() {
            if line.starts_with("?") {
                s.untracked = true;
            }
            if line.starts_with("1 .M") {
                s.unstaged = true;
            }
            if line.starts_with("1 M.") {
                s.staged = true;
            }
            if line.starts_with("1 MM") {
                s.unstaged = true;
                s.staged = true;
            }
            if line.starts_with("# branch.head") {
                let v: Vec<&str> = line.split(' ').collect();
                s.branch = String::from(v[2]);
            }
            if line.starts_with("# branch.upstream") {
                let v: Vec<&str> = line.split(' ').collect();
                s.upstream = String::from(v[2]);
            }
            if line.starts_with("# branch.ab") {
                let v: Vec<&str> = line.split(' ').collect();
                if v[2] != "+0" {
                    s.ahead = true;
                }
                if v[3] != "-0" {
                    s.behind = true;
                }
            }
        }
        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_dirty() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +1 -1
1 M. N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md
1 .M N... 100644 100644 100644 5e8a8090976077ddf16252a560460a20dbbdd6a5 5e8a8090976077ddf16252a560460a20dbbdd6a5 gh-pages.sh
? foo.txt
";
        assert_eq!(
            Status::new(&test_status).unwrap(),
            Status {
                untracked: true,
                unstaged: true,
                staged: true,
                branch: String::from("master"),
                upstream: String::from("origin/master"),
                ahead: true,
                behind: true,
            }
        );
    }

    #[test]
    fn parse_clean() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +0 -0
";
        assert_eq!(
            Status::new(&test_status).unwrap(),
            Status {
                untracked: false,
                unstaged: false,
                staged: false,
                branch: String::from("master"),
                upstream: String::from("origin/master"),
                ahead: false,
                behind: false,
            }
        );
    }

    #[test]
    fn parse_staged_and_unstaged_changes_on_same_file() {
        let test_status = "\
# branch.oid 3845e7a3c3aadaaebb2d1b261bf07a9357d35a79
# branch.head master
# branch.upstream origin/master
# branch.ab +0 -0
1 MM N... 100644 100644 100644 1290f45e7ad7575848a436d8febbd6c4ba07f1f3 311c77295c5b6056f4599c2b8d0a019d4c76746a README.md
";
        assert_eq!(
            Status::new(&test_status).unwrap(),
            Status {
                untracked: false,
                unstaged: true,
                staged: true,
                branch: String::from("master"),
                upstream: String::from("origin/master"),
                ahead: false,
                behind: false,
            }
        );
    }
}
