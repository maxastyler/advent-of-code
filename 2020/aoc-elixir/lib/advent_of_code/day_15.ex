defmodule AdventOfCode.Day15 do
  def take_turn({{value, turn}, last_times}) do
    current_turn = turn + 1
    spoken_when = Map.get(last_times, value)
    if spoken_when do
      new_val = turn - spoken_when
      {{new_val, current_turn}, Map.put(last_times, value, turn)}
    else
      {{0, current_turn}, Map.put(last_times, value, turn)}
    end
  end

  def get_nth_turn(starter_nums, nth) do
    turns = Enum.with_index(starter_nums) |> Enum.reverse
    last_times = for {x, i} <- turns, reduce: %{} do
      acc -> Map.put(acc, x, i)
    end
    {{n, _}, _} = Stream.iterate({List.first(turns), last_times}, &take_turn/1)
    |> Enum.at(nth - Enum.count(turns))
    n
  end

  def part1(args), do: get_nth_turn(args, 2020)
  def part2(args), do: get_nth_turn(args, 30000000)
end
