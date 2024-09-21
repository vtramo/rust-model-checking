use crate::LtlFormula::{*};

#[derive(Hash, Clone, Eq, PartialEq, Debug)]
pub enum LtlFormula<AP> {
    Atom(AP),
    Not(Box<LtlFormula<AP>>),
    Or(Box<LtlFormula<AP>>, Box<LtlFormula<AP>>),
    And(Box<LtlFormula<AP>>, Box<LtlFormula<AP>>),
    Next(Box<LtlFormula<AP>>),
    Until(Box<LtlFormula<AP>>, Box<LtlFormula<AP>>),
    Release(Box<LtlFormula<AP>>, Box<LtlFormula<AP>>),
    Top,
    Bottom
}

impl<AP> LtlFormula<AP> {
    pub fn atom(value: AP) -> Self {
        Atom(value)
    }

    pub fn not(ltl_formula: Self) -> Self {
        Not(Box::new(ltl_formula))
    }

    pub fn or(ltl_formula_left: Self, ltl_formula_right: Self) -> Self {
        Or(Box::new(ltl_formula_left), Box::new(ltl_formula_right))
    }

    pub fn and(ltl_formula_left: Self, ltl_formula_right: Self) -> Self {
        And(Box::new(ltl_formula_left), Box::new(ltl_formula_right))
    }

    pub fn next(ltl_formula: Self) -> Self {
        Next(Box::new(ltl_formula))
    }

    pub fn until(ltl_formula_left: Self, ltl_formula_right: Self) -> Self {
        Until(Box::new(ltl_formula_left), Box::new(ltl_formula_right))
    }

    pub fn release(ltl_formula_left: Self, ltl_formula_right: Self) -> Self {
        Release(Box::new(ltl_formula_left), Box::new(ltl_formula_right))
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Atom(_) | Top | Bottom => true,
            Not(formula) if formula.is_literal() => true,
            _ => false
        }
    }
}
