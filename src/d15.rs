//! A solution to day 15 year 2023.
//! https://adventofcode.com/2023/day/15

type Model = Vec<String>;
type Answer = usize;

pub fn parse(input: String) -> Model {
    input
        .trim()
        .split(',')
        .map(|step| step.to_string())
        .collect()
}

pub fn part1(model: Model) -> Answer {
    model.into_iter().map(hash).sum()
}

pub fn part2(model: Model) -> Answer {
    let mut boxes = vec![Box::default(); 256];

    for step in model {
        let (label, focal_length) = step.split_at(step.find(['-', '=']).unwrap());
        let (cmd, focal_length) = focal_length.split_at(1);
        let focal_length = focal_length.parse().unwrap_or(0);

        let idx = hash(label);
        if let Some(existing_lens) = boxes[idx]
            .lenses
            .iter()
            .position(|box_lens| box_lens.label == label)
        {
            if cmd == "-" {
                if let Some(existing_idx) = boxes[idx]
                    .lenses
                    .iter()
                    .position(|box_lens| box_lens.label == label)
                {
                    boxes[idx].lenses.remove(existing_idx);
                }
            } else if cmd == "=" {
                boxes[idx].lenses[existing_lens] = Lens {
                    label: label.to_string(),
                    focal_length,
                };
            }
        } else if cmd == "=" {
            // handle add new
            boxes[idx].lenses.push(Lens {
                label: label.to_string(),
                focal_length,
            });
        }
    }

    // calculate score

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_num, b)| {
            b.lenses
                .iter()
                .enumerate()
                .map(|(lens_num, lens)| (box_num + 1) * (lens_num + 1) * lens.focal_length)
                .collect::<Vec<usize>>()
        })
        .sum()
}

fn hash<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().chars().fold(0, |sum, c| {
        (sum as u8).wrapping_add(c as u8).wrapping_mul(17) as usize
    })
}

#[derive(Debug, Default, Clone)]
struct Box {
    lenses: Vec<Lens>,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/d15");
    const EXAMPLE: &str = include_str!("../examples/d15");

    #[test]
    fn d15p1_example_test() {
        assert_eq!(part1(parse(EXAMPLE.to_string())), 1320);
    }

    #[test]
    fn d15p1_input_test() {
        assert_eq!(part1(parse(INPUT.to_string())), 505459);
    }

    #[test]
    fn d15p2_example_test() {
        assert_eq!(part2(parse(EXAMPLE.to_string())), 145);
    }

    #[test]
    fn d15p2_input_test() {
        assert_eq!(part2(parse(INPUT.to_string())), 228508);
    }
}
