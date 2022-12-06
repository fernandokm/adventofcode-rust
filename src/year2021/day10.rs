use anyhow::bail;
use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 10);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    let mut syntax_error_score = 0;
    let mut autocomplete_scores = Vec::new();
    'outer: for line in input.trim().lines() {
        let mut close_tokens = Vec::new();
        for c in line.chars() {
            if let Some(ct) = get_close_token(c) {
                close_tokens.push(ct);
            } else if close_tokens.pop() != Some(c) {
                syntax_error_score += token_syntax_error_score(c)?;
                continue 'outer;
            }
        }
        autocomplete_scores.push(
            close_tokens
                .into_iter()
                .rev()
                .map(token_autocomplete_score)
                .fold_ok(0, |acc, x| 5 * acc + x)?,
        );
    }

    out.set_part1(syntax_error_score);

    autocomplete_scores.sort_unstable();
    out.set_part2(autocomplete_scores[(autocomplete_scores.len() - 1) / 2]);

    Ok(())
}

fn get_close_token(open_token: char) -> Option<char> {
    match open_token {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn token_syntax_error_score(close_token: char) -> anyhow::Result<u64> {
    match close_token {
        ')' => Ok(3),
        ']' => Ok(57),
        '}' => Ok(1197),
        '>' => Ok(25137),
        _ => bail!("invalid token: {}", close_token),
    }
}

fn token_autocomplete_score(close_token: char) -> anyhow::Result<u64> {
    match close_token {
        ')' => Ok(1),
        ']' => Ok(2),
        '}' => Ok(3),
        '>' => Ok(4),
        _ => bail!("invalid token: {}", close_token),
    }
}
