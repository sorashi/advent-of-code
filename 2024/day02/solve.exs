defmodule Solve do
  def safe(report) do
    chunk = report |> Enum.chunk_every(2, 1, :discard)
    [a, b] = chunk |> hd

    range =
      case b - a do
        x when x > 0 -> 1..3
        x when x < 0 -> -3..-1
        _ -> nil
      end

    if range do
      chunk
      |> Enum.all?(fn [a, b] ->
        Enum.member?(range, b - a)
      end)
    else
      false
    end
  end

  def safe_gold(report) do
    n = length(report)

    safe_gold(report, n, -3..-1, 0) || safe_gold(report, n, 1..3, 0) ||
      safe_gold(report |> tl, n - 1, 1..3, 1) || safe_gold(report |> tl, n - 1, -3..-1, 1)
  end

  defp safe_gold(report, n, range, skipped) when n >= 3 and skipped <= 1 do
    [a, b, c | rest] = report

    skip_b = Enum.member?(range, c - a) && safe_gold([c | rest], n - 2, range, skipped + 1)
    noskip = Enum.member?(range, b - a) && safe_gold([b, c | rest], n - 1, range, skipped)
    skip_b || noskip
  end

  defp safe_gold(report, n, range, skipped) when n >= 2 and skipped <= 1 do
    [a, b | rest] = report

    skip_b = safe_gold(rest, n - 2, range, skipped + 1)
    noskip = Enum.member?(range, b - a) && safe_gold([b | rest], n - 1, range, skipped)
    skip_b || noskip
  end

  defp safe_gold(_, _, _, skipped) when skipped > 1 do
    false
  end

  defp safe_gold(_, n, _, skipped) when n <= 1 and skipped <= 1 do
    true
  end
end

inp =
  IO.stream()
  |> Stream.map(fn x -> String.split(x) |> Enum.map(&String.to_integer/1) end)
  |> Enum.to_list()

silver = inp |> Enum.count(&Solve.safe/1)
IO.puts("silver: #{silver}")

gold = inp |> Enum.count(&Solve.safe_gold/1)
IO.puts("gold: #{gold}")

ExUnit.start()

defmodule FooTest do
  use ExUnit.Case

  test "example" do
    assert Solve.safe_gold([7, 6, 4, 2, 1])
    assert Solve.safe_gold([1, 2, 7, 8, 9]) === false
    assert Solve.safe_gold([9, 7, 6, 2, 1]) === false
    assert Solve.safe_gold([1, 3, 2, 4, 5])
    assert Solve.safe_gold([8, 6, 4, 4, 1])
    assert Solve.safe_gold([1, 3, 6, 7, 9])
  end

  test "skip first" do
    assert Solve.safe_gold([1, 10, 11, 12])
  end

  test "skip last" do
    assert Solve.safe_gold([10, 11, 12, 69])
  end

  test "edge cases from reddit" do
    assert Solve.safe_gold([48, 46, 47, 49, 51, 54, 56])
    assert Solve.safe_gold([1, 1, 2, 3, 4, 5])
    assert Solve.safe_gold([1, 2, 3, 4, 5, 5])
    assert Solve.safe_gold([5, 1, 2, 3, 4, 5])
    assert Solve.safe_gold([1, 4, 3, 2, 1])
    assert Solve.safe_gold([1, 6, 7, 8, 9])
    assert Solve.safe_gold([1, 2, 3, 4, 3])
    assert Solve.safe_gold([9, 8, 7, 6, 7])
    assert Solve.safe_gold([7, 10, 8, 10, 11])
    assert Solve.safe_gold([29, 28, 27, 25, 26, 25, 22, 20])
  end
end
