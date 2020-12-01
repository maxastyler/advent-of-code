defmodule AdventOfCode.Day05 do
  def part1(args) do
    inp = args
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(fn (x) -> Integer.parse(x) |>  elem(0) end)
    Enum.with_index

    jump(0, 0, inp)
  end

  defp jump(pos, jumps, list) do
    if (pos >= 0 and pos < Enum.count(list)) do
      jump(list[pos], jumps + 1, update_in(list, pos, &inc/1))
    else
      jumps
    end
  end

  defp inc(x), do: x+1

  def part2(args) do
  end
end
