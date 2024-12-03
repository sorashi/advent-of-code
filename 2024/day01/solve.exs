{l1, l2} =
  IO.read(:eof)
  |> String.split("\n", trim: true)
  |> Enum.map(fn s -> String.split(s) end)
  |> Enum.map(fn [a, b] -> {String.to_integer(a), String.to_integer(b)} end)
  |> Enum.unzip()

silver =
  [Enum.sort(l1), Enum.sort(l2)]
  |> Enum.zip()
  |> Enum.map(fn {a, b} -> abs(a - b) end)
  |> Enum.sum()

IO.puts("silver: #{silver}")

f = Enum.frequencies(l2)

gold =
  l1
  |> Enum.map(fn x ->
    (f[x] || 0) * x
  end)
  |> Enum.sum()

IO.puts("gold: #{gold}")
