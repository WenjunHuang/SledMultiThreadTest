a simple sled multithreaded test.

Sled Multi-threaded test

```
USAGE:
    SledMultithreadTest <INPUT> -m <mode> -t <threads>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m <mode>           Test mode
       w                write only
       r                read only
       wr               write then read
    -t <threads>        threads count

ARGS:
    <INPUT>    Sled directory
```

for example:
* ./SledMultithreadTest test_dir -m w -t 2: write to test_dir with 2 threads
* ./SledMultithreadTest test_dir -m r -t 2: read from test_dir with 2 threads
* ./SledMultithreadTest test_dir -m wr -t 2: first write to then read from test_dir with 2 threads
