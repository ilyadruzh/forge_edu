defmodule Solution do
  # Enter your code here. Read input from STDIN. Print output to STDOUT
  # n =
  IO.read(:stdio, :all)
  |> String.trim()
  |> String.split()
  |> Enum.map(&String.to_integer/1)
  |> List.first()
  |> (&Enum.each(1..10, fn x ->
        IO.puts(
          Integer.to_string(&1) <>
            " x " <> Integer.to_string(x) <> " = " <> Integer.to_string(&1 * x)
        )
      end)).()

  #  Enum.each(1..10, fn x -> IO.puts(n <> " x " <> x <> " = " <> x * n) end)
  # Enum.each(1..10, fn x ->
  #   IO.puts(
  #     Integer.to_string(n) <> " x " <> Integer.to_string(x) <> " = " <> Integer.to_string(x * n)
  #   )
  # end)
end
