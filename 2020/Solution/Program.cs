using System;
using System.Diagnostics;
using System.Linq;
using System.Reflection;
using FileParser;

namespace aoc2020
{
    class Program
    {
        static void Main(string[] args) {

            var days = Assembly.GetExecutingAssembly().GetTypes().Where(x => x.GetInterface(nameof(IDay)) != null);
            var latestDayType = days.OrderByDescending(x => {
                if(x.Name[..3] != "Day") throw new FormatException();
                return int.Parse(x.Name[3..]);
            }).First();

            var day = (IDay)Activator.CreateInstance(latestDayType);
            if(day == null) throw new NullReferenceException();

            // stopwatch measures cpu time, not the true performance of methods (because of JIT interference)
            var sw = new Stopwatch();
            var file = new ParsedFile("input.txt");
            sw.Start();
            var result = day.Part1(file);
            sw.Stop();
            Console.WriteLine($"PART1: {result}\t{sw.ElapsedMilliseconds}ms");

            file = new ParsedFile("input.txt");
            sw.Restart();
            result = day.Part2(file);
            Console.WriteLine($"PART2: {result}\t{sw.ElapsedMilliseconds}ms");
        }
    }
}
