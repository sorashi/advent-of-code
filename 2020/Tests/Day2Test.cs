using FileParser;
using Solution;
using Xunit;

namespace Tests
{
    public class Day2Test
    {
        private readonly IParsedFile fileMock;

        public Day2Test() {
            fileMock = new ParsedFile(new [] {
                new ParsedLine("1-3 a: abcde".Split(' ')),
                new ParsedLine("1-3 b: cdefg".Split(' ')),
                new ParsedLine("2-9 c: ccccccccc".Split(' '))
            });
        }
        [Fact]
        public void Part1_Test() {
            var day = new Day2();
            Assert.Equal(2, day.Part1(fileMock));
        }
        [Fact]
        public void Part2_Test() {
            var day = new Day2();
            Assert.Equal(1, day.Part2(fileMock));
        }
    }
}
