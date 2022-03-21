# A good pyomo comparison

This repository serves to verify that the results of the `cbc` solver
in `pyomo` and `good_lp` are identical when given the same model.


## `good_lp`

`cargo run`

## `pyomo`

`pyomo solve ./python/concrete_model.py --solver='cbc'`


# References

[cbc](https://www.coin-or.org/Cbc/)  
[pyomo](https://pyomo.readthedocs.io/en/stable/working_models.html)  
[good_lp](https://crates.io/crates/good_lp)  