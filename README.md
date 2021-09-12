# rusty-journal
## To-Do List Command Line tool using rust

## Features
- Add tasks 
- Delete tasks
- List out all the tasks

# Usage 

Clone the repo

```sh
cd rusty-journal

cargo build
```

```
$ target/debug/rusty-journal -h
Rusty Journal 0.1.0
A command line to-do app written in Rust

USAGE:
    rusty-journal [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-file <journal-file>    use a different journal file

SUBCOMMANDS:
    add     write tasks to journal file
    done    Remove an entry from the journal file by position
    help    Prints this message or the help of the given subcommand(s)
    list    List all the task in the journal file

```

