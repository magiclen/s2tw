s2tw
====================

[![CI](https://github.com/magiclen/s2tw/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/s2tw/actions/workflows/ci.yml)

A simple tool for converting Simple Chinese to Traditional Chinese(TW).

## Help

```
EXAMPLES:
s2tw                                # Convert each of input lines from Simple Chinese to Traditional Chinese
s2tw chs.txt cht.txt                # Convert chs.txt (in Simple Chinese) to cht.txt (in Traditional Chinese)
s2tw a.chs.txt                      # Convert a.chs.txt (in Simple Chinese) to a.cht.txt (in Traditional Chinese)

USAGE:
    s2tw [OPTIONS] [ARGS]

ARGS:
    <S_PATH>     Assign the path of your Simple Chinese document. It should be a file path.
    <TW_PATH>    Assign the path of your Traditional Chinese document. It should be a file path.

OPTIONS:
    -f, --force      Force to output if the output file exists.
    -h, --help       Print help information
    -V, --version    Print version information
```

## License

[MIT](LICENSE)