# day07

| description | download                                        | CRC32    | size  | lines | silver | gold             | CCM [ms]     | WCM [ms]     |
| ----------- | ----------------------------------------------- | -------- | ----- | ----- | ------ | ---------------- | ------------ | ------------ |
| input.txt   | -                                               | AC70A05D | 20KiB | 142   | 1524   | 32982105837605   | 11.75 ± 0.16 | 0.40 ± 0.04  |
| bigboy.txt  | [lwa559.7z](https://files.catbox.moe/lwa559.7z) | 25529F1F | 24MiB | 5002  | 838    | 2396529680880047 | 27.86 ± 1.57 | 12.09 ± 0.28 |

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
