defmodule Solution do
  def start do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)

    {head, tail} = Enum.split(n, 1)
    # # IO.inspect(tail) # [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    Enum.map(tail, fn x -> List.duplicate(x, List.first(head)) end)
    |> List.flatten()
    |> Enum.each(fn x -> IO.puts(x) end)
  end
end

Solution.start()
