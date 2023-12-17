# day17

| description | download | CRC32    | size  | lines | silver | gold | CCM [ms]      | WCM [ms]      |
| ----------- | -------- | -------- | ----- | ----- | ------ | ---- | ------------- | ------------- |
| input.txt   | -        | DE31B6BA | 20KiB | 141   | 1246   | 1389 | 263.51 ± 5.90 | 247.80 ± 2.77 |

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
