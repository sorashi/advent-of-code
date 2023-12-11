# day11

| description | download                | CRC32    | size   | lines | silver    | gold          | CCM [ms]     | WCM [ms]     |
| ----------- | ----------------------- | -------- | ------ | ----- | --------- | ------------- | ------------ | ------------ |
| input.txt   | -                       | 48443410 | 19KiB  | 140   | 9947476   | 519939907614  | 9.76 ± 0.21  | 2.66 ± 0.33  |
| bigboy.txt  | https://0x0.st/H3tu.txt | AE7E7764 | 189KiB | 440   | 386232597 | 8772424688485 | 36.49 ± 0.99 | 29.33 ± 0.66 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

The bigboy is small (1600 galaxies) and does not reflect the fact that this solution is not at all optimal.
Another bigboy: https://files.catbox.moe/admr52.txt

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
