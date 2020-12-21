defmodule AdventOfCode.Day21 do
  def parse_recipe(recipe) do
    [ingredients, allergens] =
      Regex.run(~r/(.*) \(contains (.*)\)/, recipe, capture: :all_but_first)

    {String.split(ingredients, " ", trim: true), String.split(allergens, ", ", trim: true)}
  end

  def solve_allergens(allergens) do
    {solved, to_remove} =
      for {a, i} <- Enum.filter(allergens, fn {_, i} -> Enum.count(i) == 1 end),
          reduce: {%{}, MapSet.new()} do
        {solved, to_remove} ->
          ingredient = MapSet.to_list(i) |> List.first()
          {Map.put(solved, a, ingredient), MapSet.put(to_remove, ingredient)}
      end

    filtered =
      Enum.reject(allergens, fn {_, i} -> Enum.count(i) == 1 end)
      |> Enum.map(fn {a, i} -> {a, MapSet.difference(i, to_remove)} end)

    if Enum.count(filtered) == 0 do
      solved
    else
      Map.merge(solved, solve_allergens(filtered))
    end
  end

  def parse_ingredients(input) do
    for {ingredients, allergens} <-
          String.split(input, "\n", trim: true) |> Enum.map(&parse_recipe/1),
        ingredient_set = MapSet.new(ingredients),
        reduce: {[], %{}} do
      {ingredient_list, allergen_map} ->
        {Enum.concat(ingredient_list, ingredients),
         for allergen <- allergens, reduce: allergen_map do
           allergen_map ->
             Map.update(
               allergen_map,
               allergen,
               ingredient_set,
               &MapSet.intersection(&1, ingredient_set)
             )
         end}
    end
  end

  def part1(args) do
    {ingredient_list, allergen_map} = parse_ingredients(args)

    solved = solve_allergens(allergen_map)
    dangerous = Map.values(solved)
    Enum.reject(ingredient_list, &(&1 in dangerous)) |> Enum.count()
  end

  def part2(args) do
    {_, allergen_map} = parse_ingredients(args)

    dangerous =
      solve_allergens(allergen_map)
      |> Enum.sort_by(&elem(&1, 0))
      |> Enum.map(&elem(&1, 1))
      |> Enum.join(",")
  end
end
