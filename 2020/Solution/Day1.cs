using System;
using System.Linq;
using FileParser;

namespace aoc2020
{
    public class Day1 : IDay
    {
        public object Part1(IParsedFile file) {
            var numbers = file.ToList<int>();
            var match = numbers.First(number => numbers.IndexOf(2020 - number) != -1);
            return match * (2020 - match);
        }

        public object Part2(IParsedFile file) {
            var numbers = file.ToList<int>();
            for (int x = 0; x < numbers.Count; x++)
            for (int y = x + 1; y < numbers.Count; y++)
            for (int z = y + 1; z < numbers.Count; z++)
                if (numbers[x] + numbers[y] + numbers[z] == 2020)
                    return (numbers[x] * numbers[y] * numbers[z]);

            throw new Exception();
        }
    }
}
