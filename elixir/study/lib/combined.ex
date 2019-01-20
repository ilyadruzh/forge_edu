defmodule Combined do
  @moduledoc false

  import Convert

  def height_to_mph(meters) do
    Drop.fall_velocity(meters) |> mps_to_mph
  end
end
