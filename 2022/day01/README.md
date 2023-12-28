# day01

| description | download | CRC32    | size  | lines | silver | gold   | CCM [ms]    | WCM [ms]    |
| ----------- | -------- | -------- | ----- | ----- | ------ | ------ | ----------- | ----------- |
| input       | -        | FE474FAE | 10KiB | 2248  | 69289  | 205615 | 6.91 ± 0.25 | 0.42 ± 0.08 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

## Benchmarks

hyperfine 1.18.0

Cold-cache:

```bash
hyperfine --input input.txt --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' ./solution
```

Warm-cache:

```bash
hyperfine --input input.txt -N -w 5 ./solution
```
