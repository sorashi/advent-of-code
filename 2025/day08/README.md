# day08

Kruskal with naive union-find.
The better union-find would increase memory allocation without much performance gain even for the bigboy, because the bottleneck is probably generating all $\frac{n(n-1)}{2}$ edges and sorting them. Bigboy performance could probably be imporoved by either Borůvka in parallel, or combining with another alg to avoid generating all edges and then sorting them as is required by Kruskal.

| description | download                                          | CRC32    | size   | lines | silver | gold           | CCM [ms]     | WCM [ms]     |
| ----------- | ------------------------------------------------- | -------- | ------ | ----- | ------ | -------------- | ------------ | ------------ |
| input.txt   | -                                                 | 8DF41638 | 17KiB  | 1k    | 68112  | 44543856       | 50.47 ± 0.57 | 39.37 ± 1.30 |
| bigboy.txt  | [q6qnl1.txt](https://files.catbox.moe/q6qnl1.txt) | 4FA300B0 | 388KiB | 16k   | 64     | 97529786074100 | -            | ~15000       |

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
