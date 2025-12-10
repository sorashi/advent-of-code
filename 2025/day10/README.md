# day10

| description | download | CRC32    | size  | lines | silver | gold  | CCM [ms]      | WCM [ms]      |
| ----------- | -------- | -------- | ----- | ----- | ------ | ----- | ------------- | ------------- |
| input.txt   | -        | 3EA6DBA1 | 19KiB | 197   | 522    | 18105 | 272.03 ± 4.70 | 227.59 ± 5.06 |

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
