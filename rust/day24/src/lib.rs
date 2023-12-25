use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{all_consuming, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hailstone {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
}

impl Hailstone {
    fn intersects_2d(&self, other: &Hailstone) -> Option<(f64, f64)> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line

        if self.pos.0 == other.pos.0 && self.pos.1 == other.pos.1 {
            return Some((self.pos.0 as f64, self.pos.1 as f64));
        }

        let (x1, y1) = (self.pos.0 as f64, self.pos.1 as f64);
        let (x2, y2) = (
            (self.pos.0 + self.vel.0) as f64,
            (self.pos.1 + self.vel.1) as f64,
        );
        let (x3, y3) = (other.pos.0 as f64, other.pos.1 as f64);
        let (x4, y4) = (
            (other.pos.0 + other.vel.0) as f64,
            (other.pos.1 + other.vel.1) as f64,
        );

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denom == 0.0 {
            return None;
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / denom;

        //dbg!(t, u);
        if t >= 0.0 && u >= 0.0 {
            Some((x1 + t * (x2 - x1), y1 + t * (y2 - y1)))
        } else {
            None
        }
    }
}

fn parse_i64(input: &str) -> IResult<&str, i64> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), str::parse::<i64>)(input)
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, (pos, vel)) = all_consuming(separated_pair(
            separated_list1(tag(","), preceded(space0, parse_i64)),
            tag(" @ "),
            separated_list1(tag(","), preceded(space0, parse_i64)),
        ))(s)
        .map_err(|_| ())?;

        if !input.is_empty() {
            return Err(());
        }

        if pos.len() != 3 || vel.len() != 3 {
            return Err(());
        }
        assert!(vel.iter().all(|&x| x != 0));

        Ok(Hailstone {
            pos: (pos[0], pos[1], pos[2]),
            vel: (vel[0], vel[1], vel[2]),
        })
    }
}

