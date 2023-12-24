# day24

There is probably a solution using linear algebra, but I got fed up while looking for it and used a SAT solver instead (Z3).

| description | download | CRC32    | size  | lines | silver | gold            | CCM [ms]        | WCM [ms]        |
| ----------- | -------- | -------- | ----- | ----- | ------ | --------------- | --------------- | --------------- |
| input.txt   | -        | C4B27154 | 19KiB | 300   | 11246  | 716599937560103 | 1718.19 ± 64.10 | 1646.40 ± 62.70 |

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
