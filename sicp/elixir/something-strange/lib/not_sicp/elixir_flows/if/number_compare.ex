defmodule ElixirFlows.If.NumberCompare do
  @moduledoc false

  def greater(number, other_number) do
    if number >= other_number do
      number
    else
      other_number
    end
  end

end


defmodule NumberCompareWithUnless do
  def greater(number, other_number) do
    unless number < other_number do
      number
    else
      other_number
    end
  end
end

defmodule NumberCompareWithIf do
  def greater(number, other_number) do
    if number >= other_number do
      number
    else
      other_number
    end
  end
end