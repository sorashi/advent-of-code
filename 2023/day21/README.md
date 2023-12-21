# day21

The solution is slow and dirty.

| description | download | CRC32    | size  | lines | silver | gold            | CCM [ms]        | WCM [ms]        |
| ----------- | -------- | -------- | ----- | ----- | ------ | --------------- | --------------- | --------------- |
| input.txt   | -        | 829C5555 | 17KiB | 131   | 3632   | 600336060511101 | 1537.52 ± 67.87 | 1482.92 ± 69.25 |

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
