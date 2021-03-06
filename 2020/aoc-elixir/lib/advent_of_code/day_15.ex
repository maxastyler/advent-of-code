defmodule AdventOfCode.Day15 do
  def take_turn({{value, turn}, last_times}) do
    new_value = case Map.get(last_times, value), do: (nil -> 0; x -> turn - x)
    {{new_value, turn + 1}, Map.put(last_times, value, turn)}
  end

  def get_nth_turn(starter_nums, nth) do
    turns = Enum.with_index(starter_nums) |> Enum.reverse()
    last_times = Map.new(turns)

    {{n, _}, _} =
      Stream.iterate({List.first(turns), last_times}, &take_turn/1)
      |> Enum.at(nth - Enum.count(turns))

    n
  end

  def part1(args), do: get_nth_turn(args, 2020)
  def part2(args), do: get_nth_turn(args, 30_000_000)
end
