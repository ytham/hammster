use std::io::BufReader;
use crate::hammster::{calculate_hamming_distance, create_circuit, empty_circuit, generate_setup_params, generate_keys, generate_proof, verify};
use halo2_proofs::{
    poly::commitment::Params, 
    pasta::{Fp, EqAffine}, 
    plonk::keygen_vk
};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn copy_vec_to_u8arr(v: &Vec<u8>) -> Uint8Array {
    let u8_arr = Uint8Array::new_with_length(v.len() as u32);
    u8_arr.copy_from(v);
    u8_arr
}

#[wasm_bindgen]
pub fn setup_params(k: u32) -> Uint8Array {
    log("running setup");
    
    // Generate setup params
    let params = generate_setup_params(k); 
    let mut buf = vec![];
    params.write(&mut buf).expect("Can write params");

    copy_vec_to_u8arr(&buf)
}

#[wasm_bindgen]
pub fn proof_generate(
    a: &[u8], 
    b: &[u8], 
    params_bytes: &[u8]
) -> Uint8Array {
    log("proving...");
    
    let params = Params::<EqAffine>::read(&mut BufReader::new(params_bytes)).expect("params should not fail to read");

    // Turn slices into vectors and calculate hamming distance
    let a_vec: Vec<u64> = a.to_vec().iter().map(|x| *x as u64).collect();
    let b_vec: Vec<u64> = b.to_vec().iter().map(|x| *x as u64).collect();
    let hamming_dist = calculate_hamming_distance(a_vec.clone(), b_vec.clone());

    // Generate proving key
    let empty_circuit = empty_circuit();
    let (pk, _vk) = generate_keys(&params, &empty_circuit);
    
    // Generate proof
    let hammster_circuit = create_circuit(a_vec, b_vec);
    let proof = generate_proof(&params, &pk, hammster_circuit, &hamming_dist);
    
    copy_vec_to_u8arr(&proof)
}

#[wasm_bindgen]
pub fn proof_verify(
    params_bytes: &[u8], 
    hamming_dist: u32, 
    proof: &[u8]
) -> bool {
    log("verifying...");

    let params = Params::<EqAffine>::read(&mut BufReader::new(params_bytes)).expect("params should not fail to read");

    // Generate verifying key
    let empty_circuit = empty_circuit();
    let vk = keygen_vk(&params, &empty_circuit).expect("vk should not fail to generate");

    // Transform params for verify function
    let hamming_dist_fp = vec![Fp::from(hamming_dist as u64)];
    let proof_vec = proof.to_vec();

    // Verify the proof and public input
    let ret_val = verify(&params, &vk, &hamming_dist_fp, proof_vec);
    match ret_val {
        Err(_) => false,
        _ => true,
    }
}
