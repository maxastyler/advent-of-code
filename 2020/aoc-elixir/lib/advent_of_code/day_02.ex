defmodule AdventOfCode.Day02 do

  def password_valid_1?(%{min: mi, max: ma, letter: letter, password: password}) do
    num_letters = password
    |> String.graphemes
    |> Enum.filter(&(&1 == letter))
    |> Enum.count

    num_letters in mi..ma
  end

  def password_valid_2?(%{min: mi, max: ma, letter: letter, password: password}) do
    s1 = String.at(password, mi-1) == letter
    s2 = String.at(password, ma-1) == letter
    (s1 or s2) and (s1 != s2)
  end
  
  def parse_passwords(input) do
    Regex.scan(~r/(\d+)-(\d+)\ ([[:alpha:]]):\ ([[:alpha:]]+)/, input)
    |> Enum.map(fn ([_, mi, ma, ch, pass]) -> %{min: String.to_integer(mi),
                                               max: String.to_integer(ma),
                                               letter: ch,
                                               password: pass} end)
  end

  def part1(args) do
    passes = parse_passwords(args)
    Enum.filter(passes, &password_valid_1?/1)
    |> Enum.count
  end

  def part2(args) do
    passes = parse_passwords(args)
    Enum.filter(passes, &password_valid_2?/1)
    |> Enum.count
  end
end
