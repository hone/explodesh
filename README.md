# explodesh
Converts a TOML file into a set of file and folders, so it can be easier to manipulate inside of shell.

```sh
$ target/debug/explodesh  -h
explodesh 0.1
Terence Lee <hone02@gmail.com>
Tool for converting TOML files to a set key/value files/folders

USAGE:
    explodesh <cmd> <source> <destination>

ARGS:
    <cmd>            'explode' take a TOML file and convert to a filesystem layout. 'implode'
                     will take a filesystem layout and construct a TOML file [possible values:
                     explode, implode]
    <source>         Path to the source input
    <destination>    Path to where the output is written

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

Taking a sample `foo.toml`:

```TOML
foo = "hello"
bar = 1
baz = true

[[browsers]]
id = "firefox"

[[browsers]]
id = "chrome"

[table]
field1 = "value1"
field2 = "value2"
```

Exploding this table would result in a file structure:

```sh
$ explodesh explode ./foo.toml /tmp/foo
$ tree /tmp/foo
/tmp/foo
├── bar
├── baz
├── browsers
│   ├── 0
│   │   └── id
│   └── 1
│       └── id
├── foo
└── table
    ├── field1
    └── field2
$ echo "$(cat /tmp/foo/bar)"
1
$ echo $(cat /tmp/foo/foo)"
"hello"
```

To convert this back into a TOML file:

```sh
$ explodesh implode /tmp/foo foo2.toml
```
