# day07

| description | download | CRC32    | size  | lines | silver           | gold             | CCM [ms]        | WCM [ms]       |
| ----------- | -------- | -------- | ----- | ----- | ---------------- | ---------------- | --------------- | -------------- |
| input.txt   | -        | 70F941AF | 10KiB | 1k    | 253910319        | 254083736        | 10.90 ± 0.57    | 4.34 ± 0.09    |
| bigboy.txt  | -        | 66E0CB07 | 2MiB  | 200k  | 6678229757944529 | 7246011492564128 | 1538.31 ± 12.72 | 1465.15 ± 5.37 |

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
