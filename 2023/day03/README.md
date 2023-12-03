# Day 3

Bez paralelizace though xd

| popis      | velikost | řádků | silver    | gold        | download                       |    CCM [ms] |        WCM [ms] |
| ---------- | -------- | ----- | --------- | ----------- | ------------------------------ | ----------: | --------------: |
| originální | 20k      | 140   | 525119    | 76504829    | -                              |   9.2 ± 0.3 | 0.8415 ± 0.0925 |
| bigboy     | 25M      | 5000  | 258006204 | 17158526595 | [zde](https://0x0.st/Hxtu.txt) | 108.5 ± 1.0 |      95.4 ± 0.9 |

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
