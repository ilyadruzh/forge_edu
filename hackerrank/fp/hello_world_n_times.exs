defmodule Solution do
  # Enter your code here. Read input from STDIN. Print output to STDOUT
  n =
    IO.read(:stdio, :all)
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
    |> List.first()

  1..n |> Enum.each(fn x -> IO.puts("Hello World") end)
end
