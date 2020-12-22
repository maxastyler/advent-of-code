defmodule AdventOfCode.Day22 do
  def parse_input(input) do
    [p1, p2] =
      String.split(input, "\n\n")
      |> Enum.map(fn x ->
        String.split(x, "\n", trim: true) |> Enum.drop(1) |> Enum.map(&String.to_integer/1)
      end)

    {p1, p2}
  end

  def play_game_p1({[c1 | d1], [c2 | d2]}) do
    cond do
      c1 > c2 -> {d1 ++ [c1, c2], d2}
      true -> {d1, d2 ++ [c2, c1]}
    end
    |> play_game_p1()
  end

  def play_game_p1({d1, d2}), do: d1 ++ d2

  def play_round_p2({[c1 | d1], [c2 | d2]}) do
    cond do
      length(d1) >= c1 and length(d2) >= c2 ->
        case play_game_p2({Enum.slice(d1, 0, c1), Enum.slice(d2, 0, c2)}) do
          {:p1, _} -> {d1 ++ [c1, c2], d2}
          {:p2, _} -> {d1, d2 ++ [c2, c1]}
        end

      c1 >= c2 ->
        {d1 ++ [c1, c2], d2}

      :else ->
        {d1, d2 ++ [c2, c1]}
    end
  end

  def play_game_p2({d1, d2} = state, history \\ MapSet.new()) do
    cond do
      state in history -> {:p1, d1 ++ d2}
      length(d1) == 0 -> {:p2, d2}
      length(d2) == 0 -> {:p1, d1}
      true -> play_game_p2(play_round_p2(state), MapSet.put(history, state))
    end
  end

  def part1(args) do
    for {v, i} <- play_game_p1(parse_input(args)) |> Enum.reverse() |> Enum.with_index(1),
        reduce: 0 do
      s -> s + v * i
    end
  end

  def part2(args) do
    for {v, i} <-
          elem(play_game_p2(parse_input(args)), 1)
          |> Enum.reverse()
          |> Enum.with_index(1),
        reduce: 0 do
      s -> s + v * i
    end
  end
end
