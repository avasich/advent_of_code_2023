fn first_integer_boundaries(line: &[char], offset: usize) -> Option<(usize, usize)> {
    let mut chars = line
        .iter()
        .enumerate()
        .skip(offset)
        .skip_while(|(_, c)| !c.is_numeric());

    chars.next().map(|(first, _)| {
        let last = chars
            .take_while(|(_, c)| c.is_numeric())
            .last()
            .map_or(first, |(last, _)| last);
        (first, last)
    })
}

fn parse_integer(line: &[char]) -> u64 {
    line.iter()
        .map(|c| c.to_digit(10).unwrap())
        .rfold((1, 0), |(r, acc), d| (r * 10, acc + d * r))
        .1 as u64
}

pub fn sum_part_numbers(filename: &str) -> u64 {
    let lines: Vec<Vec<_>> = crate::utils::read_lines(filename)
        .map(|line| line.chars().collect())
        .collect();

    let empty = vec![];
    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        let mut offset = 0;
        let higher_line = i.checked_sub(1).map(|j| &lines[j]).unwrap_or(&empty);
        let lower_line = lines.get(i + 1).unwrap_or(&empty);

        while let Some((start, end)) = first_integer_boundaries(line, offset) {
            offset = end + 1;
            let left = start.saturating_sub(1);
            let right = (end + 1).min(line.len() - 1);

            let is_part_number = [line[left], line[right]]
                .iter()
                .chain(higher_line.get(left..=right).into_iter().flatten())
                .chain(lower_line.get(left..=right).into_iter().flatten())
                .any(|&c| !c.is_numeric() && c != '.');

            if is_part_number {
                sum += parse_integer(&line[start..=end]);
            }
        }
    }
    sum
}

fn integer_around(line: &[char], pos: usize) -> Option<u64> {
    if !line[pos].is_numeric() {
        None
    } else {
        let left = (0..=pos)
            .rev()
            .take_while(|&i| line[i].is_numeric())
            .last()
            .unwrap();
        let right = (pos..line.len())
            .take_while(|&i| line[i].is_numeric())
            .last()
            .unwrap();

        Some(parse_integer(&line[left..=right]))
    }
}

fn integers_around(line: &[char], pos: usize) -> Vec<u64> {
    if let Some(middle) = integer_around(line, pos) {
        vec![middle]
    } else {
        let mut res = vec![];
        if let Some(right) = (pos > 0).then(|| integer_around(line, pos - 1)).flatten() {
            res.push(right);
        }
        if let Some(left) = (pos + 1 < line.len())
            .then(|| integer_around(line, pos + 1))
            .flatten()
        {
            res.push(left);
        }
        res
    }
}

pub fn gear_ratio(filename: &str) -> u64 {
    let lines: Vec<Vec<_>> = crate::utils::read_lines(filename)
        .map(|line| line.chars().collect())
        .collect();

    let mut sum = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c != '*' {
                continue;
            }
            let mut numbers = integers_around(line, x);
            if y > 0 {
                numbers.extend(integers_around(&lines[y - 1], x));
            }
            if y + 1 < lines.len() {
                numbers.extend(integers_around(&lines[y + 1], x));
            }
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
    }

    sum
}

#[cfg(test)]
mod d03_tests {
    use super::*;

    static EXAMPLE_01: &str = "./inputs/day_03/example_01.txt";
    static TASK: &str = "./inputs/day_03/task.txt";

    #[test]
    fn read_integer_test() {
        let lines: Vec<Vec<_>> = crate::utils::read_lines(EXAMPLE_01)
            .map(|line| line.chars().collect())
            .collect();

        let number = first_integer_boundaries(&lines[0], 0)
            .map(|(start, end)| parse_integer(&lines[0][start..=end]));
        assert_eq!(number, Some(467));

        let number = first_integer_boundaries(&lines[0], 2)
            .map(|(start, end)| parse_integer(&lines[0][start..=end]));
        assert_eq!(number, Some(7));

        let number = first_integer_boundaries(&lines[1], 0)
            .map(|(start, end)| parse_integer(&lines[1][start..=end]));
        assert_eq!(number, None);

        let number = first_integer_boundaries(&lines[2], 0)
            .map(|(start, end)| parse_integer(&lines[2][start..=end]));
        assert_eq!(number, Some(35));
    }

    #[test]
    fn integer_around_test() {
        let lines: Vec<Vec<_>> = crate::utils::read_lines(EXAMPLE_01)
            .map(|line| line.chars().collect())
            .collect();

        let number = integer_around(&lines[0], 0);
        assert_eq!(number, Some(467));

        let number = integer_around(&lines[0], 2);
        assert_eq!(number, Some(467));

        let number = integer_around(&lines[2], 3);
        assert_eq!(number, Some(35));
    }

    #[test]
    fn p1_example_test() {
        let res = sum_part_numbers(EXAMPLE_01);
        assert_eq!(res, 4361);
    }

    #[test]
    fn p1_task_test() {
        let res = sum_part_numbers(TASK);
        println!("{res}");
    }

    #[test]
    fn p2_example_test() {
        let res = gear_ratio(EXAMPLE_01);
        assert_eq!(res, 467835);
    }

    #[test]
    fn p2_task_test() {
        let res = gear_ratio(TASK);
        println!("{res}");
    }
}
