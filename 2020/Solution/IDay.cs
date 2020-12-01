using FileParser;

namespace aoc2020
{
    public interface IDay
    {
        public object Part1(IParsedFile file);

        public object Part2(IParsedFile file);
    }
}
