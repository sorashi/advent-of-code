using aoc2020;
using FileParser;

namespace Solution
{
    public class Day3 : IDay
    {
        public object Part1(IParsedFile file) {
            string[] rows = file.ToList<string>().ToArray();
            int x = 0, y = 0;
            var count = 0;
            while (y < rows.Length) {
                if (rows[y][x] == '#') count++;
                x += 3;
                if (x >= rows[y].Length) x %= rows[y].Length;
                y++;
            }

            return count;
        }

        struct HillPoint
        {
            public int X;
            public int Y;
            public static int HillWidth;

            public HillPoint(int x, int y) {
                this.X = x;
                this.Y = y;
            }
            public static implicit operator HillPoint((int, int) tuple) => new HillPoint(tuple.Item1, tuple.Item2);
            public static HillPoint operator +(HillPoint point1, HillPoint point2) => new HillPoint((point1.X + point2.X) % HillWidth, point1.Y + point2.Y);
        }

        public object Part2(IParsedFile file) {
            string[] rows = file.ToList<string>().ToArray();
            var slopes = new HillPoint[] {
                (1, 1), (3, 1), (5, 1), (7, 1), (1, 2) 
            };

            HillPoint.HillWidth = rows[0].Length;
            ulong count = 1;
            foreach (var slope in slopes) {
                var point = new HillPoint(0, 0);
                var trees = 0;
                while (point.Y < rows.Length) {
                    if (rows[point.Y][point.X] == '#') trees++;
                    point += slope;
                }
                count *= (ulong)trees;
            }

            return count;
        }
    }
}
