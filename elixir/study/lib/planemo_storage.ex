defmodule PlanemoStorage do
  @moduledoc false
  require Planemo

  def setup do
    planemo_table = :ets.new(
      :planemos,
      [
        :named_table,
        {:keypos, Planemo.planemo(:name) + 1}
      ]
    )

    :ets.insert :planemos,
                Planemo.planemo(
                  name: :mercury,
                  gravity: 3.7,
                  diameter: 4878,
                  distance_from_sun: 57.9
                )
    :ets.info planemo_table

  end
end
