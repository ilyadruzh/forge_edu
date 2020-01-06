defmodule Chapter3_4_2 do

  def list_len(list) do
    # TODO
  end

  def list_len_tc(list) do
    # TODO
  end

  def range(from, to) do
    # TODO
  end

  def range_tc(from, to) do
    # TODO
  end

  def positive(list) do
    # TODO
  end

  def positive_tc(list) do
    # TODO
  end
end

defmodule Chapter3_4_5 do
  def lines_lengths!(filepath) do
    # TODO
  end

  def longest_line_length!(filepath) do
    # TODO
  end

  def longest_line!(filepath) do
    # TODO
  end

  def words_per_line!(filepath) do
    # TODO
  end
end

defmodule Chapter3.NaturalNums do
  def print(1), do: IO.puts(1)

  def print(n) when n > 0 do
    print(n - 1)
    IO.puts(n)
  end
end
