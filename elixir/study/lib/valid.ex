defprotocol Valid do
  @doc "Возвращает тру, если данные можно считать допустимыми"
  def valid?(data)
end
