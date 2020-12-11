using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using aoc2020;
using FileParser;
using MoreLinq.Extensions;

namespace Solution
{
    public class Day9 : IDay
    {
        public object Part1(IParsedFile file) {
            var numbers = file.ToList<string>().Select(ulong.Parse).ToArray();
            for (int i = 25; i < numbers.Length; i++) {
                var seg = new ArraySegment<ulong>(numbers, i - 25, 25);
                if (GetSums(seg).All(x => x != numbers[i]))
                    return numbers[i];
            }
            throw new Exception();
        }

        public IEnumerable<ulong> GetSums(ArraySegment<ulong> seg) {
            for (int i = 0; i < seg.Count; i++)
            for (int j = i + 1; j < seg.Count; j++)
                yield return seg[i] + seg[j];
        }

        public object Part2(IParsedFile file) {
            const ulong invalid = 375054920UL;
            var numbers = file.ToList<string>().Select(ulong.Parse).ToArray();
            var prefix = new BigInteger[numbers.Length];
            prefix[0] = numbers[0];
            for (int i = 1; i < numbers.Length; i++) prefix[i] = prefix[i - 1] + numbers[i];
            for (int i = 0; i < numbers.Length; i++) {
                for (int j = i+1; j < numbers.Length; j++) {
                    if (prefix[j] - prefix[i] == invalid) {
                        var segment = numbers.Skip(i + 1).Take(j - i);
                        Debug.Assert(segment.Aggregate(0UL, (a,c) => a+c) == invalid);
                        return segment.Min() + segment.Max();
                    }
                }
            }

            return null;
        }
    }
}
