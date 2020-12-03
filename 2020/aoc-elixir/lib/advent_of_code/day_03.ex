defmodule AdventOfCode.Day03 do

  def toboggan_race(step_x, step_y, [first_line | _] = trees) do
    width = String.length(first_line)
    x_positions = Stream.iterate(0, fn x -> rem(x + step_x, width) end)
    Stream.take_every(trees, step_y)
    |> Stream.zip(x_positions)
    |> Stream.map(fn {line, x} -> if String.at(line, x) == "#", do: 1, else: 0 end)
    |> Enum.sum()
  end

  def part1(args) do
    toboggan_race(3, 1, String.split(args, "\n"))
  end

  def part2(args) do
    trees = String.split(args, "\n")
    [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]]
    |> Enum.map(fn [x_step, y_step] -> toboggan_race(x_step, y_step, trees) end)
    |> Enum.reduce(1, fn (e, a) -> e * a end)
  end
end
