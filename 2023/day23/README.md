# day23

The solution is a brute-force of the longest path on a simplified graph.

However, given the specific nature of the graph, I'd guess there exists a polynomial solution.
For example, by getting rid of cycles, we could get an acyclic graph, for which there is a linear time solution.

| description | download | CRC32   | size  | lines | silver | gold | CCM [ms]         | WCM [ms]         |
| ----------- | -------- | ------- | ----- | ----- | ------ | ---- | ---------------- | ---------------- |
| input.txt   | -        | C9C7380 | 20KiB | 141   | 2502   | 6726 | 5649.56 ± 325.09 | 5451.85 ± 304.38 |

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
