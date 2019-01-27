defmodule Drop do
  @moduledoc """
  Функции вычисления скорости свободно падающего объекта в вакууме.
  """

  require Planemo

  @vsn 0.1

  @doc """
  Вычисляет скорость свободно падающего объекта на Земле.
  """

  def fall_velocity({planemo, distance}) when distance >= 0 do
    fall_velocity(planemo, distance)
  end

  def fall_velocity(planemo, distance) do
    try do

      gravity = case planemo do
        :earth -> 9.8
        :moon -> 1.6
        :mars -> 3.41
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

    rescue
      ArithmeticError -> {:error, "Distance must be non-negative"}
      CaseClauseError -> {:error, "Unknown planemo #{planemo}"}
    after
      IO.puts("after")
    end
  end

  def drop do
    setup
    handle_drops

  end

  def handle_drops do
    receive do
      {from, planemo, distance} ->
        send(from, {planemo, distance, fall_velocity(planemo, distance)})
        handle_drops
    end
  end

  def setup do
   :mnesia.create_schema([node()])
   :mnesia.start()
   :mnesia.create_table(PlanemoTable, [{:attributes,
   [:name, :gravity, :diameter, :distance_from_sun]},
     {:recird_name, :planemo}])

   end


  def insert_into_table([]) do
    :undefined
  end

  def insert_into_table([{name, gravity, diameter, distance} | tail]) do
    :ets.insert(:planemos, Planemo.new(name: name, gravity: gravity,
    diameter: diameter, distance_from_sun: distance))
    insert_into_table(tail)
  end

  def wrongness() do
    total_distance = fall_velocity(:earth, 20) +
      fall_velocity(:moon, 20) +
      fall_velocity(:jupiter, 20) +
      fall_velocity(:earth, "20")
    total_distance
  end
end