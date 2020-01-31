defmodule Solution do
  # Enter your code here. Read input from STDIN. Print output to STDOUT

  def start() do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> List.first()
      |> String.to_integer()

    IO.puts(extract(<<n::64>>))
  end

  def extract(str) when is_binary(str) do
    extract(str, [])
  end

  defp extract(<<b::size(1), bits::bitstring>>, acc) when is_bitstring(bits) do
    extract(bits, [b | acc])
  end

  defp extract(<<>>, acc) do
    acc
    |> Enum.reverse()
    |> List.foldl(acc, fn x, acc ->
      cond do
        x == 0 ->
          [0 | acc]

        x == 1 ->
          cond do
            List.first(acc) == 0 -> [1 | acc]
            List.first(acc) > 0 -> [List.first(acc) + 1 | acc]
          end
      end
    end)
    |> Enum.max()
  end
end

Solution.start()
