using System;
using System.Collections.Generic;
using System.Linq;
using aoc2020;
using FileParser;

namespace Solution
{
    public class Day10 : IDay
    {
        public object Part1(IParsedFile file) {
            var adapters = file.ToList<int>();
            adapters.Add( adapters.Max() + 3);
            adapters.Add(0);
            adapters.Sort();
            var enumer = adapters.Zip(adapters.Skip(1)).Select((x) => x.Second - x.First);
            return enumer.Count(x => x == 1)* enumer.Count(x => x == 3);
        }
        public object Part2(IParsedFile file) {
            var adapters = file.ToList<int>();
            adapters.Add(0);
            adapters.Sort();
            adapters.Add(adapters[^1] + 3);
            var arrangements = new ulong[adapters[^1] + 1];
            arrangements[^1] = 1;
            for (int i = adapters.Count - 2; i >= 0; i--)
            for (int j = 1; j <= 3; j++)
                if (adapters.Contains(adapters[i] + j))
                    arrangements[adapters[i]] += arrangements[adapters[i] + j];
            return arrangements[0];
        }
    }
}
