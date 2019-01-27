defmodule Bounce do
  @moduledoc false
  def report(count) do
    new_count = receive do
      msg -> IO.puts("Received #{msg}")
      count+1
    end
    report(new_count)
  end
end
