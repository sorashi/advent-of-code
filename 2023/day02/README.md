# Day 2

Bez paralelizace though xd

| popis      | velikost | řádků | silver      | gold       | download                                  |  CCM [ms] |        WCM [ms] |
| ---------- | -------- | ----- | ----------- | ---------- | ----------------------------------------- | --------: | --------------: |
| originální | 10k      | 100   | 2149        | 71274      | -                                         | 8.4 ± 0.2 | 0.9572 ± 0.0905 |
| bigboy     | 83M      | 1M    | 71327370192 | 2221048073 | [zde](https://files.catbox.moe/bvnzno.7z) | 1435 ± 18 |       1378 ± 34 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

## Benchmarky

Cold-cache

```bash
sudo -v
hyperfine --input input.txt --prepare 'sync; echo 3 | sudo tee /proc/sys/vm/drop_caches' ./solution
```

Warm-cache

```bash
hyperfine --input input.txt -N -w 5 ./solution
```
