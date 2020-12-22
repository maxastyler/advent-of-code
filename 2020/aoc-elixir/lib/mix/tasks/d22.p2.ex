defmodule Mix.Tasks.D22.P2 do
  use Mix.Task

  import AdventOfCode.Day22

  @shortdoc "Day 22 Part 2"
  def run(args) do
    input = """
    Player 1:
    50
    19
    40
    22
    7
    4
    3
    16
    34
    45
    46
    39
    44
    32
    20
    29
    15
    35
    41
    2
    21
    28
    6
    26
    48

    Player 2:
    14
    9
    37
    47
    38
    27
    30
    24
    36
    31
    43
    42
    11
    17
    18
    10
    12
    5
    33
    25
    8
    23
    1
    13
    49
    """

    if Enum.member?(args, "-b"),
      do: Benchee.run(%{part_2: fn -> input |> part2() end}),
      else:
        input
        |> part2()
        |> IO.inspect(label: "Part 2 Results")
  end
end
