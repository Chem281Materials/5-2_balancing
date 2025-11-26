# Balancing Chemical Equations in Rust

In this problem, you will modify the contents of `main.rs` to initialize balanced chemical equations.


## Terminology

Consider the following chemical equation:

```
2 N2 + 5 O2 -> 2 N2O5
```

We will call `N2`, `O2`, and `N2O5` the molecules.
We will call `2 N2`, `5 O2`, and `2 N2O5` the terms of the equation, which in this case have coefficients of `2`, `5`, and `2`, respectively.

## Task

The `main.rs` file includes structs for `Molecules`, `Terms`, and `Equations`, following the definitions above.
It also includes some tests that attempt to create some new equations, and then test that they are balanced.
Currently, these tests fail because there is no implementation of `Equation::new`.
Implement `Equation::new` so that it properly initializes and balances new equations.
The method signature for `Equation::new` should be selected in such a way that the tests are valid as written.

Balancing chemical equations in an automated fashion typically involves solving a system of linear equations; however, for the purpose of this problem, you are permitted to use brute force trial-and-error.
You may also assume that the term coefficients will never be greater than 5.

Do not modify the tests.
