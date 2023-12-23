use memoize::memoize;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Invalid condition char: {}", c),
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct RowData {
    layout: Vec<Condition>,
    damaged_groups: Vec<u64>,
}

fn parse_row(row: &str) -> RowData {
    let row_sections = row.split_whitespace();
    let layout: Vec<Condition> = row_sections
        .clone()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| Condition::from_char(c))
        .collect();
    let damaged_groups: Vec<u64> = row_sections
        .clone()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    RowData {
        layout,
        damaged_groups,
    }
}

fn unfold_row(row_data: &RowData) -> RowData {
    let mut unfolded_layout = row_data.layout.clone();
    let mut unfolded_damaged_groups = row_data.damaged_groups.clone();

    for _ in 0..4 {
        unfolded_layout.push(Condition::Unknown);
        unfolded_layout.append(&mut row_data.layout.clone());
        unfolded_damaged_groups.append(&mut row_data.damaged_groups.clone());
    }

    RowData {
        layout: unfolded_layout,
        damaged_groups: unfolded_damaged_groups,
    }
}

#[memoize]
fn get_row_permutations(row_data: RowData, mut damaged_streak: u64) -> u64 {
    let mut permutations: u64 = 0;
    let mut damaged_groups = row_data.damaged_groups.clone();
    let mut must_be_operational = false;

    for (i, condition) in row_data.layout.iter().enumerate() {
        match condition {
            Condition::Damaged => {
                if damaged_groups.len() == 0 || must_be_operational {
                    return 0;
                }
                damaged_streak += 1;
                if damaged_streak == damaged_groups[0] {
                    damaged_streak = 0;
                    damaged_groups.remove(0);
                    must_be_operational = true;
                } else if damaged_streak > damaged_groups[0] {
                    return 0;
                }
            }
            Condition::Operational => {
                if damaged_streak > 0 {
                    return 0;
                }
                must_be_operational = false;
            }
            Condition::Unknown => {
                if !must_be_operational {
                    let mut layout_if_damaged = vec![Condition::Damaged];
                    layout_if_damaged.append(&mut row_data.layout[i + 1..].to_vec().clone());
                    let sub_row_if_damaged = RowData {
                        layout: layout_if_damaged,
                        damaged_groups: damaged_groups.clone(),
                    };
                    permutations += get_row_permutations(sub_row_if_damaged, damaged_streak);
                }

                if damaged_streak == 0 {
                    let mut layout_if_operational = vec![Condition::Operational];
                    layout_if_operational.append(&mut row_data.layout[i + 1..].to_vec().clone());
                    let sub_row_if_operational = RowData {
                        layout: layout_if_operational,
                        damaged_groups: damaged_groups.clone(),
                    };
                    permutations += get_row_permutations(sub_row_if_operational, damaged_streak);
                }

                // Return to exit the loop
                return permutations;
            }
        }
    }

    if damaged_groups.len() == 0 {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut sum_of_arrangements: u64 = 0;
    for line in reader.lines() {
        let mut row = parse_row(&line.unwrap());
        row = unfold_row(&row);
        sum_of_arrangements += get_row_permutations(row, 0) as u64;
    }

    println!("Sum of arrangements: {}", sum_of_arrangements);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_row_1() {
        let row = parse_row("???.### 1,1,3");
        assert_eq!(
            row,
            RowData {
                layout: vec![
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Unknown,
                    Condition::Operational,
                    Condition::Damaged,
                    Condition::Damaged,
                    Condition::Damaged,
                ],
                damaged_groups: vec![1, 1, 3],
            }
        );
    }

    #[test]
    fn it_evaluates_example_row_1() {
        let row = parse_row("???.### 1,1,3");
        assert_eq!(get_row_permutations(row, 0), 1);
    }

    #[test]
    fn it_evaluates_example_row_2() {
        let row = parse_row(".??..??...?##. 1,1,3");
        assert_eq!(get_row_permutations(row, 0), 4);
    }

    #[test]
    fn it_evaluates_example_row_3() {
        let row = parse_row("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(get_row_permutations(row, 0), 1);
    }

    #[test]
    fn it_evaluates_example_row_4() {
        let row = parse_row("????.#...#... 4,1,1");
        assert_eq!(get_row_permutations(row, 0), 1);
    }

    #[test]
    fn it_evaluates_example_row_5() {
        let row = parse_row("????.######..#####. 1,6,5");
        assert_eq!(get_row_permutations(row, 0), 4);
    }

    #[test]
    fn it_evaluates_example_row_6() {
        let row = parse_row("?###???????? 3,2,1");
        assert_eq!(get_row_permutations(row, 0), 10);
    }

    #[test]
    fn it_evaluates_edge_case() {
        let row = parse_row("..?????#?? 4,1");
        assert_eq!(get_row_permutations(row, 0), 2);
    }
}
