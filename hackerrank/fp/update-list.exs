defmodule Solution do
  require Integer

  def main do
    IO.read(:stdio, :all)
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
    |> Enum.map(fn x ->
      if x > 0 do
        x
      else
        x * -1
      end
    end)
    |> Enum.each(fn x -> IO.puts(x) end)
  end
end

Solution.main()
