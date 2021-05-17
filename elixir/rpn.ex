defmodule RpnCalc do
  def calc do
    "2 3 +"
    |> String.to_charlist()
    |> IO.inspect()
    |> stack_calc()
  end

  def stack_calc(list) do
    [head1, head2, head3, head4 | tail] = list

    IO.inspect(head1)
    IO.inspect(head2)
    IO.inspect(head3)
    IO.inspect(head4)
    IO.inspect(tail)

    if is_integer(head1) do
      rpn("+", [head1, head2])
    end
  end

  def rpn("+", list) do
    [head1, head2 | tail] = list
    head1 + head2
  end
end
