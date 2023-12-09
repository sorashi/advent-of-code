# day09

| description | download                | CRC32    | size  | lines | silver           | gold    | CCM [ms]     | WCM [ms]     |
| ----------- | ----------------------- | -------- | ----- | ----- | ---------------- | ------- | ------------ | ------------ |
| input.txt   | -                       | D034A98A | 20KiB | 200   | 1479011877       | 973     | 7.39 ± 0.22  | 0.51 ± 0.07  |
| bigboy.txt  | https://0x0.st/H3SN.txt | 23DB4799 | 17MiB | 80k   | 8137764536324356 | 7508194 | 81.56 ± 1.38 | 67.78 ± 1.42 |

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
