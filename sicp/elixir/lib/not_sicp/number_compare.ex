defmodule NumberCompare do
  @moduledoc false

  def simple_greater(number, other_number) do
    check(number >= other_number, number, other_number)
  end

  def greater_with_guard(number, other_number) when number >= other_number do
    number
  end

  def greater_with_guard(_, other_number) do
    other_number
  end

  defp check(true, number, _) do
    number
  end

  defp check(false, _, other_number) do
    other_number
  end

end
