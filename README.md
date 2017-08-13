# git-status-line

[![Build Status](https://travis-ci.org/chasetopher/git-status-line.svg?branch=master)](https://travis-ci.org/chasetopher/git-status-line)

## Install

```bash
brew install chasetopher/personal/git-status-line
```

## Usage

```
$ PS1='$(git status-line)$ '
$ cd my_git_project
[master ea9cf71] $ touch foo.txt
[master ea9cf71 ?] $ git add foo.txt
[master ea9cf71 +] $ git commit -m 'Add foo'
[master 8d442f9] Add foo
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 foo.txt
[master 8d442f9] $ echo >> foo.txt
[master 8d442f9 *] $ touch bar.txt
[master 8d442f9 *?] $ git add .
[master 8d442f9 +] $ git commit -m 'foobar'
[master 368fcec] foobar
 2 files changed, 1 insertion(+)
 create mode 100644 bar.txt
[master 368fcec] $
```

## Performance

Here's how `git-status-line` compares to `__git_ps1` (provided by `git`) and a old Perl script I used to use called `git-prompt`:

![img](https://user-images.githubusercontent.com/606164/29245191-4acb1a8a-7f87-11e7-88da-0553dbf1e754.png)

## TODO

- Add color
- Customization of colors/symbols/format
- Indication of merging or rebasing
