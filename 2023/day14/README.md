# day14

| description | download                          | CRC32    | size  | lines | silver | gold   | CCM [ms]     | WCM [ms]    |
| ----------- | --------------------------------- | -------- | ----- | ----- | ------ | ------ | ------------ | ----------- |
| input.txt   | -                                 | 6885A7A6 | 10KiB | 100   | 111979 | 102055 | 16.25 ± 0.19 | 9.39 ± 0.20 |
| bigboy.txt  | https://pastebin.pl/view/7be0ebca | E3A67310 | 4KiB  | 40    | 75     | 190    | 7.31 ± 0.13  | 0.62 ± 0.07 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

The solution could be more optimized.
Another [bigboy](https://paste.sh/3faVocvE#J5dmnzZKkUaA1MAwHnrSoxv-) runs for 9 seconds.

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
