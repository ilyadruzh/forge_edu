defmodule Checkout do
  @moduledoc false



  def total_cost(price, tax_rate) when price >=0 and tax_rate >= 0 do
    price * (tax_rate + 1)
  end
end
