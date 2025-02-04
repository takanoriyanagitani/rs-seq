# Simple Benchmark

## macOS

| tool        | user | sys | total | CPU% | rate     | ratio |
|:-----------:|:----:|:---:|:-----:|:----:|:--------:|:-----:|
| native      |  2.6 | 0.1 |  2.7  | 100% | 570 MB/s | 5.5x  |
| wasi/wazero |  7.8 | 0.2 |  7.9  | 113% | 196 MB/s | 1.9x  |
| (seq)       | 19.2 | 0.2 | 19.3  | 100% | 103 MB/s | (1x)  |
