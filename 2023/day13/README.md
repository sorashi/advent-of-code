# day13

| description | download | CRC32 | size | lines | silver | gold | CCM [ms] | WCM [ms] |
| ----------- | -------- | ----- | ---- | ----- | ------ | ---- | -------- | -------- |
| input.txt | -       | E7B230F4 |  17KiB | 1365 | 33122 | 32312 | 6.64 ± 0.13 | 0.48 ± 0.06 |

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
