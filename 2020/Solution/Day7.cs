using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using aoc2020;
using FileParser;

namespace Solution
{
    public class Day7 : IDay
    {
        public object Part1(IParsedFile file) {
            // (bag, parent)
            var parentPairing = new HashSet<(string bag, string parent)>();
            while (!file.Empty) {
                var line = file.NextLine().ToSingleString();
                var containIndex = line.IndexOf(" contain ", StringComparison.Ordinal);
                var bag = line[..(containIndex-1)];
                var contained = line[(containIndex + "contain  ".Length)..^1].Split(", ");
                Debug.Assert(contained.Length > 0);
                if (contained.Length == 1 && contained[0] == "no other bags") continue;
                foreach (string containedBag in contained) {
                    int count = 0;
                    var index = 0;
                    while (char.IsDigit(containedBag[index])) {
                        count *= 10;
                        count += containedBag[index++] - '0';
                    }

                    var bagName = containedBag[(index + 1)..^(count > 1 ? 1 : 0)];
                    parentPairing.Add((bagName, bag));
                }
            }
            var set = new HashSet<string>();
            var enumerable = parentPairing.Where(x => x.bag == "shiny gold bag").Select(x => x.parent);
            while (enumerable.Any()) {
                var newEnum = Enumerable.Empty<string>();
                foreach (string parent in enumerable) {
                    set.Add(parent);
                    newEnum = newEnum.Concat(parentPairing.Where(x => x.bag == parent).Select(x => x.parent));
                }

                enumerable = newEnum;
            }

            return set.Count;
        }

        public object Part2(IParsedFile file) {
            var dictionary = new Dictionary<string, Dictionary<string, int>>();
            while (!file.Empty) {
                var line = file.NextLine().ToSingleString();
                var containIndex = line.IndexOf(" contain ", StringComparison.Ordinal);
                var bagName = line[..(containIndex - 1)];
                if(dictionary.ContainsKey(bagName)) throw new Exception();
                var contained = line[(containIndex + "contain  ".Length)..^1].Split(", ");
                Debug.Assert(contained.Length > 0);
                if (contained.Length == 1 && contained[0] == "no other bags") continue;
                var bag = new Dictionary<string, int>();
                dictionary.Add(bagName, bag);
                foreach (string containedBag in contained) {
                    int count = 0;
                    var index = 0;
                    while (char.IsDigit(containedBag[index])) {
                        count *= 10;
                        count += containedBag[index++] - '0';
                    }

                    var containedBagName = containedBag[(index + 1)..^(count > 1 ? 1 : 0)];
                    bag.Add(containedBagName, count);
                }
            }

            return CountBags(dictionary, "shiny gold bag");
        }

        private int CountBags(Dictionary<string, Dictionary<string, int>> bagDictionary, string currentBag) {
            if (!bagDictionary.ContainsKey(currentBag)) return 0;
            var result = 0;
            foreach (KeyValuePair<string, int> keyValuePair in bagDictionary[currentBag]) {
                result += keyValuePair.Value + keyValuePair.Value * CountBags(bagDictionary, keyValuePair.Key);
            }

            return result;
        }
    }
}
