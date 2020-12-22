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

  def play_game_p1({d1, []}), do: {:p1, d1}
  def play_game_p1({[], d2}), do: {:p2, d2}

  def play_round_p2({[c1 | d1], [c2 | d2]}) do
    cond do
      length(d1) >= c1 and length(d2) >= c2 ->
        case play_game_p2({Enum.take(d1, c1), Enum.take(d2, c2)}) do
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
      d1 == [] -> {:p2, d2}
      d2 == [] -> {:p1, d1}
      state in history -> {:p1, d1 ++ d2}
      true -> play_game_p2(play_round_p2(state), MapSet.put(history, state))
    end
  end

  def score_game({_, cards}) do
    for {v, i} <- Enum.reverse(cards) |> Enum.with_index(1), reduce: 0, do: (s -> s + v * i)
  end

  def part1(args), do: parse_input(args) |> play_game_p1() |> score_game()
  def part2(args), do: parse_input(args) |> play_game_p2() |> score_game()
end
