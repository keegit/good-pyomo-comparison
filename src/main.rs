use std::error::Error;

use good_lp::{constraint, coin_cbc, SolverModel, variables};

fn main() -> Result<(), Box<dyn Error>> {
    variables! {
        vars:
               0 <= x1;
               0 <= x2;
    }
    let _solution = vars.minimise(2 * x1 + 3 * x2)
        .using(coin_cbc)
        .with(constraint!((3 * x1) + (4 * x2) >= 1))
        .solve()?;

    Ok(())
}
