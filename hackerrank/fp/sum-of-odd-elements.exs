defmodule Solution do
  require Integer

  def main do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> Enum.filter(fn x -> Integer.is_odd(x) end)
      |> Enum.sum()

    IO.inspect(n)
  end
end

Solution.main()
