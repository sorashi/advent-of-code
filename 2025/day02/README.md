# day02

Could definitely be optimized further.

| description | download                                        | CRC32    | size  | lines | silver          | gold            | CCM [ms]       | WCM [ms]       |
| ----------- | ----------------------------------------------- | -------- | ----- | ----- | --------------- | --------------- | -------------- | -------------- |
| input       | -                                               | C44839A5 | 442B  | 1     | 38437576669     | 49046150754     | 20.05 ± 0.26   | 8.34 ± 0.40    |
| bigboy.txt  | [fnn0h2.7z](https://files.catbox.moe/fnn0h2.7z) | 9790BFBE | 10MiB | 1     | 245376725123244 | 250966399724248 | 2190.23 ± 5.08 | 2172.36 ± 7.99 |

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
