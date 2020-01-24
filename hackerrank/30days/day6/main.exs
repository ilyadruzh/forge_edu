defmodule Solution do
  use Bitwise

  IO.read(:stdio, :all)
  |> String.trim()
  |> String.split()
  |> Enum.filter(&:erlang.is_bitstring/1)
  |> tl
  |> Enum.map(fn x ->
    String.graphemes(x)
    |> Enum.with_index()
    |> Enum.split_with(fn x ->
      x
      |> elem(1)
      |> (&(is_integer(&1) and (&1 &&& 1) == 0)).()
    end)
    |> Tuple.to_list()
    |> Enum.map(fn x ->
      x |> Enum.map(fn x -> x |> elem(0) end) |> (&(&1 ++ [" "])).() |> List.to_string()
    end)
    |> IO.puts()
  end)
end
