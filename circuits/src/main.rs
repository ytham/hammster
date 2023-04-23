#[cfg(not(target_family = "wasm"))]
use hammster::hammster::{calculate_hamming_distance, create_circuit, empty_circuit, draw_circuit, generate_setup_params, generate_keys, generate_proof, verify, run_mock_prover};

#[cfg(not(target_family = "wasm"))]
fn main() {
    let k = 6;
    let a_vec = vec![1, 1, 0, 1, 0, 1, 0, 0];
    let b_vec: Vec<u64> = vec![0, 1, 0, 0, 0, 1, 1, 0];
    let hamming_dist = calculate_hamming_distance(a_vec.clone(), b_vec.clone());

    let hammster_circuit = create_circuit(a_vec, b_vec);
    
    // Debug items
    draw_circuit(k, &hammster_circuit);
    run_mock_prover(k, &hammster_circuit, &hamming_dist);

    // Generate setup params
    let params = generate_setup_params(k);

    // Generate proving and verifying keys
    let empty_circuit = empty_circuit();
    let (pk, vk) = generate_keys(&params, &empty_circuit);

    // Generate proof
    let proof = generate_proof(&params, &pk, hammster_circuit, &hamming_dist);
    
    // Verify proof
    let verify = verify(&params, &vk, &hamming_dist, proof);
    println!("Verify result: {:?}", verify);
}