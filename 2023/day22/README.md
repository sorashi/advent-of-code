# day22

| description | download | CRC32 | size | lines | silver | gold | CCM [ms] | WCM [ms] |
| ----------- | -------- | ----- | ---- | ----- | ------ | ---- | -------- | -------- |
| input.txt | -       | 40DE5B2 |  22KiB | 1497 | 534 | 88156 | 692.39 ± 39.15 | 680.65 ± 36.84 |

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
