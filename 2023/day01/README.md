# Day 1

Bigboy vstupy

| popis      | velikost | řádků | silver   | gold     | download                       |    CCM [ms] |        WCM [ms] |
| ---------- | -------- | ----- | -------- | -------- | ------------------------------ | ----------: | --------------: |
| originální | 21k      | 1000  | 55712    | 55413    | -                              |   7.7 ± 0.1 |  0.569 ± 0.0614 |
| "do výšky" | 21M      | 1M    | 55022487 | 55015199 | [zde](https://0x0.st/HxMf.txt) | 168.6 ± 4.0 |     157.8 ± 1.4 |
| "do šířky" | 75k      | 4     | 231      | 229      | [zde](https://bpa.st/raw/BEEQ) |   7.8 ± 0.2 | 0.5376 ± 0.0704 |

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
