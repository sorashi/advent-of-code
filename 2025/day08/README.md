# day08

| description | download | CRC32 | size | lines | silver | gold | CCM [ms] | WCM [ms] |
| ----------- | -------- | ----- | ---- | ----- | ------ | ---- | -------- | -------- |
| input.txt | -       | 8DF41638 |  17KiB | 1k | 68112 | 44543856 | 50.59 ± 0.72 | 39.65 ± 1.47 |

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
