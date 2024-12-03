inp = IO.read(:eof) |> String.trim()

r = ~r/mul\((\d+),(\d+)\)/

silver =
  Regex.scan(r, inp)
  |> Enum.map(&tl/1)
  |> Enum.map(fn [a, b] -> String.to_integer(a) * String.to_integer(b) end)
  |> Enum.sum()

IO.puts("silver: #{silver}")

r = ~r/mul\((\d+),(\d+)\)|do\(\)|don't\(\)/

{gold, _} =
  Regex.scan(r, inp)
  |> Enum.reduce({0, true}, fn [inst | rest], acc ->
    {acc, enabled} = acc

    case inst do
      "mul" <> _ when enabled ->
        [a, b] = rest |> Enum.map(&String.to_integer/1)
        {acc + a * b, true}

      "don't" <> _ ->
        {acc, false}

      "do" <> _ ->
        {acc, true}

      _ ->
        {acc, enabled}
    end
  end)

IO.puts("gold: #{gold}")
