defmodule Solution do
  def start do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    {head, tail} = Enum.split(n, 1)
    Enum.filter(tail, fn x -> x < List.first(head) end) |> Enum.each(fn x -> IO.puts(x) end)
  end
end

Solution.start()
