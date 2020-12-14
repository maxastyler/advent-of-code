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

  def location_mask(location, mask) do
    for {g, i} <- String.graphemes(mask) |> Enum.reverse |> Enum.with_index, reduce: [location] do
      acc -> case g do
               "0" -> acc
               "1" -> for n <- acc, do: n ||| (1 <<< i)
               "X" -> (for n <- acc, do: [n &&& ~~~(1 <<< i), n ||| (1 <<< i)]) |> Enum.concat
             end
    end
  end

  def part1(args) do
    {_, _, memory} = parse_input(args)
    Map.values(memory) |> Enum.sum
  end

  def part2(args) do
    {_, memory} = for l <- String.trim(args) |> String.split("\n"), reduce: {"", %{}} do
      {mask, memory} -> case l do
                          "mask = " <> rest -> {rest, memory}
                          mem -> with {_, loc, val} <- parse_line(mem) do
                                   {mask, Enum.reduce(location_mask(loc, mask),memory,
                                       fn e, a -> Map.put(a, e, val) end)}
                                 end
                        end
    end
    Map.values(memory) |> Enum.sum
  end
end
