defmodule FallWeb.FallController do
  @moduledoc false

  use FallWeb, :controller

  def welcome(conn, params) do
    choices = ["Earth": 1, "Moon": 2, "Nars": 3]
    render conn, "welcome.html", choices: choices
  end

  def faller(conn, params) do
    choices = ["Earth": 1, "Moon": 2, "Nars": 3]
    speed = 0
    render conn, "faller.html", speed: speed, choices: choices
  end

  def faller(
        conn,
        %{
          "calculation" => %{
            "planemo" => planemo,
            "distance" => distance

          }
        }
      ) do

    calc_planemo = String.to_integer(planemo)
    calc_distance = String.to_integer(distance)
    speed = fall_velocity(calc_planemo, calc_distance)
    chices = ["Earth": 1, "Moon": 2, "Mars": 3]
    render conn, "faller.html", speed: speed, choices: choices
  end

  def fall_velocity(1, distance)do
    :math.sqrt(2 * 9.8 * distance)
  end

  def fall_velocity(2, distance)do
    :math.sqrt(2 * 1.6 * distance)
  end
  def fall_velocity(3, distance)do
    :math.sqrt(2 * 3.71 * distance)
  end

end
