extern crate clebsch;
extern crate num;
use clebsch::clebschgordansq;
use num::traits::Signed;
use num::iter::range_step_inclusive;
use std::io::{self, Write};
use std::fs::File;

fn test<F: Write>(mut file: F) -> io::Result<()>
{
    for tj1 in 0..10
    {
        let j1 = tj1 as f32 * 0.5;
        for tj2 in 0..10
        {
            let j2 = tj2 as f32 * 0.5;
            for tj12 in range_step_inclusive((tj1 - tj2).abs(), tj1 + tj2, 2)
            {
                let j12 = tj12 as f32 * 0.5;
                for tm1 in range_step_inclusive(-tj1, tj1, 2)
                {
                    let m1 = tm1 as f32 * 0.5;
                    for tm2 in range_step_inclusive(-tj2, tj2, 2)
                    {
                        let m2 = tm2 as f32 * 0.5;
                        let tm12 = tm1 + tm2;
                        let m12 = m1 + m2;
                        if (tj1 + tj2 + tj12) % 2 != 0 { continue }
                        let (s, r) = clebschgordansq(j1, m1, j2, m2, j12, m12);
                        try!(writeln!(file, "{} {} {} {} {} {} {} {}", tj1, tm1, tj2, tm2, tj12, tm12, s, r));
                    }
                }
            }
        }
    }
    Ok(())
}

fn main()
{
    File::create("out.txt").and_then(|f| test(f)).unwrap();
}
