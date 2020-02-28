defmodule Solution do
  def start do
    [head | tail] =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    tail
    |> sort()
  end

  def sort(list) when is_list(list) do
    make_pass(do_sort(list, [], 0), list)
  end

  def make_pass(bubbled_list, old_list) when bubbled_list != old_list do
    do_sort(bubbled_list, [], 0) |> make_pass(bubbled_list)
  end

  def make_pass(bubbled_list, old_list) when tl(bubbled_list) == old_list do
    IO.inspect(bubbled_list)
    IO.inspect(old_list)

    IO.puts("Array is sorted in #{hd(bubbled_list)} swaps.")
    first_elem(tl(bubbled_list))
    last_elem(bubbled_list)
    # bubbled_list
  end

  def do_sort(_list = [], _acc, _count) do
    []
  end

  def do_sort([first | []], acc, count) do
    [count] ++ acc ++ [first]
  end

  def do_sort([first | [second | tail]], acc, count) do
    [new_first, new_second, new_count] = swap(first, second, count)
    do_sort([new_second | tail], acc ++ [new_first], new_count)
  end

  defp swap(e1, e2, count) do
    if e1 <= e2 do
      [e1, e2, count]
    else
      [e2, e1, count = count + 1]
    end
  end

  defp first_elem(input) do
    n = hd(tl(input))
    IO.puts("First Element: #{n}")
  end

  defp last_elem(input) do
    n = Enum.reverse(input) |> hd()
    IO.puts("Last Element: #{n}")
  end
end

Solution.start()
