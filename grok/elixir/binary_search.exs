def binary_search(list, item) do
cond do
  Enum.at(list, Enum.count(list) / 2) == item -> {:ok, item}
  Enum.at(list, Enum.count(list) / 2) > item -> binary_search(list, item) # ищем в левой части
  Enum.at(list, Enum.count(list) / 2) < item -> binary_search(list, item) # ищем в правой части
end
end
