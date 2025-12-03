# day03

| description | download                                        | CRC32    | size  | lines | silver | gold            | CCM [ms]      | WCM [ms]     |
| ----------- | ----------------------------------------------- | -------- | ----- | ----- | ------ | --------------- | ------------- | ------------ |
| input.txt   | -                                               | 98CBB56A | 20KiB | 200   | 16858  | 167549941654721 | 11.81 ± 0.26  | 0.45 ± 0.05  |
| bigboy.txt  | [klc5b5.7z](https://files.catbox.moe/klc5b5.7z) | 98A126D8 | 95MiB | 1k    | 83984  | 847316731798752 | 111.98 ± 1.50 | 90.62 ± 1.19 |

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
