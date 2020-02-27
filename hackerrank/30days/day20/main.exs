defmodule Solution do
  def start do
    [head | tail] =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    tail
    |> sort([], 0, head)
  end

  def sort([], result, count, step) when step == 0  do
    IO.puts("Array is sorted in #{count} swaps.")
    first_elem(result)
    last_elem(result)
  end

  def sort(input, result, count, step) do
    case hd(input) > hd(result) do
      true -> sort(tl(input), result ++ [hd(input)], count = count + 1, step)
      false -> sort(tl(input), [hd(input)] ++ result, count, step = step - 1)
    end
  end

  defp first_elem(input) do
    n = hd(input)
    IO.puts("First Element: #{n}")
  end

  defp last_elem(input) do
    n = Enum.reverse(input) |> hd()
    IO.puts("Last Element: #{n}")
  end
end

Solution.start()
