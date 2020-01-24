defmodule Solution do
  use Bitwise
  # Enter your code here. Read input from STDIN. Print output to STDOUT
  n =
    IO.read(:stdio, :all)
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
    |> List.first

  cond do
    is_integer(n) and (n &&& 1) == 1 -> IO.puts("Weird")
    is_integer(n) and (n &&& 1) == 0 and n in 2..5 -> IO.puts("Not Weird")
    is_integer(n) and (n &&& 1) == 0 and n in 6..20 -> IO.puts("Weird")
    is_integer(n) and (n &&& 1) == 0 and n > 20 -> IO.puts("Not Weird")
    true -> IO.puts("Error")
  end
end
