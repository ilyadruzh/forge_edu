defmodule YourTurn1 do
  @moduledoc false

end

# TODO Create a function that returns Tic-Tac-Toe game winners. You can repre-
# sent the board with a tuple of nine elements, where each group of three
# items is a row. The return of the function should be a tuple. When we
# have a winner, the first element should be the atom :winner , and the second
#                                                              should be the player. When we have no winner, the tuple should contain
#               one item that is the atom :no_winner . It should work like this:
#                                                         TicTacToe.winner({
#                                                                          :x, :o, :x,
#               :o, :x, :o,
#               :o, :o, :x
#               })
#                # {:winner, :x}
#                TicTacToe.winner({
#                :x, :o, :x,
#                    :o, :x, :o,
#                        :o, :x, :o
#                            })
#                            # :no_winneraaaaaaa


# TODO: In RPGs, players have points to spend on their character attributes.
# Create a function that returns the total number of points players have spent on
# their characters. The function will receive a map containing the strength,
# dexterity, and intelligence values. Each point in strength should be multi-
# plied by two, and dexterity and intelligence should be multiplied by three.

#  abilities = %{strength: 16, dexterity: 12, intelligence: 10}
#  user_input = IO.gets "Write your ability score:\n"
#  {ability_score, _} = Integer.parse(user_input)
#  ability_modifier = (ability_score - 10) / 2
#  IO.puts "Your ability modifier is #{ability_modifier}"


# TODO Create a function that calculates income tax following these rules: a salary
#equal or below $2,000 pays no tax; below or equal to $3,000 pays 5%; below
#or equal to $6,000 pays 10%; everything higher than $6,000 pays 15%.

# TODO Create an Elixir script where users can type their salary and see the
#income tax and the net wage. You can use the module from the previous
#exercise, but this script should parse the user input and display a message
#when users type something that is not a valid number.