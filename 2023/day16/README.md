# day16

| description | download | CRC32    | size  | lines | silver | gold | CCM [ms]      | WCM [ms]      |
| ----------- | -------- | -------- | ----- | ----- | ------ | ---- | ------------- | ------------- |
| input.txt   | -        | 4BC7F504 | 12KiB | 110   | 7884   | 8185 | 258.14 ± 2.93 | 250.70 ± 0.23 |

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
