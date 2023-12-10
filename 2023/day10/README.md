# day10

| description | download | CRC32    | size  | lines | silver | gold | CCM [ms]     | WCM [ms]     |
| ----------- | -------- | -------- | ----- | ----- | ------ | ---- | ------------ | ------------ |
| input.txt   | -        | 8BA7694D | 19KiB | 140   | 6649   | 601  | 43.64 ± 0.61 | 37.88 ± 0.21 |

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
