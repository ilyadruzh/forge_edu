def binary_search(list, item) do
  cond do
    Enum.at(list, Enum.count(list) / 2) == item -> {:ok, item}
    # ищем в левой части
    Enum.at(list, Enum.count(list) / 2) > item -> binary_search(list, item)
    # ищем в правой части
    Enum.at(list, Enum.count(list) / 2) < item -> binary_search(list, item)
  end
end
