defmodule Hof do
  @moduledoc false

  def tripler(value, function) do
    3 * function.(value)
  end

end
