# day19

| description | download | CRC32    | size  | lines | silver | gold            | CCM [ms]    | WCM [ms]    |
| ----------- | -------- | -------- | ----- | ----- | ------ | --------------- | ----------- | ----------- |
| input.txt   | -        | 20FC38C3 | 18KiB | 715   | 319295 | 110807725108076 | 7.73 ± 0.29 | 0.65 ± 0.07 |

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
