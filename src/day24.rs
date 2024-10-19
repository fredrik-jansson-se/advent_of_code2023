use nom::sequence::separated_pair;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day24.txt")?;

    println!(
        "24:1 - {}",
        run_1(&input, 200000000000000.0, 400000000000000.0)?
    );
    println!("24:2 - {}", run_2(&input)?);

    Ok(())
}

#[derive(Debug)]
struct Coord3 {
    x: f64,
    y: f64,
    _z: f64,
}

impl From<(i64, i64, i64)> for Coord3 {
    fn from((x, y, z): (i64, i64, i64)) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
            _z: z as f64,
        }
    }
}

impl From<(f64, f64, f64)> for Coord3 {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, _z: z }
    }
}

//#[derive(Debug)]
//struct Coord2 {
//    x: f64,
//    y: f64,
//}
//
//impl From<Coord3> for Coord {
//    fn from(c: Coord3) -> Self {
//        (c.x as i32, c.y as i32).into()
//    }
//}

fn parse_line(i: crate::Input) -> crate::PResult<(Coord3, Coord3)> {
    let comma_sep = |i| {
        let (i, _) = nom::bytes::complete::tag(",")(i)?;
        let (i, _) = nom::character::complete::space0(i)?;
        Ok((i, ()))
    };
    let (i, (px, (py, pz))) = separated_pair(
        nom::character::complete::i64,
        comma_sep,
        separated_pair(
            nom::character::complete::i64,
            comma_sep,
            nom::character::complete::i64,
        ),
    )(i)?;
    let (i, _) = nom::character::complete::space0(i)?;
    let (i, _) = nom::bytes::complete::tag("@")(i)?;
    let (i, _) = nom::character::complete::space0(i)?;
    let (i, (vx, (vy, vz))) = separated_pair(
        nom::character::complete::i64,
        comma_sep,
        separated_pair(
            nom::character::complete::i64,
            comma_sep,
            nom::character::complete::i64,
        ),
    )(i)?;

    Ok((i, ((px, py, pz).into(), (vx, vy, vz).into())))
}

fn parse(i: crate::Input) -> crate::PResult<Vec<(Coord3, Coord3)>> {
    let (i, res) = nom::multi::separated_list1(nom::character::complete::newline, parse_line)(i)?;
    Ok((i, res))
}

//p1 + v1*t1 = p2 + v2*t2
// |x1| - |x2| = |-v1x  v2x||t1|
// |y1| - |y2| = |-v1y  v2y||t2|
//
fn solve2((p1, v1): &(Coord3, Coord3), (p2, v2): &(Coord3, Coord3)) -> Option<Coord3> {
    let a = nalgebra::Matrix2::new(-v1.x, v2.x, -v1.y, v2.y);
    let b = nalgebra::Matrix2x1::new(p1.x - p2.x, p1.y - p2.y);
    let c = a.try_inverse()? * b;
    let t1 = c[0];
    let t2 = c[1];
    if t1 >= 0.0 && t2 >= 0.0 {
        Some((p1.x + t1 * v1.x, p1.y + t1 * v1.y, 0.0).into())
    } else {
        None
    }
}

fn run_1(input: &str, min: f64, max: f64) -> anyhow::Result<usize> {
    let (_, lines) = parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;

    let mut cnt = 0;
    for a in 0..(lines.len() - 1) {
        for b in (a + 1)..lines.len() {
            if let Some(intersect) = solve2(&lines[a], &lines[b]) {
                if (min..=max).contains(&intersect.x) && (min..=max).contains(&intersect.y) {
                    //dbg! {(&lines[a], &lines[b], &intersect)};
                    cnt += 1;
                }
            }
        }
    }
    Ok(cnt)
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn day24_intersect() {
        //let i = super::solve2(
        //    &((19, 13, 30).into(), (-2, 1, -2).into()),
        //    &((18, 19, 22).into(), (-1, -1, -2).into())).unwrap();
        //assert!((i.x - 14.333).abs() < 0.5);
        //assert!((i.y - 15.333).abs() < 0.5);
        //
        //let i = super::solve2(
        //    &((19, 13, 30).into(), (-2, 1, -2).into()),
        //    &((20, 25, 34).into(), (-2, -2, -4).into())).unwrap();
        //assert!((i.x - 11.667).abs() < 0.5);
        //assert!((i.y - 16.667).abs() < 0.5);

        let i = super::solve2(
            &((19, 13, 30).into(), (-2, 1, -2).into()),
            &((12, 31, 28).into(), (-1, -2, -1).into()),
        )
        .unwrap();
        assert!((i.x - 6.2).abs() < 0.5);
        assert!((i.y - 19.4).abs() < 0.5);

        let i = super::solve2(
            &((18, 19, 22).into(), (-1, -1, -2).into()),
            &((12, 31, 28).into(), (-1, -2, -1).into()),
        )
        .unwrap();
        assert!((i.x - -6.0).abs() < 0.5);
        assert!((i.y - -5.0).abs() < 0.5);
    }

    #[test]
    fn day24_run_1() {
        assert_eq!(super::run_1(INPUT, 7.0, 27.0).unwrap(), 2);
    }

    #[test]
    fn day24_run_2() {}
}
