# git-status-line

[![Build Status](https://travis-ci.org/chasetopher/git-status-line.svg?branch=master)](https://travis-ci.org/chasetopher/git-status-line)

## Install

```bash
brew install https://gist.githubusercontent.com/chasetopher/0065ce4f47aa165b813488b159ca40ef/raw/git-status-line.rb
```

## Usage

```bash
$ PS1='$(git status-line)$ '
$ cd my_git_project
[master ea9cf71] $ touch foo.txt
[master ea9cf71 ?] $ git add foo.txt
[master ea9cf71 +] $ git commit -m 'Add foo'
[master 8d442f9] Add foo
 1 file changed, 0 insertions(+), 0 deletions(-)
 create mode 100644 foo.txt
[master 8d442f9] $
```
