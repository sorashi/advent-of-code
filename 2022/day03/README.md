# day03

| description | download | CRC32    | size  | lines | silver | gold | CCM [ms]    | WCM [ms]    |
| ----------- | -------- | -------- | ----- | ----- | ------ | ---- | ----------- | ----------- |
| input       | -        | B71E5634 | 10KiB | 300   | 7581   | 2525 | 7.27 ± 0.30 | 0.57 ± 0.08 |

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
