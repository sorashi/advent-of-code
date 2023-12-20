# day20

| description | download | CRC32    | size | lines | silver    | gold            | CCM [ms]     | WCM [ms]    |
| ----------- | -------- | -------- | ---- | ----- | --------- | --------------- | ------------ | ----------- |
| input.txt   | -        | 75AFA58E | 792B | 58    | 791120136 | 215252378794009 | 14.55 ± 0.26 | 7.29 ± 0.39 |

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
