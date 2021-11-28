use anyhow::Context;
use aoc::ProblemOutput;

use crate::util::math;

aoc::register!(solve, 2020, 13);

pub fn solve(input: &str, out: &mut ProblemOutput) -> anyhow::Result<()> {
    let (start, ids) = input.trim().split_once('\n').context("invalid input")?;
    let start: u64 = start.parse()?;
    let ids: Vec<Option<u64>> = ids.split(',').map(|id| id.parse().ok()).collect();

    let (time, id) = ids
        .iter()
        .filter_map(|&x| x)
        .map(|id| (next_multiple(start, id), id))
        .min()
        .context("input has no valid bus ids")?;
    out.set_part1((time - start) * id);

    /* Remark: part 2 can be solved using the Chinese Remainder Theorem (CRT).
     * This solution does not use the CRT.
     *
     * For part 2, we want to find t0 = k*id[0], where k is an integer s.t.:
     *     k*ids[0]+j == kj*ids[j]                                   (1)
     * for some kj, for all indices j which hold valid buses.
     *
     * Let j,k be such that (1) holds. Suppose that (1) also holds for j,k+Δk.
     * Then:
     *     k*ids[0]+j == kj*ids[j]                                   (1.a)
     *     (k+Δk)*ids[0]+j == (kj+Δkj)*ids[j]                        (1.b)
     * By subtracting (1.a) from (1.b), and simplifying the result, we obtain:
     *     Δk*ids[0] == Δkj*ids[j],                                  (2)
     * i.e. Δk*ids[0] must be a multiple of ids[j].
     *
     * Conversely, if (1.a) and (2) hold, then:
     *     (k+Δk)*ids[0]+j == kj*ids[j] + Δk*ids[0]
     *                     == kj*ids[j] + Δkj*ids[j]
     *                     == (kj+Δkj)*ids[j]
     * so that (1.b) is true.
     *
     * Therefore, given j,k which solve (1), j,k+Δk also solves (1) iff Δk*ids[0]
     * is a multiple of ids[j], i.e., Δk is a multiple of dk=ids[j]/gcd(ids[j], ids[0]).
     *
     * This means that there exists some k0, 0 <= k0 < dk, such that the
     * set of solutions to (1) with fixed j is exactly:
     *     {k0 + a*dk : a integer}
     * with k0 being the smallest non-negative solution.
     *
     * This gives the following algorithm:
     *  - k = 1
     *  - step = 1
     *  - for each j >= 1 which holds a valid bus:
     *    - dk = ids[j]/gcd(ids[j], ids[0])
     *    - for k0 in 0..dk:
     *      - new_k = (k..).step_by(step).find(|x| x % dk == k0)
     *      - if (1) holds for new_k:
     *        - k = new_k
     *        - step = lcm(step, m)
     *        - break the inner loop
     */

    let id0 = ids[0].unwrap();
    let mut k = 1;
    let mut step = 1;

    ids.iter()
        .enumerate()
        .skip(1)
        .filter_map(|(j, idj)| idj.and_then(|id| Some((j, id))))
        .try_for_each(|(j, idj)| -> anyhow::Result<()> {
            let dk = idj / math::gcd(idj, id0);
            k = (0..dk)
                .map(|k0| (k..).step_by(step).find(|&new_k| new_k % dk == k0).unwrap())
                .find(|&new_k| (new_k * id0 + j as u64) % idj == 0)
                .context(format!("no new_k found for id {}", idj))?;
            step = math::lcm(step, dk as usize);
            Ok(())
        })?;

    out.set_part2(k * id0);

    Ok(())
}

fn next_multiple(n: u64, factor: u64) -> u64 {
    // The code below is a branchless version of:
    //
    //     if n % factor == 0 {
    //         n
    //     } else {
    //         n + factor - n % factor
    //     }
    //
    // i.e.:
    //     if `n` is already a multiple of `factor`, return it
    //     otherwise, subtract the excess (`n % factor`)
    //     and add `factor` to ensure that the result is >= n

    debug_assert!(n != 0);

    n + factor - (n - 1) % factor - 1
}
