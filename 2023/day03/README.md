# day03

| description | download                        | CRC32    | size    | lines | silver    | gold        | CCM [ms]      | WCM [ms]     |
| ----------- | ------------------------------- | -------- | ------- | ----- | --------- | ----------- | ------------- | ------------ |
| original    | -                               | 6E270CBB | 19.3KiB | 140   | 525119    | 76504829    | 9.17 ± 0.19   | 0.84 ± 0.10  |
| bigboy.txt  | [here](https://0x0.st/Hxtu.txt) | 4B514B9F | 23.8MiB | 5k    | 258006204 | 17158526595 | 109.89 ± 1.33 | 93.76 ± 1.86 |

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
