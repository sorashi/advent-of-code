# day06

| description | download | CRC32    | size | lines | silver | gold     | CCM [ms]    | WCM [ms]    |
| ----------- | -------- | -------- | ---- | ----- | ------ | -------- | ----------- | ----------- |
| original    | -        | 222D1A0F | 74B  | 2     | 503424 | 32607562 | 7.85 ± 2.21 | 0.44 ± 0.08 |

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
