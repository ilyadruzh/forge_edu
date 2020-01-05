defmodule HighOrderFunctions.TestData do
  @moduledoc false

  def enchanted_items do
    enchanted_items = [
      %{title: "Edwin's Longsword", price: 150},
      %{title: "Healing Potion", price: 60},
      %{title: "Edwin's Rope", price: 30},
      %{title: "Dragon's Spear", price: 100}
    ]
  end

  def medals do
    medals = [
      %{medal: :gold, player: "Anna"},
      %{medal: :silver, player: "Joe"},
      %{medal: :gold, player: "Zoe"},
      %{medal: :bronze, player: "Anna"},
      %{medal: :silver, player: "Anderson"},
      %{medal: :silver, player: "Peter"}
    ]
  end
end
