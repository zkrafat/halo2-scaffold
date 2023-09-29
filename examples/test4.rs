use clap::Parser;
use halo2_base::gates::{GateChip, GateInstructions };
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
    pub arr: Vec<u8>, // input bytes, right padded with arbitrary bytes to fixed `MAX_LEN`
    pub len: usize,            // the variable length of the input bytes
}

fn test_circuit3_zk<F: ScalarField>(
    ctx: &mut Context<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    // load the input
    let arr = ctx.assign_witnesses(input.arr.iter().map(|b| F::from(*b as u64)));
    let _max_len = arr.len();
    let len = ctx.load_witness(F::from(input.len as u64));

}