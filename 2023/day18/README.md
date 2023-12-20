# day18

I think my solution is not general.
Specifically, it finds a hull around the holes and then uses the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula) to find its volume.
There are some edge cases I thought I would need to cover, but they do not appear in my input and their absence is not guaranteed by the task description.

I thought about these, but there might be more:

- what if the loop goes counter-clockwise,
- what if there are two digging instructions with the same direction (for example dig 4 right, then dig 4 right again).

| description | download                            | CRC32    | size   | lines   | silver    | gold                | CCM [ms]       | WCM [ms]       |
| ----------- | ----------------------------------- | -------- | ------ | ------- | --------- | ------------------- | -------------- | -------------- |
| input.txt   | -                                   | D9E11995 | 10KiB  | 746     | 40761     | 106920098354636     | 7.87 ± 0.22    | 0.45 ± 0.07    |
| bigboy.txt  | https://files.catbox.moe/ohhrln.txt | 896A4F0A | 126MiB | 9452532 | 217197721 | 1477260902388793750 | 854.06 ± 11.71 | 837.64 ± 11.95 |

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
