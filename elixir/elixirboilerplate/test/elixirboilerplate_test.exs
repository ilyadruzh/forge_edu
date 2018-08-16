defmodule ElixirboilerplateTest do
  use ExUnit.Case
  doctest Elixirboilerplate

  test "greets the world" do
    assert Elixirboilerplate.hello() == :world
  end
end
