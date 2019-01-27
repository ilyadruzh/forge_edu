defmodule NewType do
  @moduledoc false
@type planetuple :: {atom(), number()}

@spec fall_velocity(planetuple, number()) :: float
  def fall_velocity({_planemo, gravity}, distance) when distance > 0 do
    :math.sqrt(2 * gravity * distance)
  end
end
