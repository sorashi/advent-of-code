# day05

| description | download                                        | CRC32    | size  | lines   | silver | gold            | CCM [ms]      | WCM [ms]      |
| ----------- | ----------------------------------------------- | -------- | ----- | ------- | ------ | --------------- | ------------- | ------------- |
| input.txt   | -                                               | FE4D6594 | 21KiB | 1178    | 707    | 361615643045059 | 12.90 ± 0.74  | 0.45 ± 0.04   |
| bigboy.txt  | [oj8a57.7z](https://files.catbox.moe/oj8a57.7z) | 980E1308 | 86MiB | 4000001 | 1554   | 450048181074    | 250.98 ± 4.74 | 231.74 ± 5.35 |

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
