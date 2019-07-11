# hist2d
Generates a 2d histogram from paired count data.

```
USAGE:
    hist2d [OPTIONS] <INPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --nbins <N>          Number of bins in each dimension (memory use will be O(N^2)) [default: 1000]
    -s, --sep <SEPARATOR>    Separator for output [default:  ]

ARGS:
    <INPUT>    Input file (with counts in 2nd and 3rd columns)
```
