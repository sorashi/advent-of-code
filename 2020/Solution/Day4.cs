using aoc2020;
using FileParser;

namespace Solution
{
    public class Day4 : IDay
    {
        public object Part1(IParsedFile file) {
            file.NextLine();
            file.NextLine();
            file.NextLine();
            var a = file.NextLine();
            return a;
        }

        public object Part2(IParsedFile file) {
            return null;
        }
    }
}
