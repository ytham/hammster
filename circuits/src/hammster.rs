use std::{
    marker::PhantomData,
};

use halo2_proofs::{
    arithmetic::Field, 
    dev::MockProver,
    circuit::{Layouter, Chip, Value, AssignedCell, Region, SimpleFloorPlanner}, 
    plonk::{Column, Advice, Instance, Error, Selector, ConstraintSystem, Circuit, Expression, create_proof, keygen_vk, keygen_pk, ProvingKey, VerifyingKey, verify_proof, SingleVerifier}, 
    poly::{Rotation, commitment::Params}, 
    pasta::{Fp, EqAffine}, transcript::{Blake2bWrite, Challenge255, Blake2bRead}, 
};
use rand_core::OsRng;

const BINARY_LENGTH: usize = 8;

trait Instructions<F: Field>: Chip<F> {
    type Num;

    fn load_private_and_check_binary(&self, layouter: impl Layouter<F>, column: usize, value: [Value<F>; BINARY_LENGTH]) -> Result<Vec<Self::Num>, Error>;

    fn xor(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;

    fn accumulator(&self, layouter: impl Layouter<F>, values: [Self::Num; BINARY_LENGTH]) -> Result<Self::Num, Error>;

    fn expose_public(&self, layouter: impl Layouter<F>, num: Self::Num) -> Result<(), Error>;
}

pub struct HammsterChip<F: Field> {
    config: HammsterConfig,
    _marker: PhantomData<F>,
}

impl<F: Field> Chip<F> for HammsterChip<F> {
    type Config = HammsterConfig;
    type Loaded = ();

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

#[derive(Debug, Clone)]
pub struct HammsterConfig {
    advice: [Column<Advice>; 3],
    instance: Column<Instance>,
    s_binary_l: Selector,
    s_binary_r: Selector,
    s_xor: Selector,
    s_accumulator: Selector,
}

impl<F: Field> HammsterChip<F> {
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Self { 
            config, 
            _marker: PhantomData, 
        }
    }

    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 3],
        instance: Column<Instance>,
    ) -> <Self as Chip<F>>::Config {
        meta.enable_equality(instance);
        for column in &advice {
            meta.enable_equality(*column);
        }
        let s_binary_l = meta.selector();
        let s_binary_r = meta.selector();
        let s_xor = meta.selector();
        let s_accumulator = meta.selector();

        meta.create_gate("is binary left", |meta| {
            let value = meta.query_advice(advice[0], Rotation::cur());
            let s_binary_l = meta.query_selector(s_binary_l);

            vec![s_binary_l * (value.clone() * (Expression::Constant(F::ONE) - value))]
        });

        meta.create_gate("is binary right", |meta| {
            let value = meta.query_advice(advice[1], Rotation::cur());
            let s_binary_r = meta.query_selector(s_binary_r);

            vec![s_binary_r * (value.clone() * (Expression::Constant(F::ONE) - value))]
        });

        meta.create_gate("xor", |meta| {
            let lhs = meta.query_advice(advice[0], Rotation::cur());
            let rhs = meta.query_advice(advice[1], Rotation::cur());
            let out = meta.query_advice(advice[2], Rotation::cur());
            let s_xor = meta.query_selector(s_xor);

            vec![s_xor * ((lhs * out.clone()) + (rhs * out.clone()) - out)]
        });

        meta.create_gate("accumulator", |meta| {
            let inputs_sum = (0..BINARY_LENGTH)
                .map(|i| meta.query_advice(advice[2], Rotation((i as i32) - (BINARY_LENGTH as i32))))
                .fold(Expression::Constant(F::ZERO), |acc, e| acc + e);
            let sum = meta.query_advice(advice[2], Rotation::cur());
            let s_accumulator = meta.query_selector(s_accumulator);

            vec![s_accumulator * (inputs_sum - sum)]
        });

        HammsterConfig {
            advice,
            instance,
            s_binary_l,
            s_binary_r,
            s_xor,
            s_accumulator,
        }
    }
}

#[derive(Clone, Debug)]
struct Number<F: Field>(AssignedCell<F, F>);

