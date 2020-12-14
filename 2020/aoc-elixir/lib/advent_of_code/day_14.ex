defmodule AdventOfCode.Day14 do
  use Bitwise
  def parse_mask(mask) do
    for x <- String.graphemes(mask), reduce: {0, 0} do
      {zeros, ones} -> case x do
                         "X" -> {(zeros <<< 1) ||| 1, ones <<< 1}
                         "0" -> {zeros <<< 1, ones <<< 1}
                         "1" -> {(zeros <<< 1) ||| 1, (ones <<< 1) ||| 1}
                       end
    end
  end

  def parse_line("mask = " <> rest) do
    {zeros, ones} = parse_mask(rest)
    {:mask, zeros, ones}
  end
  def parse_line("mem" <> rest) do
    [[loc], [val]] = Regex.scan(~r/(\d+)/, rest, capture: :all_but_first)
    {:set, String.to_integer(loc), String.to_integer(val)}
  end

  def parse_input(input) do
    instructions = String.trim(input) |> String.split("\n") |> Enum.map(&parse_line/1)
    for {inst, x1, x2} <- instructions, reduce: {0, 0, %{}} do
      {zeros, ones, memory} -> case inst do
                                 :mask -> {x1, x2, memory}
                                 :set -> {zeros, ones, Map.put(memory, x1, (x2 &&& zeros) ||| ones)}
                               end
    end
  end

  def part1(args) do
    {_, _, memory} = parse_input(args)
    Map.values(memory) |> Enum.sum
  end

  def part2(args) do
  end
end
