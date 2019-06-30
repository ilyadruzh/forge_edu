defmodule HighOrderFunctions do
  @moduledoc false

  def compose(f, g) do
    fn arg -> f.(g.(arg)) end
  end
end
