use std::collections::HashMap;

struct Molecule {
    atoms: HashMap<&'static str, i32>,
}

impl Molecule {
    fn new(atoms: &[(&'static str, i32)]) -> Self {
        Self {
            atoms: atoms.iter().cloned().collect(),
        }
    }
}

struct Term {
    coeff: i32,
    molecule: Molecule,
}

struct Equation {
    reactants: Vec<Term>,
    products: Vec<Term>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn h2_test() {
        // 2 H -> H2
        let h = Molecule::new(&[("H", 1)]);
        let h2   = Molecule::new(&[("H", 2)]);
        let equation = Equation::new(vec![h], vec![h2]);
        assert_eq!(equation.reactants[0].coeff, 2);
        assert_eq!(equation.products[0].coeff, 1);
    }

    #[test]
    fn h_test() {
        // H2 -> 2 H
        let h2   = Molecule::new(&[("H", 2)]);
        let h = Molecule::new(&[("H", 1)]);
        let equation = Equation::new(vec![h2], vec![h]);
        assert_eq!(equation.reactants[0].coeff, 1);
        assert_eq!(equation.products[0].coeff, 2);
    }

    #[test]
    fn k3po4_test() {
        // H3PO4 + 3 KOH -> K3PO4 + 3 H2O
        let h3po4 = Molecule::new(&[("H", 3), ("P", 1), ("O", 4)]);
        let koh   = Molecule::new(&[("K", 1), ("O", 1), ("H", 1)]);
        let k3po4 = Molecule::new(&[("K", 3), ("P", 1), ("O", 4)]);
        let h2o   = Molecule::new(&[("H", 2), ("O", 1)]);
        let equation = Equation::new(vec![h3po4, koh], vec![k3po4, h2o]);
        assert_eq!(equation.reactants[0].coeff, 1);
        assert_eq!(equation.reactants[1].coeff, 3);
        assert_eq!(equation.products[0].coeff, 1);
        assert_eq!(equation.products[1].coeff, 3);
    }

    #[test]
    fn n2o5_test() {
        // 2 N2 + 5 O2 -> 2 N2O5
        let n2 = Molecule::new(&[("N", 2)]);
        let o2   = Molecule::new(&[("O", 2)]);
        let n2o5 = Molecule::new(&[("N", 2), ("O", 5)]);
        let equation = Equation::new(vec![n2, o2], vec![n2o5]);
        assert_eq!(equation.reactants[0].coeff, 2);
        assert_eq!(equation.reactants[1].coeff, 5);
        assert_eq!(equation.products[0].coeff, 2);
    }

}

