use rayon::prelude::*;

pub fn entropy(file: Vec<u8>) {
    let symbol_count = file.len();

    let mut occurences: Vec<u32> =  vec![0; 256];
    for symbol in &file {
        occurences[*symbol as usize] += 1;
    }
    //println!("{:?}", &occurences);
    let probability: Vec<f32> = occurences.par_iter().map(|x| (*x as f32) / (symbol_count as f32)).collect();
    //println!("{:?}", &probability);

    //conditional_occurences[condition][symbol]
    let mut conditional_occurences = vec![vec![0_u32; 256]; 256];
    let mut prev = 0_u8;
    for symbol in &file {
        conditional_occurences[prev as usize][*symbol as usize] += 1;
        prev = *symbol;
    }
    //println!("{:?}", &conditional_occurences);
    let condition_sums: Vec<u32> = conditional_occurences.par_iter().map(|row| row.par_iter().sum::<u32>()).collect();
    //println!("{:?}", &sums);
    let mut conditional_probability = vec![vec![0_f32; 256]; 256];
    for (i, sum ) in condition_sums.iter().enumerate().take(256) {
        for j in 0..256 {
            if *sum != 0 {
                conditional_probability[j][i] = (conditional_occurences[j][i] as f32) / *sum as f32;
            } else {
                conditional_probability[j][i] = 0.0;   
            }
        }
    }
    //println!("{:?}", &conditional_probability);


    let entropy: f32 = probability.par_iter().fold(|| 0., |e: f32, x| if *x == 0.0 {e} else {e - (x * x.log2())}).sum();
    println!("entropy: {:?}", entropy);

    let mut conditional_entropy = 0_f32;
    let mut tmp = 0_f32;
    for x in 0..256 {
        for y in 0..256 {
            if conditional_probability[y][x] != 0.0 {
                tmp -= conditional_probability[y][x] * conditional_probability[y][x].log2()
            }
        }
        conditional_entropy += probability[x] * tmp;
        tmp = 0.;
    }
    println!("conditional_entropy: {:?}", conditional_entropy);

    println!("diff: {:?}", entropy - conditional_entropy)
}
