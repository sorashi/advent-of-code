# day15

| description | download                            | CRC32    | size   | lines | silver   | gold   | CCM [ms]        | WCM [ms]        |
| ----------- | ----------------------------------- | -------- | ------ | ----- | -------- | ------ | --------------- | --------------- |
| input.txt   | -                                   | 4D805C79 | 22KiB  | 1     | 517315   | 247763 | 7.48 ± 0.60     | 0.59 ± 0.08     |
| bigboy.txt  | https://files.catbox.moe/s1jsrm.txt | D9AC021B | 680KiB | 1     | 13134060 | 1      | 2285.74 ± 29.56 | 2296.01 ± 54.86 |

- CCM = Cold-Cache Mean
- WCM = Warm-Cache Mean

The solution is not optimal.
As you can see, the bigboy runs over 2 seconds.
It could probably be made more fast by using a hand-crafted well-suited data structure instead of the currently used `Vec`.

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
