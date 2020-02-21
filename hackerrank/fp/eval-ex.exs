defmodule Solution do
  require Integer

  def main do
    IO.read(:stdio, :all)
    |> String.trim()
    |> String.split()
    |> Enum.drop(1)
    |> Enum.map(&String.to_float/1)
    |> Enum.map(fn x -> 1 + Enum.reduce(0..9, fn y, acc -> acc + part(x, y) end) end)
    |> Enum.each(fn x -> IO.puts(Float.floor(x, 4)) end)
  end

  def part(x, n) do
    :math.pow(x, n) / factorial(n)
  end

  def factorial(0), do: 1
  def factorial(n) when n > 0, do: n * factorial(n - 1)
end

Solution.main()
