defmodule Solution do
  def main do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.to_integer()

    IO.inspect(Enum.to_list(0..(n - 1)), limit: :infinity)
  end
end

Solution.main()
