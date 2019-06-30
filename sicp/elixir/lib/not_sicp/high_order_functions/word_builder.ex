defmodule WordBuilder do
  @moduledoc false
  def build(alphabet, positions) do
    letters = Enum.map(positions, String.at(alphabet))
    Enum.join(letters)
  end
end
