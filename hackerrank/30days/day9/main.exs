defmodule Solution do
  # Enter your code here. Read input from STDIN. Print output to STDOUT

  def factorial do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> List.first()
      |> String.to_integer()

    IO.puts(fact(n))
  end

  def fact(), do: 0

  def fact(1), do: 1

  def fact(n) do
    n * fact(n - 1)
  end
end

Solution.factorial()
