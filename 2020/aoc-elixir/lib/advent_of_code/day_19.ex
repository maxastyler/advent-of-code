defmodule AdventOfCode.Day19 do
  def parse_rule(rule) do
    [num, rest] = Regex.run(~r/(\d+): (.*)/, rule, capture: :all_but_first)

    {String.to_integer(num),
     case Regex.run(~r/"([[:alpha:]+])"/, rest, capture: :all_but_first) do
       nil ->
         String.split(rest, "|")
         |> Enum.map(fn x -> String.split(x) |> Enum.map(&String.to_integer/1) end)

       [letter] ->
         letter
     end}
  end

  def satisfy("", [], _), do: true
  def satisfy("", _, _), do: false
  def satisfy(_, [], _), do: false

  def satisfy(str, [r | rs], rules) do
    case rules[r] do
      xs when is_list(xs) ->
        Enum.any?(xs, &satisfy(str, &1 ++ rs, rules))

      x ->
        case String.split_at(str, 1) do
          {^x, rest} -> satisfy(rest, rs, rules)
          _ -> false
        end
    end
  end

  def count_matching(input, rule_modification \\ %{}) do
    [rules, strings] = String.split(input, "\n\n")

    rules =
      String.split(rules, "\n", trim: true)
      |> Enum.map(&parse_rule/1)
      |> Map.new()
      |> Map.merge(rule_modification)

    String.split(strings, "\n", trim: true)
    |> Enum.filter(&satisfy(&1, [0], rules))
    |> Enum.count()
  end

  def part1(args), do: count_matching(args)

  def part2(args) do
    count_matching(args, %{8 => [[42], [42, 8]], 11 => [[42, 31], [42, 11, 31]]})
  end
end
