defmodule Solution do
  require Integer

  def main do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> Enum.reduce(0, fn _, acc -> acc + 1 end)

    IO.inspect(n)
  end
end

Solution.main()
