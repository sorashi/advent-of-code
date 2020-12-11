using System;
using System.Collections.Generic;
using aoc2020;
using FileParser;
using System.Linq;

namespace Solution
{
    public class Day11 : IDay
    {
        public object Part1(IParsedFile file) {
            var height = file.Count;
            var line = file.NextLine().ToSingleString();
            var layout = new char[line.Length, height];
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < line.Length; x++) layout[x, y] = line[x];
                if(!file.Empty)
                    line = file.NextLine().ToSingleString();
            }

            bool changed = true;
            while (changed) {
                changed = false;
                var newLayout = (char[,])layout.Clone();
                for (int y = 0; y < layout.GetLength(1); y++) {
                    for (int x = 0; x < layout.GetLength(0); x++) {
                        if (layout[x, y] == '.') continue;
                        var adjSeats = CountAdjacent(layout, '#', x, y);
                        if (layout[x, y] == 'L' && adjSeats == 0) {
                            newLayout[x, y] = '#';
                            changed = true;
                        }
                        else if (layout[x, y] == '#' && adjSeats >= 4) {
                            newLayout[x, y] = 'L';
                            changed = true;
                        }
                    }
                }

                layout = newLayout;
            }

            return CountSeats(layout);
        }

        public int CountAdjacent(char[,] layout, char what, int x, int y) {
            int count = 0;
            for (int ydiff = -1; ydiff <= 1; ydiff++)
            for (int xdiff = -1; xdiff <= 1; xdiff++) {
                if (ydiff == 0 && xdiff == 0) continue;
                var currentX = x + xdiff;
                var currentY = y + ydiff;
                if (currentX < 0 || currentX >= layout.GetLength(0) || currentY < 0 ||
                    currentY >= layout.GetLength(1)) continue;
                if (layout[currentX, currentY] == what) count++;
            }

            return count;
        }

        private List<(int X, int Y)>[,] seatsInSight;
        public int CountOccupiedInSight(char[,] layout, int x, int y) {
            if(seatsInSight == null) throw new Exception();
            if (seatsInSight[x, y] != null) return seatsInSight[x, y].Count(p => layout[p.X, p.Y] == '#');
            int count = 0;
            seatsInSight[x, y] = new List<(int X, int Y)>();
            for (int ydiff = -1; ydiff <= 1; ydiff++)
            for (int xdiff = -1; xdiff <= 1; xdiff++) {
                if (ydiff == 0 && xdiff == 0) continue;
                var currentX = x + xdiff;
                var currentY = y + ydiff;
                while (true) {
                    if (currentX < 0 || currentX >= layout.GetLength(0) || currentY < 0 ||
                        currentY >= layout.GetLength(1)) break;
                    if (layout[currentX, currentY] == 'L') {
                        seatsInSight[x,y].Add((currentX, currentY));
                        break;
                    }

                    if (layout[currentX, currentY] == '#') {
                        seatsInSight[x, y].Add((currentX, currentY));
                        count++;
                    }
                    currentX += xdiff;
                    currentY += ydiff;
                }
            }

            return count;
        }

        public int CountSeats(char[,] layout) {
            int count = 0;
            for (int y = 0; y < layout.GetLength(1); y++)
            for (int x = 0; x < layout.GetLength(0); x++)
                if (layout[x, y] == '#')
                    count++;

            return count;
        }

        public object Part2(IParsedFile file) {
            var height = file.Count;
            var line = file.NextLine().ToSingleString();
            var layout = new char[line.Length, height];
            seatsInSight = new List<(int X, int Y)>[line.Length, height];
            for (int y = 0; y < height; y++) {
                for (int x = 0; x < line.Length; x++) layout[x, y] = line[x];
                if (!file.Empty)
                    line = file.NextLine().ToSingleString();
            }

            bool changed = true;
            while (changed) {
                changed = false;
                var newLayout = (char[,])layout.Clone();
                for (int y = 0; y < layout.GetLength(1); y++) {
                    for (int x = 0; x < layout.GetLength(0); x++) {
                        if (layout[x, y] == '.') continue;
                        var adjSeats = CountOccupiedInSight(layout, x, y);
                        if (layout[x, y] == 'L' && adjSeats == 0) {
                            newLayout[x, y] = '#';
                            changed = true;
                        }
                        else if (layout[x, y] == '#' && adjSeats >= 5) {
                            newLayout[x, y] = 'L';
                            changed = true;
                        }
                    }
                }

                layout = newLayout;
            }

            return CountSeats(layout);
        }
    }
}
