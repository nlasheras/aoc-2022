use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

#[derive(Copy, Clone, Debug)]
pub struct Number {
    n: i64,
    index: usize,
}

#[aoc_generator(day20)]
pub fn parse_input(input: &str) -> Vec<Number> {
    let ns = input
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    (0..ns.len())
        .map(|i| Number { n: ns[i], index: i })
        .collect()
}

fn mix(input: &[Number]) -> Vec<Number> {
    let mut mixed = input.to_owned();

    for i in 0..input.len() {
        let n = mixed.iter().find(|n| n.index == i).unwrap();
        if n.n == 0 {
            continue;
        }
        let pos = mixed.iter().position(|n2| n2.index == n.index).unwrap();
        let elem = mixed.remove(pos);
        let mut new_index = (pos as i64 + elem.n).rem_euclid(mixed.len() as i64);
        new_index = new_index.rem_euclid(mixed.len() as i64);
        if new_index == 0 {
            new_index = mixed.len() as i64;
        }
        mixed.insert(new_index as usize, elem);
    }
    mixed
}

#[aoc(day20, part1)]
fn decrypt_sum_3(input: &[Number]) -> u64 {
    let mixed = mix(input);
    let zero = mixed.iter().position(|n| n.n == 0).unwrap();
    (mixed[(1000 + zero) % mixed.len()].n
        + mixed[(2000 + zero) % mixed.len()].n
        + mixed[(3000 + zero) % mixed.len()].n) as u64
}

#[aoc(day20, part2)]
fn decrypt_sum_3_v2(input: &[Number]) -> u64 {
    let decrypted = input
        .iter()
        .map(|num| Number {
            n: num.n * 811589153,
            index: num.index,
        })
        .collect::<Vec<Number>>();
    let mut mixed = decrypted;
    for _ in 0..10 {
        mixed = mix(&mixed);
    }
    let zero = mixed.iter().position(|n| n.n == 0).unwrap();
    (mixed[(1000 + zero) % mixed.len()].n
        + mixed[(2000 + zero) % mixed.len()].n
        + mixed[(3000 + zero) % mixed.len()].n) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY20_EXAMPLE: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_day20_part1() {
        let input = parse_input(DAY20_EXAMPLE);
        assert_eq!(decrypt_sum_3(&input), 3);
    }

    #[test]
    fn test_day20_mix() {
        let input = parse_input(DAY20_EXAMPLE);
        let mixed = mix(&input);
        let tmp = mixed.iter().map(|n| n.n).collect::<Vec<i64>>();
        assert_eq!(tmp, [1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_day20_mix_multiplied() {
        let input = parse_input(DAY20_EXAMPLE);
        let decrypted = input
            .iter()
            .map(|num| Number {
                n: num.n * 811589153,
                index: num.index,
            })
            .collect::<Vec<Number>>();
        let mixed_1 = mix(&decrypted);
        let tmp = mixed_1.iter().map(|n| n.n).collect::<Vec<i64>>();
        assert_eq!(
            tmp,
            [
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153
            ]
        );
    }

    #[test]
    fn test_day20_part2() {
        let input = parse_input(DAY20_EXAMPLE);
        assert_eq!(decrypt_sum_3_v2(&input), 1623178306);
    }
}
