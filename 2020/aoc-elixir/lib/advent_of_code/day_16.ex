defmodule AdventOfCode.Day16 do
  def parse_ticket(ticket) do
    for [t] <- Regex.scan(~r/(\d+)/, ticket, capture: :all_but_first) do
      String.to_integer(t)
    end
  end

  @doc "returns a list of the invalid elements from the ticket"
  def invalid_ticket(ticket, rules) do
    Enum.filter(ticket, fn x ->
      not Enum.any?(Enum.flat_map(rules, fn {_, rs} -> rs end), &(x in &1))
    end)
  end

  @doc "returns a tuple of {[{description, [rules]}], my_ticket, other_tickets}"
  def parse_input(input) do
    [[rules, my_ticket, other_tickets]] =
      Regex.scan(~r/(.*)your ticket:\n(.*)nearby tickets:\n(.*)/s, input, capture: :all_but_first)

    {for [desc, a, b, c, d] <-
           Regex.scan(~r/([[:alpha:] ]+): (\d+)-(\d+) or (\d+)-(\d+)/, rules,
             capture: :all_but_first
           ) do
       {desc,
        [
          String.to_integer(a)..String.to_integer(b),
          String.to_integer(c)..String.to_integer(d)
        ]}
     end, parse_ticket(my_ticket), String.split(other_tickets) |> Enum.map(&parse_ticket/1)}
  end

  def part1(args) do
    {rules, _, other_tickets} = parse_input(args)

    other_tickets |> Enum.flat_map(&invalid_ticket(&1, rules)) |> Enum.sum()
  end

  @doc "does the given rule match the set?"
  def rule_matches_set({_, [r1, r2]}, set), do: Enum.all?(set, &(&1 in r1 or &1 in r2))

  def part2(args) do
    {rules, my_ticket, [h | _] = other_tickets} = parse_input(args)

    other_tickets = Enum.filter(other_tickets, &([] == invalid_ticket(&1, rules)))

    pos_sets =
      for ticket <- other_tickets, reduce: List.duplicate(MapSet.new(), Enum.count(h)) do
        acc -> Enum.zip(acc, ticket) |> Enum.map(fn {s, t} -> MapSet.put(s, t) end)
      end

    {fields, _} =
      for {desc, allowed_positions} <-
            Enum.map(rules, fn {desc, _} = rule ->
              {desc,
               Enum.with_index(pos_sets)
               |> Enum.filter(&rule_matches_set(rule, elem(&1, 0)))
               |> Enum.map(&elem(&1, 1))}
            end)
            |> Enum.sort_by(&Enum.count(elem(&1, 1))),
          reduce: {%{}, MapSet.new()} do
        {fields, used} ->
          next_pos = Enum.reject(allowed_positions, &(&1 in used)) |> List.first()
          {Map.put(fields, desc, next_pos), MapSet.put(used, next_pos)}
      end

    for {"departure" <> _, pos} <- fields, reduce: 1 do
      acc -> acc * Enum.at(my_ticket, pos)
    end
  end
end
