use clap::Parser;
use halo2_base::gates::{GateChip, GateInstructions};
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub x: String, // field element, but easier to deserialize as a string
}

fn test_circuit_zk<F: ScalarField>(
    ctx: &mut Context<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    let x = F::from_str_vartime(&input.x).expect("deserialize field element should not fail");

    let x = ctx.load_witness(x);// RHS: x: F. LHS x: contains both the value of x and its location in the table

    make_public.push(x); // Make it public so verifier also has access. default: all inputs (witnesses) are private

    // create a Gate chip that contains methods for basic arithmetic operations
    let gate = GateChip::<F>::default();

    let c = F::from(24);

    let out = gate.mul_add(ctx,x,x,Constant(c));

    println!("x: {:?}", x.value());
    println!("val_assigned: {:?}", out.value());
    assert_eq!(*x.value() * x.value() + c, *out.value());
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    // run different zk commands based on the command line arguments
    run(test_circuit_zk, args);
}