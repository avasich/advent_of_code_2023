fn parse_numbers(line: &str) -> Vec<u64> {
    line.split_whitespace()
        .flat_map(|val| val.parse().ok())
        .collect()
}

type Map = Vec<(u64, u64, u64)>;

fn parse_file(filename: &str) -> (Vec<u64>, Vec<Map>) {
    let mut lines = crate::utils::read_lines(filename);
    let seeds = lines
        .next()
        .map(|line| parse_numbers(line.split_once(':').unwrap().1))
        .unwrap();

    let mut tmp = vec![];
    let mut maps = vec![];

    lines.next();

    for line in lines {
        if line.is_empty() {
            tmp.sort_by_key(|(_, source, _)| *source);
            maps.push(tmp);
            tmp = vec![];
            continue;
        }
        if line.ends_with(':') {
            continue;
        }

        let mut numbers = line
            .split_whitespace()
            .flat_map(|number| number.parse::<u64>().ok());

        let dest = numbers.next().unwrap();
        let source = numbers.next().unwrap();
        let length = numbers.next().unwrap();

        tmp.push((dest, source, length));
    }

    tmp.sort_by_key(|(_, source, _)| *source);
    maps.push(tmp);

    (seeds, maps)
}

fn map_value(val: u64, map: &Map) -> u64 {
    map.binary_search_by_key(&val, |(_, s, _)| *s)
        .map_or_else(|e| (e > 0).then(|| map[e - 1]), |r| Some(map[r]))
        .map(|(dest, source, length)| {
            if (source..(source + length)).contains(&val) {
                dest + (val - source)
            } else {
                val
            }
        })
        .unwrap_or(val)
}

pub fn lowest_location(filename: &str) -> u64 {
    let (seeds, maps) = parse_file(filename);
    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, map_value))
        .min()
        .unwrap()
}

#[cfg(test)]
mod d05_tests {
    use super::*;
    static EXAMPLE_01: &str = "./inputs/day_05/example_01.txt";
    static TASK: &str = "./inputs/day_05/task.txt";

    #[test]
    fn map_value_test() {
        let (seeds, maps) = parse_file(EXAMPLE_01);
        let res = map_value(1, &maps[0]);
        assert_eq!(res, 1);

        let res = map_value(79, &maps[0]);
        assert_eq!(res, 81);

        let res = map_value(99, &maps[0]);
        assert_eq!(res, 51);
    }

    #[test]
    fn p1_example_test() {
        let res = lowest_location(EXAMPLE_01);
        assert_eq!(res, 35);
    }

    #[test]
    fn p1_task_test() {
        let res = lowest_location(TASK);
        println!("{res}");
    }
}
