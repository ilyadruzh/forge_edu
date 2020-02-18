defmodule Solution do
  # Enter your code here. Read input from STDIN. Print output to STDOUT
  def start() do
    IO.read(:stdio, :all)
    |> String.trim()
    |> String.split()
    |> Enum.map(&String.to_integer/1)
    |> engine
  end

  def engine(n) do
    max =
      Enum.at(n, 0) + Enum.at(n, 1) + Enum.at(n, 2) + Enum.at(n, 7) +
        Enum.at(n, 12) + Enum.at(n, 13) + Enum.at(n, 14)

    sum(n, max, 0)
  end

  def sum(nil, max, line) do
    IO.puts(max)
  end

  def sum(n, max, line) do
    [_ | tail] = n

    cond do
      Enum.at(n, 14) == nil ->
        sum(nil, max, line + 1)

      line == 4 || line == 5 || line == 10 || line == 11 || line == 16 || line == 17 || line == 22 ||
          line == 23 ->
        sum(tail, max, line + 1)

      Enum.at(n, 14) != nil ->
        result =
          Enum.at(n, 0) + Enum.at(n, 1) + Enum.at(n, 2) + Enum.at(n, 7) +
            Enum.at(n, 12) + Enum.at(n, 13) + Enum.at(n, 14)

        cond do
          result > max ->
            sum(tail, result, line + 1)

          result < max ->
            sum(tail, max, line + 1)

          result == max ->
            sum(tail, max, line + 1)
        end
    end
  end
end

Solution.start()
