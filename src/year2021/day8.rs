use anyhow::Context;
use itertools::Itertools;

use aoc::ProblemOutput;

aoc::register!(solve, 2021, 8);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let displays: Vec<_> = input
        .trim()
        .lines()
        .map(|line| -> anyhow::Result<_> {
            let (inp, out) = line.split_once('|').context("invalid input")?;
            Ok(SegDisplay {
                digits: inp.trim().split(' ').map(sort_str).collect(),
                outputs: out.trim().split(' ').map(sort_str).collect(),
            })
        })
        .try_collect()?;

    out.set_part1(
        displays
            .iter()
            .map(|d| {
                d.outputs
                    .iter()
                    .filter(|out| out.len() != 5 && out.len() != 6)
                    .count()
            })
            .sum::<usize>(),
    );

    out.set_part2(
        displays
            .iter()
            .map(|seg| seg.solve())
            .try_fold(0, |acc, xopt| xopt.map(|x| x + acc))
            .context("couldn't solve part 2")?,
    );

    Ok(())
}

fn to_sorted_string(mut bytes: Vec<u8>) -> String {
    bytes.sort_unstable();
    String::from_utf8(bytes).unwrap()
}

fn sort_str(s: &str) -> String {
    s.chars().sorted().collect()
}

struct SegDisplay {
    digits: Vec<String>,
    outputs: Vec<String>,
}

impl SegDisplay {
    fn find_digit_encodings(&self) -> Option<[String; 10]> {
        let mut counts = (b'a'..=b'g')
            .map(|c| {
                self.digits
                    .iter()
                    .filter(|d| d.bytes().contains(&c))
                    .count()
            })
            .collect_vec();

        let one = self.digits.iter().find(|d| d.len() == 2)?;
        let four = self.digits.iter().find(|d| d.len() == 4)?;
        let seven = self.digits.iter().find(|d| d.len() == 3)?;

        let seg_a = seven
            .bytes()
            .zip_longest(one.bytes())
            .find(|z| z.as_ref().left() != z.as_ref().right())?
            .left()?;
        counts[usize::from(seg_a - b'a')] = 0;

        let seg_b = counts.iter().position(|&cnt| cnt == 6)? as u8 + b'a';
        let seg_c = counts.iter().position(|&cnt| cnt == 8)? as u8 + b'a';
        let seg_e = counts.iter().position(|&cnt| cnt == 4)? as u8 + b'a';
        let seg_f = counts.iter().position(|&cnt| cnt == 9)? as u8 + b'a';

        let (mut seg_d, mut seg_g) = counts
            .iter()
            .positions(|&cnt| cnt == 7)
            .map(|i| i as u8 + b'a')
            .collect_tuple()?;

        if four.bytes().contains(&seg_g) {
            std::mem::swap(&mut seg_d, &mut seg_g);
        }

        Some([
            to_sorted_string(vec![seg_a, seg_b, seg_c, seg_e, seg_f, seg_g]), // digit 0
            to_sorted_string(vec![seg_c, seg_f]),                             // digit 1
            to_sorted_string(vec![seg_a, seg_c, seg_d, seg_e, seg_g]),        // digit 2
            to_sorted_string(vec![seg_a, seg_c, seg_d, seg_f, seg_g]),        // digit 3
            to_sorted_string(vec![seg_b, seg_c, seg_d, seg_f]),               // digit 4
            to_sorted_string(vec![seg_a, seg_b, seg_d, seg_f, seg_g]),        // digit 5
            to_sorted_string(vec![seg_a, seg_b, seg_d, seg_e, seg_f, seg_g]), // digit 6
            to_sorted_string(vec![seg_a, seg_c, seg_f]),                      // digit 7
            to_sorted_string(vec![seg_a, seg_b, seg_c, seg_d, seg_e, seg_f, seg_g]), // digit 8
            to_sorted_string(vec![seg_a, seg_b, seg_c, seg_d, seg_f, seg_g]), // digit 9
        ])
    }

    fn solve(&self) -> Option<usize> {
        let encodings = self.find_digit_encodings()?;
        self.outputs
            .iter()
            .flat_map(|d| encodings.iter().position(|dd| dd == d))
            .reduce(|acc, x| 10 * acc + x)
    }
}
