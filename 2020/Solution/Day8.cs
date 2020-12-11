using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using aoc2020;
using FileParser;
using Microsoft.VisualBasic.CompilerServices;

namespace Solution
{
    public class Day8 : IDay
    {
        public object Part1(IParsedFile file) {
            var bootloader = ParseBootloader(file).ToArray();
            CorrectBootloader(bootloader, out var acc);
            return acc;
        }

        public object Part2(IParsedFile file) {
            var bootloader = ParseBootloader(file).ToArray();
            int i = 0;
            while (i < bootloader.Length) {
                i = Array.FindIndex(bootloader, i, o => o.Op != Op.Acc);
                var op = bootloader[i++];
                op.Switch();
                if (CorrectBootloader(bootloader, out var acc))
                    return acc;
                op.Switch();
            }
            throw new Exception();
        }

        private static bool CorrectBootloader(Operation[] ops, out int acc) {
            var s = new HashSet<int>();
            var i = 0;
            acc = 0;
            while (!s.Contains(i)) {
                s.Add(i);
                if (i < 0 || i >= ops.Length) return true;
                var op = ops[i];
                switch (op.Op) {
                    case Op.Acc:
                        acc += op.Arg;
                        i++;
                        break;
                    case Op.Nop:
                        i++;
                        break;
                    case Op.Jmp:
                        i += op.Arg;
                        break;
                    default:
                        throw new ArgumentOutOfRangeException();
                }
            }

            return false;
        }

        private static IEnumerable<Operation> ParseBootloader(IParsedFile file) {
            while (!file.Empty) {
                var line = file.NextLine();
                Op op = line.NextElement<string>() switch {
                    "acc" => Op.Acc,
                    "nop" => Op.Nop,
                    "jmp" => Op.Jmp,
                    _ => throw new Exception()
                };
                yield return new Operation(op, line.NextElement<int>());
            }
        }
        enum Op
        {
            Acc, Nop, Jmp
        }
        private class Operation
        {
            public Op Op;
            public int Arg;

            public void Switch() {
                Op = Op switch {
                    Op.Jmp => Op.Nop,
                    Op.Nop => Op.Jmp,
                    _ => Op
                };
            }
            public Operation(Op op, int arg) {
                this.Op = op;
                Arg = arg;
            }
        }
    }
}
