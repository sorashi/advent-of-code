# day01

| description | download                            | CRC32    | size  | lines | silver | gold                    | CCM [ms]     | WCM [ms]     |
| ----------- | ----------------------------------- | -------- | ----- | ----- | ------ | ----------------------- | ------------ | ------------ |
| input       | -                                   | A3308BB3 | 18KiB | 4577  | 1139   | 6684                    | 12.33 ± 0.22 | 0.52 ± 0.06  |
| bigboy      | [KJ12.txt](https://0x0.st/KJ12.txt) | E89564CF | 20MiB | 1M    | 257513 | 49952593196376656314868 | 58.57 ± 0.78 | 45.14 ± 0.89 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

## Benchmarks

hyperfine 1.20.0

Cold-cache:

```bash
hyperfine --input input.txt --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' ./solution
```

Warm-cache:

```bash
hyperfine --input input.txt -N -w 5 ./solution
```
