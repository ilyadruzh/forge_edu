defmodule Solution do
  def main do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> Enum.reverse()
      |> Enum.each(fn x -> IO.inspect(x) end)
  end
end

Solution.main()
