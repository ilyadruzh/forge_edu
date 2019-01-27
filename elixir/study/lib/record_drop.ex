defmodule RecordDrop do
  @moduledoc false

  require Planemo
  require Tower

  def fall_velocity(t = Tower.tower()) do
    fall_velocity(Tower.tower(t, :planemo), Tower.tower(t, :height))
  end

  def fall_velocity(:earth, distance) when distance >= 0  do
    :math.sqrt(2 * 9.8 * distance)
  end

end
