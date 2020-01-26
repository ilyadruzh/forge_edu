defmodule Solution do
  use Bitwise

  def start do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    Enum.filter(Enum.with_index(n), fn {_, i} -> is_integer(i) and (i &&& 1) == 1 end)
    |> Enum.each(fn {x, _} -> IO.puts(x) end)
  end
end

Solution.start()
