# day04

| description | download                                        | CRC32    | size  | lines | silver  | gold    | CCM [ms]       | WCM [ms]       |
| ----------- | ----------------------------------------------- | -------- | ----- | ----- | ------- | ------- | -------------- | -------------- |
| input.txt   | -                                               | 99C543FC | 19KiB | 138   | 1549    | 8887    | 13.81 ± 0.19   | 2.86 ± 0.18    |
| bigboy.txt  | [pj1x30.7z](https://files.catbox.moe/pj1x30.7z) | CE95D35B | 11MiB | 3333  | 2023510 | 5520583 | 432.89 ± 30.69 | 419.59 ± 35.20 |

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
