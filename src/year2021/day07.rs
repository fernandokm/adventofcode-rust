use std::str::FromStr;

use aoc::ProblemOutput;
use itertools::Itertools;

aoc::register!(solve, 2021, 7);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    // Def: given an ordered list of positions L=[x0, ..., xn] (with x0 <= x1 <= ...
    // <= xn),      the cost of a point x is:
    //          C(x, L) := sum_i(|x-xi|).
    //      A point xp is optimal if it minimizes C(x).
    //
    // Prop 1: any optimal point xp of L=[x0, ..., xn] satisfies x0 <= xp <= xn.
    //
    //      Proof: since x0 <= xi <= xn for all i, we have:
    //                  C(x0-k, L) = C(x0, L) + k*(n+1)
    //                  C(xn+k, L) = C(xn, L) + k*(n+1)
    //             which implies that points outside of [x0, xn] cannot be optimal,
    //             since their cost is always above min(C(x0, L), C(xn, L)).
    //
    // Prop 2: the optimal points of L1=[x0, ..., xn] are the same as
    //         the optimal points of L2=[x1, ..., x(n-1)].
    //
    //      Proof: let xp be an optimal point of either L1 or L2. Then,
    //             we certainly have at least x0 <= xp <= xn, implying that:
    //              C(xp, L1) = sum_i(|x-xi|)
    //                        = (xn-xp) + (xp-x0) + sum_i(|x-xi|, 1<=i<n)
    //                        = xn-x0 + C(xp, L2).
    //             Therefore, any point which minimizes the cost for L1 also
    // minimizes             the cost for L2, and vice-versa.
    //
    // Prop 3: if L=[x0, ..., xn] has an even number of elements,
    //         then the set of optimal points is [x(n//2), x(n//2+1)];
    //         otherwise, the set of optimal points is {x(n//2)}.
    //
    //      Proof: by induction.
    //
    // Corollary: the optimal cost is C(x(n//2)).

    let mut positions: Vec<_> = input.trim().split(',').map(i32::from_str).try_collect()?;
    positions.sort_unstable();

    let xp = positions[(positions.len() - 1) / 2];
    out.set_part1(positions.iter().map(|&n| (n - xp).abs()).sum::<i32>());

    let mut groups = positions
        .into_iter()
        .dedup_with_count()
        .map(|(count, position)| SubmarineGroup {
            position,
            count: count as i32,
            current_cost_of_travel: count as i32,
        })
        .collect_vec();
    let mut total_cost = 0;
    while groups.len() > 1 {
        let cost_first = groups.first().unwrap().current_cost_of_travel;
        let cost_last = groups.last().unwrap().current_cost_of_travel;
        total_cost += if cost_first <= cost_last {
            move_group(&mut groups, 0, 1, 1)
        } else {
            let groups_len = groups.len();
            move_group(&mut groups, groups_len - 1, groups_len - 2, -1)
        };
    }

    out.set_part2(total_cost);

    Ok(())
}

fn move_group(groups: &mut Vec<SubmarineGroup>, i: usize, i_next: usize, delta_pos: i32) -> i32 {
    let cost = groups[i].current_cost_of_travel;

    groups[i].current_cost_of_travel += groups[i].count;
    groups[i].position += delta_pos;
    if groups[i].position == groups[i_next].position {
        groups[i].count += groups[i_next].count;
        groups[i].current_cost_of_travel += groups[i_next].current_cost_of_travel;
        groups.remove(i_next);
    }

    cost
}

struct SubmarineGroup {
    position: i32,
    count: i32,
    current_cost_of_travel: i32,
}
