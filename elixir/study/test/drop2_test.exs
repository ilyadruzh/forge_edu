defmodule Drop2Test do
  use ExUnit.Case, aync: true

  @moduledoc false

  test "Earth calc correct" do
    calculated = Drop.fall_velocity(:earth, 1)
    assert_in_delta calculated, 4.4, 0.0001,
                    "Result of #{calculated} is not within 0.05 of 4.4"
  end

end
