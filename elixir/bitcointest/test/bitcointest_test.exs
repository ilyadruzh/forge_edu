defmodule BitcointestTest do
  use ExUnit.Case
  doctest Bitcointest

  test "greets the world" do
    assert Bitcointest.hello() == :world
  end
end
