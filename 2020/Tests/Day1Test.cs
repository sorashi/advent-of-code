using System;
using System.Collections.Generic;
using aoc2020;
using FileParser;
using Moq;
using Xunit;

namespace Tests
{
    public class Day1Test
    {
        private readonly Mock<IParsedFile> fileMock = new Mock<IParsedFile>();

        public Day1Test() {
            fileMock.Setup(x => x.ToList<int>(null))
                .Returns(new List<int> { 1721, 979, 366, 299, 675, 1456 });
        }
        [Fact]
        public void Part1_Test() {
            var day = new Day1();
            Assert.Equal(514579, day.Part1(fileMock.Object));
        }

        [Fact]
        public void Part2_Test() {
            var day = new Day1();
            Assert.Equal(241861950, day.Part2(fileMock.Object));
        }
    }
}
