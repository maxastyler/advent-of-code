defmodule AdventOfCode.Day04 do
  def input_to_maps(input) do
    String.split(input, "\n\n")
    |> Enum.map(&(Regex.scan(~r/([[:alpha:]]+):([[:alnum:]#]+)/, &1)
                  |> Map.new(fn [_, k, v] -> {String.to_atom(k), v} end)))
  end

  def has_keys(map) do
    [:byr, :iyr, :eyr, :hgt, :hcl, :ecl, :pid]
    |> Enum.all?(&(Map.has_key?(map, &1)))
  end

  def part1(args) do
    Enum.filter(input_to_maps(args), &has_keys/1) |> Enum.count
  end

  def val_byr(x), do: (1920 <= String.to_integer(x)) and (String.to_integer(x) <= 2002)
  def val_iyr(x), do: (2010 <= String.to_integer(x)) and (String.to_integer(x) <= 2020)
  def val_eyr(x), do: (2020 <= String.to_integer(x)) and (String.to_integer(x) <= 2030)
  def val_hgt([_, x, "cm"]), do: (150 <= String.to_integer(x)) and (String.to_integer(x) <= 193)
  def val_hgt([_, x, "in"]), do: (59 <= String.to_integer(x)) and (String.to_integer(x) <= 76)
  def val_hgt(_), do: false
  def val_hcl(x), do: Regex.match?(~r/#[0-9a-f]{6}/, x)
  def val_ecl(x), do: Enum.member?(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"], x)
  def val_pid(x), do: Regex.match?(~r/^\d{9}$/, x)

  def val_passport(%{byr: byr, iyr: iyr, eyr: eyr, hgt: hgt, hcl: hcl, ecl: ecl, pid: pid}) do
    Enum.all?([val_byr(byr), val_iyr(iyr), val_eyr(eyr), val_hgt(Regex.run(~r/(\d+)(cm|in)/, hgt)),
               val_hcl(hcl), val_ecl(ecl), val_pid(pid)])
  end
  def val_passport(_), do: false

  def part2(args) do
    input_to_maps(args) |> Enum.filter(&val_passport/1) |> Enum.count
  end
end
