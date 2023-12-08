# day08

| description | download                | CRC32    | size  | lines | silver  | gold           | CCM [ms]     | WCM [ms]     |
| ----------- | ----------------------- | -------- | ----- | ----- | ------- | -------------- | ------------ | ------------ |
| input.txt   | -                       | 72FF4F44 | 13KiB | 760   | 18023   | 14449445933179 | 9.16 ± 0.97  | 2.30 ± 0.12  |
| bigboy.txt  | https://0x0.st/H3cq.txt | 317D6AA9 | 1MiB  | 9     | 1488003 | 2214154416012  | 75.35 ± 0.73 | 66.37 ± 0.70 |

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
