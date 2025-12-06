# day06

| description | download | CRC32    | size  | lines | silver        | gold          | CCM [ms]     | WCM [ms]    |
| ----------- | -------- | -------- | ----- | ----- | ------------- | ------------- | ------------ | ----------- |
| input.txt   | -        | EC763344 | 18KiB | 5     | 4878670269096 | 8674740488592 | 12.00 ± 0.26 | 0.50 ± 0.05 |

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
