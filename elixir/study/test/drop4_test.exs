defmodule Drop4Test do
  use ExUnit.Case, async: true

  alias Drop4

  @moduletag :capture_log

  doctest Drop

  test "module exists" do
    assert is_list(Drop.module_info())
  end

  test "Unknown planemo causes error" do
    assert_raise CaseClauserError, fn ->
    Drop.fall_velocity(:planetX, 10)
    end
  end

  test "Negative distance causes error" do
    assert_raise ArithmeticError, fn ->
    Drop.fall_velocity(:earth, -10)
    end
  end
end
