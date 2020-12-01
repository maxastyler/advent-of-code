defmodule AdventOfCode.Day02Test do
  use ExUnit.Case

  import AdventOfCode.Day02

  test "part1" do
    input = "5 1 9 5
7 5 3
2 4 6 8"
    result = part1(input)

    assert result == 18
  end

  @tag :skip
  test "part2" do
    input = nil
    result = part2(input)

    assert result
  end
end
