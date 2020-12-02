using System.Linq;
using aoc2020;
using FileParser;

namespace Solution
{
    public class Day2 : IDay
    {
        public object Part1(IParsedFile file) {
            var valid = 0;
            while (!file.Empty) {
                var line = file.NextLine();
                var minmax = line.NextElement<string>().Split('-').Select(int.Parse).ToArray();
                var ch = line.NextElement<string>()[0];
                var str = line.NextElement<string>();
                var cnt = str.Count(x => x == ch);
                if (cnt >= minmax[0] && cnt <= minmax[1]) valid++;
            }

            return valid;
        }

        public object Part2(IParsedFile file) {
            var valid = 0;
            while (!file.Empty) {
                var line = file.NextLine();
                var rule = line.NextElement<string>().Split('-').Select(int.Parse).ToArray();
                var ch = line.NextElement<string>()[0];
                var str = line.NextElement<string>();
                if (rule.Count(x => ch == str[x - 1]) == 1) valid++;
            }

            return valid;
        }
    }
}
