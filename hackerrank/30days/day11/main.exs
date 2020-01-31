defmodule Solution do
  # Enter your code here. Read input from STDIN. Print output to STDOUT

  # берем общее количество эллемнтов, делим на 3 - получаем номер начаола нового блока и его инкремент
  def start() do
    n =
      IO.read(:stdio, :all)
      |> String.trim()
      |> String.split()
      |> Enum.map(&String.to_integer/1)
      |> (&Enum.chunk_every(&1, round(:math.sqrt(length(&1))))).()
      |> Enum.map(fn x -> Enum.chunk_every(x, 3) end)
      |> IO.inspect()
      |> Enum.map(fn x -> Enum.sum(x) end)
      |> Enum.max()
      |> engine
  end

  def engine(n) do
    IO.inspect(n)
  end

  def line() do
  end
end

Solution.start()
