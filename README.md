# goban

**goban** is a CLI tool to run commands for all combinations of parameters.

## Getting started
Save **parameters.json** in the current directory.
```json
{
	"fruit": ["banana", "apple"],
	"N": [ 10, 100, 1000 ]
}
```

and run the following command.
```sh
$ goban "echo Please give me {{N}} {{fruit}} "
```

Then, the result is
```

[1 / 4]
Parameters: {"N": "10", "fruit": "banana"}
$ echo Please give me 10 banana
Please give me 10 banana
[exit status: 0]

[2 / 4]
Parameters: {"N": "100", "fruit": "banana"}
$ echo Please give me 100 banana
Please give me 100 banana
[exit status: 0]

[3 / 4]
Parameters: {"fruit": "apple", "N": "10"}
$ echo Please give me 10 apple
Please give me 10 apple
[exit status: 0]

[4 / 4]
Parameters: {"fruit": "apple", "N": "00"}
$ echo Please give me 100 apple
Please give me 100 apple
[exit status: 0]
```

## Usage
goban 0.1.0

USAGE:
    goban [OPTIONS] <COMMAND>

ARGS:
    <COMMAND>    Command to execution

OPTIONS:
    -f, --filepath <FILEPATH>    Path to a parameter file [default: parameters.json]
    -h, --help                   Print help information
    -V, --version                Print version information
