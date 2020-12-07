defmodule AdventOfCode.Day07 do
  
  def parse_line(line) do
    [colour, contained] = Regex.run(~r/([[:alnum:]\ ]+) bags contain (.+)/,
      line, capture: :all_but_first)
    {colour, Regex.scan(~r/(\d+) ([[:alnum:]\ ]+) bag/, contained, capture: :all_but_first)
    |> Enum.map(fn [n, c] -> {c, String.to_integer(n)} end)
    |> Map.new}
  end

  def parse_input(input) do
    input |> String.trim |> String.split("\n") |> Enum.map(&parse_line/1) |> Map.new
  end

  def contains_shiny_gold(colour, rules) do
    if Map.has_key?(rules[colour], "shiny gold") do
      true
    else
      Enum.any?(rules[colour], fn {c, _} -> contains_shiny_gold(c, rules) end)
    end
  end

  def count_bags(bag, rules) do
    Enum.reduce(rules[bag], 0, fn {c, n}, acc -> acc + (n * (count_bags(c, rules) + 1)) end)
  end
  
  def part1(args) do
    input = parse_input(args)
    Enum.filter(input, fn {c, _} -> contains_shiny_gold(c, input) end)
    |> Enum.count
  end

  def part2(args), do: count_bags("shiny gold", parse_input(args))
end
