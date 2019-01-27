defmodule DropTest do
  use ExUnit.Case, async: true
  doctest Drop

  setup_all do
    IO.puts "Beginning all tests"

    on_exit fn ->
      IO.puts "Exit from all test"
    end

    {:ok, [connection: :fake_PID]}
  end

  test "Zero distance gives zero velocity" do
    assert Drop.fall_velocity(:earth, 0) == 0
  end

  test "Mars calculation correct" do
    assert Drop.fall_velocity(:mars, 10) == :math.sqrt(2 * 3.71 * 10)
  end

end
