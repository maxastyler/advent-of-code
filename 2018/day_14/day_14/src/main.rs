fn recipes(n: usize) -> Vec<u8> {
    //! Produce recipes of at least this size
    let mut e1 = 0;
    let mut e2 = 1;
    let mut recipes = Vec::with_capacity(n);
    recipes.push(3);
    recipes.push(7);
    while recipes.len() < n {
        recipes.extend_from_slice(&digitise(recipes[e1] + recipes[e2]));
        e1 = (e1 + 1 + (recipes[e1] as usize)) % recipes.len();
        e2 = (e2 + 1 + (recipes[e2] as usize)) % recipes.len();
    }
    recipes
}

fn digitise(n: u8) -> Vec<u8> {
    format!("{}", n)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

fn main() {
    let initial: Vec<u8> = vec![0, 4, 7, 8, 0, 1];
    let r = recipes(30000001);
    println!("Part 1: {:?}", &r[47801..47801 + 6]);
    for i in 0..(r.len() - initial.len()) {
        if initial[..] == r[i..(i + initial.len())] {
            println!("Part 2: {:?}", i);
            break;
        }
    }
}