fn solve_stage1(input: &[Hailstone], limits: (i64, i64)) -> u64 {
    let mut count = 0;
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some((x, y)) = input[i].intersects_2d(&input[j]) {
                if x >= limits.0 as f64
                    && x <= limits.1 as f64
                    && y >= limits.0 as f64
                    && y <= limits.1 as f64
                {
                    //dbg!((&input[i], &input[j], x, y));
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve_stage2(input: &[Hailstone]) -> i64 {
    let x1 = input[0].pos.0 as f64;
    let y1 = input[0].pos.1 as f64;
    let z1 = input[0].pos.2 as f64;
    let vx1 = input[0].vel.0 as f64;
    let vy1 = input[0].vel.1 as f64;
    let vz1 = input[0].vel.2 as f64;
    let x2 = input[1].pos.0 as f64;
    let y2 = input[1].pos.1 as f64;
    let z2 = input[1].pos.2 as f64;
    let vx2 = input[1].vel.0 as f64;
    let vy2 = input[1].vel.1 as f64;
    let vz2 = input[1].vel.2 as f64;
    let x3 = input[2].pos.0 as f64;
    let y3 = input[2].pos.1 as f64;
    let z3 = input[2].pos.2 as f64;
    let vx3 = input[2].vel.0 as f64;
    let vy3 = input[2].vel.1 as f64;
    let vz3 = input[2].vel.2 as f64;
    // unknown vector: [x0, y0, z0, vx0, vy0, vz0, t1, t2, t3]
    // eq1: x0 + vx0 * t1 = x1 + vx1 * t1
    // eq2: y0 + vy0 * t1 = y1 + vy1 * t1
    // eq3: z0 + vz0 * t1 = z1 + vz1 * t1
    // eq4: x0 + vx0 * t2 = x2 + vx2 * t2
    // eq5: y0 + vy0 * t2 = y2 + vy2 * t2
    // eq6: z0 + vz0 * t2 = z2 + vz2 * t2
    // eq7: x0 + vx0 * t3 = x3 + vx3 * t3
    // eq8: y0 + vy0 * t3 = y3 + vy3 * t3
    // eq9: z0 + vz0 * t3 = z3 + vz3 * t3

    // eliminate t1,t2,t3 by combining eq1 & eq2, ...
    // eq1': eq1 & eq2:     x0*vy1-x0*vy0-x1*vy1+x1vy0 = y0*vx1-y0*vx0-y1*vx1+y1*vx0
    // eq2': eq1 & eq3:     x0*vz1-x0*vz0-x1*vz1+x1vz0 = z0*vx1-z0*vx0-z1*vx1+z1*vx0
    // eq3': eq4 & eq5:     x0*vy2-x0*vy0-x2*vy2+x2vy0 = y0*vx2-y0*vx0-y2*vx2+y2*vx0
    // eq4': eq4 & eq6:     x0*vz2-x0*vz0-x2*vz2+x2vz0 = z0*vx2-z0*vx0-z2*vx2+z2*vx0
    // eq5': eq7 & eq8:     x0*vy3-x0*vy0-x3*vy3+x3vy0 = y0*vx3-y0*vx0-y3*vx3+y3*vx0
    // eq6': eq7 & eq9:     x0*vz3-x0*vz0-x3*vz3+x3vz0 = z0*vx3-z0*vx0-z3*vx3+z3*vx0
    // eq7': eq2 & eq3:     y0*vz1-y0*vz0-y1*vz1+y1vz0 = z0*vy1-z0*vy0-z1*vy1+z1*vy0
    // eq8': eq6 & eq6:     y0*vz2-y0*vz0-y2*vz2+y2vz0 = z0*vy2-z0*vy0-z2*vy2+z2*vy0
    // eq9': eq8 & eq9:     y0*vz3-y0*vz0-y3*vz3+y3vz0 = z0*vy3-z0*vy0-z3*vy3+z3*vy0

    // now these equations have the same non-linear unknowns, so we can subtract them to get rid of them
    // eq1'': eq1' - eq3':  x0 * (vy1-vy2) - x1* (vy1-vy0) - x2 * (vy0-vy2) = y0 * (vx1-vx2) - y1 * (vx1-vx0) - y2 * (vx0-vx2)
    // eq2'': eq1' - eq5':  x0 * (vy1-vy3) - x1* (vy1-vy0) - x3 * (vy0-vy3) = y0 * (vx1-vx3) - y1 * (vx1-vx0) - y3 * (vx0-vx3)
    // eq3'': eq2' - eq4':  x0 * (vz1-vz2) - x1* (vz1-vz0) - x2 * (vz0-vz2) = z0 * (vx1-vx2) - z1 * (vx1-vx0) - z2 * (vx0-vx2)
    // eq4'': eq2' - eq6':  x0 * (vz1-vz3) - x1* (vz1-vz0) - x3 * (vz0-vz3) = z0 * (vx1-vx3) - z1 * (vx1-vx0) - z3 * (vx0-vx3)
    // eq5'': eq7' - eq8':  y0 * (vz1-vz2) - y1* (vz1-vz0) - y2 * (vz0-vz2) = z0 * (vy1-vy2) - z1 * (vy1-vy0) - z2 * (vy0-vy2)
    // eq6'': eq7' - eq9':  y0 * (vz1-vz3) - y1* (vz1-vz0) - y3 * (vz0-vz3) = z0 * (vy1-vy3) - z1 * (vy1-vy0) - z3 * (vy0-vy3)

    // make them a bit nicer, factoring out the variables
    // eq1'': x0 * (vy1-vy2) + y0 * (vx2-vx1) + vx0 * (y2-y1) + vy0 * (x1-x2) = -y1*vx1 + y2*vx2 + x1*vy1 - x2*vy2
    // eq2'': x0 * (vy1-vy3) + y0 * (vx3-vx1) + vx0 * (y3-y1) + vy0 * (x1-x3) = -y1*vx1 + y3*vx3 + x1*vy1 - x3*vy3
    // eq3'': x0 * (vz1-vz2) + z0 * (vx2-vx1) + vx0 * (z2-z1) + vz0 * (x1-x2) = -z1*vx1 + z2*vx2 + x1*vz1 - x2*vz2
    // eq4'': x0 * (vz1-vz3) + z0 * (vx3-vx1) + vx0 * (z3-z1) + vz0 * (x1-x3) = -z1*vx1 + z3*vx3 + x1*vz1 - x3*vz3
    // eq5'': y0 * (vz1-vz2) + z0 * (vy2-vy1) + vy0 * (z2-z1) + vz0 * (y1-y2) = -z1*vy1 + z2*vy2 + y1*vz1 - y2*vz2
    // eq6'': y0 * (vz1-vz3) + z0 * (vy3-vy1) + vy0 * (z3-z1) + vz0 * (y1-y3) = -z1*vy1 + z3*vy3 + y1*vz1 - y3*vz3

    // unkown vector [x0,y0,z0, vx0,vy0,vz0]
    // build a matrix
    let mut matrix: [[f64; 7]; 6] = [
        // eq1'': x0 * (vy1-vy2) + y0 * (vx2-vx1) + vx0 * (y2-y1) + vy0 * (x1-x2) = -y1*vx1 + y2*vx2 + x1*vy1 - x2*vy2
        [
            vy1 - vy2,
            vx2 - vx1,
            0.0,
            y2 - y1,
            x1 - x2,
            0.0,
            -y1 * vx1 + y2 * vx2 + x1 * vy1 - x2 * vy2,
        ],
        // eq2'': x0 * (vy1-vy3) + y0 * (vx3-vx1) + vx0 * (y3-y1) + vy0 * (x1-x3) = -y1*vx1 + y3*vx3 + x1*vy1 - x3*vy3
        [
            vy1 - vy3,
            vx3 - vx1,
            0.0,
            y3 - y1,
            x1 - x3,
            0.0,
            -y1 * vx1 + y3 * vx3 + x1 * vy1 - x3 * vy3,
        ],
        // eq3'': x0 * (vz1-vz2) + z0 * (vx2-vx1) + vx0 * (z2-z1) + vz0 * (x1-x2) = -z1*vx1 + z2*vx2 + x1*vz1 - x2*vz2
        [
            vz1 - vz2,
            0.0,
            vx2 - vx1,
            z2 - z1,
            0.0,
            x1 - x2,
            -z1 * vx1 + z2 * vx2 + x1 * vz1 - x2 * vz2,
        ],
        // eq4'': x0 * (vz1-vz3) + z0 * (vx3-vx1) + vx0 * (z3-z1) + vz0 * (x1-x3) = -z1*vx1 + z3*vx3 + x1*vz1 - x3*vz3
        [
            vz1 - vz3,
            0.0,
            vx3 - vx1,
            z3 - z1,
            0.0,
            x1 - x3,
            -z1 * vx1 + z3 * vx3 + x1 * vz1 - x3 * vz3,
        ],
        // eq5'': y0 * (vz1-vz2) + z0 * (vy2-vy1) + vy0 * (z2-z1) + vz0 * (y1-y2) = -z1*vy1 + z2*vy2 + y1*vz1 - y2*vz2
        [
            0.0,
            vz1 - vz2,
            vy2 - vy1,
            0.0,
            z2 - z1,
            y1 - y2,
            -z1 * vy1 + z2 * vy2 + y1 * vz1 - y2 * vz2,
        ],
        // eq6'': y0 * (vz1-vz3) + z0 * (vy3-vy1) + vy0 * (z3-z1) + vz0 * (y1-y3) = -z1*vy1 + z3*vy3 + y1*vz1 - y3*vz3
        [
            0.0,
            vz1 - vz3,
            vy3 - vy1,
            0.0,
            z3 - z1,
            y1 - y3,
            -z1 * vy1 + z3 * vy3 + y1 * vz1 - y3 * vz3,
        ],
    ];

    // gaussian elimination:
    for i in 0..matrix.len() - 1 {
        for j in i + 1..matrix.len() {
            let factor = matrix[j][i] / matrix[i][i];
            for k in i..matrix[i].len() {
                matrix[j][k] -= factor * matrix[i][k];
            }
        }
    }

    let vz0 = matrix[5][6] / matrix[5][5];
    let vy0 = (matrix[4][6] - matrix[4][5] * vz0) / matrix[4][4];
    let vx0 = (matrix[3][6] - matrix[3][4] * vy0 - matrix[3][5] * vz0) / matrix[3][3];
    let z0 = (matrix[2][6] - matrix[2][3] * vx0 - matrix[2][4] * vy0 - matrix[2][5] * vz0)
        / matrix[2][2];
    let y0 = (matrix[1][6]
        - matrix[1][2] * z0
        - matrix[1][3] * vx0
        - matrix[1][4] * vy0
        - matrix[1][5] * vz0)
        / matrix[1][1];
    let x0 = (matrix[0][6]
        - matrix[0][1] * y0
        - matrix[0][2] * z0
        - matrix[0][3] * vx0
        - matrix[0][4] * vy0
        - matrix[0][5] * vz0)
        / matrix[0][0];

    let x0 = x0.round() as i64;
    let y0 = y0.round() as i64;
    let z0 = z0.round() as i64;
    x0 + y0 + z0
}

pub struct Day24Solver;
impl AdventOfCodeDay<'_> for Day24Solver {
    type ParsedInput = Vec<Hailstone>;

    type Part1Output = u64;

    type Part2Output = i64;

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        solve_stage1(input, (200000000000000, 400000000000000))
    }

    fn solve_part2(input: &Self::ParsedInput) -> Self::Part2Output {
        solve_stage2(input)
    }

    fn parse_input(input: &str) -> Self::ParsedInput {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day24Solver;

    const TEST_INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
    #[test]
    fn test_stage1() {
        let input = Day24Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage1(&input, (7, 27)), 2);
    }
    #[test]
    fn test_stage2() {
        let input = Day24Solver::parse_input(TEST_INPUT);
        assert_eq!(super::solve_stage2(&input), 47);
    }
}