impl<F: Field> Instructions<F> for HammsterChip<F> {
    type Num = Number<F>;

    fn load_private_and_check_binary(&self, mut layouter: impl Layouter<F>, column: usize, values: [Value<F>; BINARY_LENGTH]) -> Result<Vec<Self::Num>, Error> {
        let config = self.config();

        layouter.assign_region(
            || "assign private values", 
            |mut region| {
                values
                    .iter()
                    .enumerate()
                    .map(|(i,value)| {
                        if column == 0 {
                            config.s_binary_l.enable(&mut region, i)?;
                        } else {
                            config.s_binary_r.enable(&mut region, i)?;
                        }
                        region
                            .assign_advice(|| "assign private input", config.advice[column], i, || *value)
                            .map(Number)
                        }
                    )
                    .collect()
            }
        )
    }

    fn xor(&self, mut layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "assign xor region", 
            |mut region: Region<'_, F>| {
                config.s_xor.enable(&mut region, 0)?;

                let a_val  = a.0.copy_advice(|| "lhs", &mut region, config.advice[0], 0)?;
                let b_val = b.0.copy_advice(|| "rhs", &mut region, config.advice[1], 0)?;

                let xor_result = a_val.value()
                    .zip(b_val.value())
                    .map(|(a, b)| if *a == *b { F::ZERO } else { F::ONE });

                region
                    .assign_advice(|| "a xor b", config.advice[2], 0, || xor_result)
                    .map(Number)
            },
        )
    }

    fn accumulator(&self, mut layouter: impl Layouter<F>, values: [Self::Num; BINARY_LENGTH]) -> Result<Self::Num, Error> {
        let config = self.config();

        layouter.assign_region(
            || "assign accumulator region", 
            |mut region: Region<'_, F>| {
                config.s_accumulator.enable(&mut region, BINARY_LENGTH)?;

                for (i, value) in values.iter().enumerate() {
                    (*value).0.copy_advice(|| format!("output[{}]", i), &mut region, config.advice[2], i)?;
                }

                let accumulation = values
                    .iter()
                    .map(|n| n.0.value().copied())
                    .fold(Value::known(F::ZERO), |acc, e| acc + e);

                region
                    .assign_advice(|| "accumulation result", config.advice[2], BINARY_LENGTH, || accumulation)
                    .map(Number)
            }
        )
    }

    // Constrain the accumulation value from advice[2], row BINARY_LENGTH to equal instance column value in row 0
    fn expose_public(&self, mut layouter: impl Layouter<F>, num: Self::Num) -> Result<(), Error> {
        let config = self.config();
        // Ok(())
        layouter.constrain_instance(num.0.cell(), config.instance, 0)
    }
}

#[derive(Default)]
pub struct HammsterCircuit<F: Field> {
    a: [Value<F>; BINARY_LENGTH],
    b: [Value<F>; BINARY_LENGTH],
}

impl<F: Field> Circuit<F> for HammsterCircuit<F> {
    type Config = HammsterConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice = [meta.advice_column(), meta.advice_column(), meta.advice_column()];
        let instance = meta.instance_column();

        HammsterChip::configure(meta, advice, instance)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let hammster_chip = HammsterChip::<F>::construct(config);

        // Load private variable vectors & check if they're binary
        let a = hammster_chip.load_private_and_check_binary(layouter.namespace(|| "load a"), 0, self.a)?;
        let b = hammster_chip.load_private_and_check_binary(layouter.namespace(|| "load b"), 1, self.b)?;

        // Perform XOR on each row
        let xor_results: Vec<Number<F>> = (0..BINARY_LENGTH)
            .map(|i| {
                hammster_chip.xor(layouter.namespace(|| format!("xor[{}]", i)), a[i].clone(), b[i].clone()).unwrap()
            })
            .collect();
        let xor_slice: [Number<F>; 8] = xor_results.clone().try_into().unwrap();

        // Accumulate the results of the XOR output column
        let accumulate = hammster_chip.accumulator(layouter.namespace(|| "accumulate xor results"), xor_slice)?;

