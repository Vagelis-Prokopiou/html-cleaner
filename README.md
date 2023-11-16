html-cleaner: A command line utility that strips all html tags, leaving the pure text.

# Usage

```shell
/absolute/path/to/html-cleaner.exe /absolute/path/to/file.txt
```

The input file should be a `txt` file, with one entry per line. Example:
```
<h1>foo</h1>
<div><h1>foo bar baz</h1></div>
...
```

Non valid html structure should not create a problem. Example:
```
<div><h1>foo bar baz</h1></div></div></div> => foo bar baz
```

The result is written to `html-cleaner-result.txt`, in the current path.
