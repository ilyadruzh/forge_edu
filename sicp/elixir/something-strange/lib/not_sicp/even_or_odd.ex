defmodule EvenOrOdd do
  @moduledoc false

  require Integer

  def check(number) when Integer.is_even(number) do
    "even"
  end

  def check(number) when Integer.is_odd(number) do
    "odd"
  end

  def is_even(number) do
    require Integer
    Integer.is_even(number)
  end

  def is_odd(number), do: Integer.is_odd(number)

end
