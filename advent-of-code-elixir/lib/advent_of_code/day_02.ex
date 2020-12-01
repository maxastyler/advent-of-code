defmodule AdventOfCode.Day02 do
  defp line_parse(line) do
    String.split(line, "\t")
    |> Enum.map(&(Integer.parse(&1) |> elem(0)))
  end

  def part1(args) do
    String.trim(args)
    |> String.split("\n")
    |> Enum.map(
    fn (x) -> line_parse(x) |> (fn (y) -> (Enum.max(y) - Enum.min(y)) end).() end
    ) |> Enum.sum
  end

  def find_pair(nums) do
    List.first(for x <- nums, y <- nums, x != y, rem(max(x, y), min(x, y))==0, do: max(x, y) / min(x, y))
  end

  def part2(args) do
    String.trim(args)
    |> String.split("\n")
    |> Enum.map(fn (x) -> line_parse(x) |> find_pair() end)
    |> Enum.sum()
  end
end
