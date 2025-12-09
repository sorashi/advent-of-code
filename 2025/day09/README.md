# day09

| description | download | CRC32    | size | lines | silver     | gold       | CCM [ms]     | WCM [ms]     |
| ----------- | -------- | -------- | ---- | ----- | ---------- | ---------- | ------------ | ------------ |
| input.txt   | -        | A4BD2774 | 6KiB | 496   | 4758598740 | 1474699155 | 77.29 ± 1.51 | 65.79 ± 1.15 |

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
