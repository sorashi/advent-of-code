# day24

There is probably a solution using linear algebra, but I got fed up while looking for it and used an SAT solver instead ([Z3](https://microsoft.github.io/z3guide/)).

| description | download | CRC32    | size  | lines | silver | gold            | CCM [ms]       | WCM [ms]       |
| ----------- | -------- | -------- | ----- | ----- | ------ | --------------- | -------------- | -------------- |
| input.txt   | -        | C4B27154 | 19KiB | 300   | 11246  | 716599937560103 | 750.76 ± 20.80 | 734.42 ± 26.92 |

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