        // Ensure the accumulated value equals the public input (of the precalculated accumulation value)
        hammster_chip.expose_public(layouter.namespace(|| "expose accumulate"), accumulate)
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn draw_circuit<F: Field>(k: u32, circuit: &HammsterCircuit<F>) {
    use plotters::prelude::*;
    let base = BitMapBackend::new("layout.png", (1600,1600)).into_drawing_area();
    base.fill(&WHITE).unwrap();
    let base = base.titled("Hammster Circuit", ("sans-serif", 24)).unwrap();

    halo2_proofs::dev::CircuitLayout::default()
        .show_equality_constraints(true)
        .render(k, circuit, &base)
        .unwrap();
}

pub fn empty_circuit() -> HammsterCircuit<Fp> {
    HammsterCircuit {
        a: [Value::unknown(); BINARY_LENGTH],
        b: [Value::unknown(); BINARY_LENGTH],
    }
}

pub fn create_circuit(a: Vec<u64>, b: Vec<u64>) -> HammsterCircuit<Fp> {
    // Put inputs into circuit-friendly form
    let a_vec: [Value<Fp>; 8] = a
        .clone()
        .iter()
        .map(|f| Value::known(Fp::from(*f)))
        .collect::<Vec<Value<Fp>>>()
        .try_into()
        .unwrap();

    let b_vec: [Value<Fp>; 8] = b
        .clone()
        .iter()
        .map(|f| Value::known(Fp::from(*f)))
        .collect::<Vec<Value<Fp>>>()
        .try_into()
        .unwrap();

    // Create circuit from inputs
    HammsterCircuit {
        a: a_vec,
        b: b_vec,
    }
}

pub fn calculate_hamming_distance(a: Vec<u64>, b: Vec<u64>) -> Vec<Fp> {
    let hamming_dist = a
        .clone()
        .iter()
        .enumerate()
        .map(|(i, x)| (x + b[i]) % 2)
        .fold(0, |acc, x| acc + x);
    vec![Fp::from(hamming_dist)]
}

pub fn run_mock_prover(
    k: u32,
    circuit: &HammsterCircuit<Fp>,
    pub_input: &Vec<Fp>,
) {
    let prover = MockProver::run(k, circuit, vec![pub_input.clone()]).expect("Mock prover should run");
    let res = prover.verify();
    match res {
        Ok(()) => println!("MockProver OK"),
        Err(e) => println!("err {:#?}", e),
    }
}

pub fn generate_setup_params(
    k: u32,
) -> Params<EqAffine> {
    // Generate a universal trusted setup of our own for testing
    Params::<EqAffine>::new(k)
}

pub fn generate_keys(
    params: &Params<EqAffine>,
    circuit: &HammsterCircuit<Fp>,
) -> (ProvingKey<EqAffine>, VerifyingKey<EqAffine>) {
    // just to emphasize that for vk, pk we don't need to know the value of `x`
    let vk = keygen_vk(params, circuit).expect("vk should not fail");
    let pk = keygen_pk(params, vk.clone(), circuit).expect("pk should not fail");
    (pk, vk)
}

pub fn generate_proof(
    params: &Params<EqAffine>,
    pk: &ProvingKey<EqAffine>,
    circuit: HammsterCircuit<Fp>,
    pub_input: &Vec<Fp>,
) -> Vec<u8> {
    println!("Generating proof...");
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
    create_proof(
        params, 
        pk, 
        &[circuit],
        &[&[pub_input]], 
        OsRng, 
        &mut transcript
    ).expect("Prover should not fail");
    let proof = transcript.finalize();
    proof
}

pub fn verify(
    params: &Params<EqAffine>,
    vk: &VerifyingKey<EqAffine>,
    pub_input: &Vec<Fp>,
    proof: Vec<u8>,
) -> Result<(), Error> {
    println!("Verifying proof...");
    let strategy = SingleVerifier::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
    let verify_res = verify_proof(
        params, 
        vk, 
        strategy, 
        &[&[pub_input]], 
        &mut transcript,
    );
    
    verify_res
}