defmodule AdventOfCode.Day10 do
  def parse_input(input) do
    [0 | String.trim(input)
         |> String.split("\n")
         |> Enum.map(&String.to_integer/1)
         |> Enum.sort
         |> (&(&1 ++ [List.last(&1) + 3])).()]
  end

  def get_steps(input) do
    Enum.chunk_every(input, 2, 1, :discard)
    |> Enum.map(fn [x, y] -> y - x end)
  end

  def count_paths(d, _) when d <= 1, do: 1
  def count_paths(d, dx), do: Enum.map(1..min(dx, d), &count_paths(d-&1, dx)) |> Enum.sum

  def chunk_list([dh | dt], [h | t]) do
    case dh do
      1 -> chunk_list(dt, [h+1 | t])
      _ -> chunk_list(dt, [0 | [h | t]])
    end
  end
  def chunk_list(_, xs), do: xs

  def part1(args) do
    parse_input(args)
    |> get_steps
    |> Enum.frequencies
    |> (fn %{1 => ones, 3 => threes} -> ones * threes end).()
  end

  def part2(args) do
    parse_input(args)
    |> get_steps
    |> chunk_list([0]) |> Enum.reduce(1, &(count_paths(&1, 3) * &2))
  end
end
