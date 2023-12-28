# day02

| description | download | CRC32    | size  | lines | silver | gold  | CCM [ms]    | WCM [ms]    |
| ----------- | -------- | -------- | ----- | ----- | ------ | ----- | ----------- | ----------- |
| input       | -        | AAD95C8B | 10KiB | 2500  | 15572  | 16098 | 7.28 ± 0.17 | 0.41 ± 0.05 |

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
