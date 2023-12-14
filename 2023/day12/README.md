# day12

| description | download | CRC32    | size  | lines | silver | gold            | CCM [ms]     | WCM [ms]     |
| ----------- | -------- | -------- | ----- | ----- | ------ | --------------- | ------------ | ------------ |
| input.txt   | -        | C8A705FE | 22KiB | 1k    | 7718   | 128741994134728 | 48.40 ± 0.82 | 42.83 ± 0.64 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

The solution is not optimal.

Given one line of input, let the length of spring conditions be $n$ and let the groups be a sequence $G$.
For one line this line the complexity of this solution is $\mathcal O(n |G| \max G)$.
I've heard there exists a solution with $\mathcal O(n |G|)$.

[This bigboy](https://files.catbox.moe/h3lff6.txt) takes over 3 minutes (iterating over 3 lines per second).

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
