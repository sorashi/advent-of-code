# day08

| description | download                | CRC32    | size  | lines | silver  | gold           | CCM [ms]     | WCM [ms]     |
| ----------- | ----------------------- | -------- | ----- | ----- | ------- | -------------- | ------------ | ------------ |
| input.txt   | -                       | 72FF4F44 | 13KiB | 760   | 18023   | 14449445933179 | 7.73 ± 0.38  | 1.25 ± 0.11  |
| bigboy.txt  | https://0x0.st/H3cq.txt | 317D6AA9 | 1MiB  | 9     | 1488003 | 2214154416012  | 37.47 ± 0.40 | 30.18 ± 0.86 |

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
