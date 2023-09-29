use clap::Parser;
use halo2_base::gates::{RangeChip, GateInstructions, RangeInstructions};
use halo2_base::utils::ScalarField;
use halo2_base::AssignedValue;
#[allow(unused_imports)]
use halo2_base::{
    Context,
    QuantumCell::{Constant, Existing, Witness},
};
use halo2_scaffold::scaffold::cmd::Cli;
use halo2_scaffold::scaffold::run;
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub x: String, // field element, but easier to deserialize as a string
}

fn test_circuit3_zk<F: ScalarField>(
    ctx: &mut Context<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    // lookup bits must agree with the size of the lookup table, which is specified by an environmental variable
    let lookup_bits =
        var("LOOKUP_BITS").unwrap_or_else(|_| panic!("LOOKUP_BITS not set")).parse().unwrap();

    let x = F::from_str_vartime(&input.x).expect("deserialize field element should not fail");

    let x = ctx.load_witness(x);// RHS: x: F. LHS x: contains both the value of x and its location in the table

    make_public.push(x); // Make it public so verifier also has access. default: all inputs (witnesses) are private

    let range = RangeChip::default(lookup_bits);

    let c = F::from(32);

    // check that `x` is in [0, 2^16)
    range.range_check(ctx, x, 16);

    let out = range.gate().div_unsafe(ctx,x,Constant(c));

    println!("x: {:?}", x.value());
    println!("val_assigned: {:?}", out.value());
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    // run different zk commands based on the command line arguments
    run(test_circuit3_zk, args);
}

