# day04

| description | download                         | CRC32    | size  | lines | silver  | gold    | CCM [ms]      | WCM [ms]      |
| ----------- | -------------------------------- | -------- | ----- | ----- | ------- | ------- | ------------- | ------------- |
| input.txt   | -                                | F56FD1C4 | 23KiB | 202   | 23235   | 5920640 | 6.97 ± 0.15   | 0.53 ± 0.07   |
| bigboy.txt  | [7z](https://0x0.st/HxE7.txt.7z) | 329E1755 | 43MiB | 200k  | 1475828 | 211552  | 266.58 ± 5.23 | 241.22 ± 1.08 |

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
