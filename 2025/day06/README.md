# day06

| description | download                                        | CRC32    | size  | lines | silver        | gold          | CCM [ms]      | WCM [ms]      |
| ----------- | ----------------------------------------------- | -------- | ----- | ----- | ------------- | ------------- | ------------- | ------------- |
| input.txt   | -                                               | EC763344 | 18KiB | 5     | 4878670269096 | 8674740488592 | 11.46 ± 0.13  | 0.50 ± 0.05   |
| bigboy.txt  | [y6nuey.7z](https://files.catbox.moe/y6nuey.7z) | 615A6587 | 31MiB | 6     | 117309571763  | 163990321309  | 132.51 ± 1.77 | 116.23 ± 1.86 |

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
