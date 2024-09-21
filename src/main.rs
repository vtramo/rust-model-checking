use rust_model_checking::{LtlFormula::{*}, LtlFormula, OnTheFlyLtl};

fn main() {

    let ltl_formula = LtlFormula::release(Bottom, Atom::<u8>(1));
    // let ltl_formula = LtlFormula::until(LtlFormula::Atom::<u8>(1), LtlFormula::Atom::<u8>(2));
    let mut on_the_fly_ltl = OnTheFlyLtl::new();
    on_the_fly_ltl.create_graph(&ltl_formula);

    println!("{:#?}", on_the_fly_ltl);

}