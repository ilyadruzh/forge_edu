defmodule TowerOld do
  @moduledoc false
  defstruct location: "", height: 20, planemo: :earth, name: ""
end

defimpl Valid, for: TowerOld do
  def valid?(%TowerOld{height: h, planemo: p}) do
    h >= 0 and p != nil
  end
end

defimpl Inspect, for: TowerOld do
  import Inspect.Algebra

  def inspect(item, _options) do
    metres = concat(to_string(item.height), "m:")
    msg = concat([metres, break, item.name, ",", break, item.location, ",", break, to_string(item.planemo)])
  end
end