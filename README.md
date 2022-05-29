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

## Discription
goban 0.1.0

Usage:
    goban [OPTIONS] <COMMAND>

Args:
    <COMMAND>    Command to execution

Options:
    -f, --filepath <FILEPATH>    Path to a parameter file [default: parameters.json]
    -h, --help                   Print help information
    -V, --version                Print version information

### File format
- The format of the parameter file must be JSON.
- The parameter file must be JSON Object.
- The values in the JSON Object must be JSON arrays.
- The JSON arrays can have any valid JSON values.

### Command template format
goban use **Handlebars**, which is a simple template engine.
The goban's template format is following to Handlebars's one.
See https://handlebarsjs.com/guide/expressions.html.

