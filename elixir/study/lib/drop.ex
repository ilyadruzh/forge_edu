defmodule Drop do
  @moduledoc """
  Функции вычисления скорости свободно падающего объекта в вакууме.
  """

  @vsn 0.1

  @doc """
  Вычисляет скорость свободно падающего объекта на Земле.
  """

  def fall_velocity({planemo, distance}) when distance >=0 do
    fall_velocity(planemo, distance)
  end

  def fall_velocity(planemo, distance) when distance >=0 do

    gravity = case planemo do
      :earth -> 9.8
      :moon -> 1.6
      :mars -> 3.71
    end
    velocity = :math.sqrt(2 * gravity * distance)

    IO.puts("velocity: #{velocity}!")

    description = cond do
      velocity == 0 -> :stable
      velocity < 5 -> :slow
      velocity >= 5 and velocity < 10 -> :moving
      velocity >= 10 and velocity < 20 -> :fast
      velocity >= 20 -> :speedy
    end

    if description == :speedy do
      IO.puts("Look out below!")
    else
      IO.puts("Reasonable...")
    end
  end

#  defp fall_velocity(:earth, distance) when distance >= 0 do
#    :math.sqrt(2 * 9.8 * distance)
#  end
#
#  defp fall_velocity(:moon, distance) when distance >= 0 do
#    :math.sqrt(2 * 1.6 * distance)
#  end
#
#  defp fall_velocity(:mars, distance) when distance >= 0 do
#    :math.sqrt(2 * 3.71 * distance)
#  end

end