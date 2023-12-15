# day15

| description | download | CRC32    | size  | lines | silver | gold   | CCM [ms]    | WCM [ms]    |
| ----------- | -------- | -------- | ----- | ----- | ------ | ------ | ----------- | ----------- |
| input.txt   | -        | 4D805C79 | 22KiB | 1     | 517315 | 247763 | 8.02 ± 0.50 | 0.59 ± 0.08 |

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
