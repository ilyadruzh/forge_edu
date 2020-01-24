defmodule Solution do
  [head | tail] =
    IO.read(:stdio, :all)
    |> String.split("\n")

  {first, second} = Enum.split(tail, String.to_integer(head))

  firstList =
    first
    |> Enum.map(fn x -> String.split(x) end)
    |> Enum.map(fn [a, b] -> {String.to_atom(a), b} end)
    |> Map.new()

  Enum.each(second, fn x ->
    case Map.has_key?(firstList, String.to_atom(x)) do
      true -> IO.puts(x <> "=" <> Map.get(firstList, String.to_atom(x)))
      false -> IO.puts("Not found")
    end
  end)

  # Enum.each(second, fn x ->
  #   case Keyword.has_key?(firstList, String.to_atom(x)) do
  #     true -> IO.puts(x <> "=" <> Keyword.get(firstList, String.to_atom(x)))
  #     false -> IO.puts("Not found")
  #   end
  # end)

  # second
  # |> Enum.map(fn x ->
  #   case Keyword.has_key?(firstList, String.to_atom(x)) do
  #     true -> IO.puts(x <> "=" <> Keyword.get(firstList, String.to_atom(x)))
  #     false -> IO.puts("Not found")
  #   end
  # end)
end
