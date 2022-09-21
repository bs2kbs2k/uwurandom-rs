pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {

    use crate::keysmash;

    use super::catgirl_nonsense;

    #[test]
    fn catgirl_nonsense_test() {
        // Use a stable-algorithm RNG with fixed seed
        let mut rng = rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let mut state_machine = catgirl_nonsense::StateMachine::Ny;
        let mut result = String::from("ny");
        for _ in 0..100 {
            let (new_state, generated) = state_machine.generate(&mut rng);
            result.push(generated);
            state_machine = new_state;
        }
        assert_eq!(&result, "nyaaaameowmrowrmrowmrrmeowmrowmeownyanyaaaaaaaaaaaaamraowrmeowwwmeowmraowmrowmrowmeowmeowrnyamreownyaa");
    }

    #[test]
    fn keysmash_test() {
        // Use a stable-algorithm RNG with fixed seed
        let mut rng = rand_pcg::Pcg32::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7);
        let mut state_machine = keysmash::StateMachine::A;
        let mut result = String::from("a");
        for _ in 0..100 {
            let (new_state, generated) = state_machine.generate(&mut rng);
            result.push(generated);
            state_machine = new_state;
        }
        assert_eq!(&result, "ajhfhiurgjfgajhnghgadfghkfghiurgjeghnhgjalkjfhgnhrgjhnhiuradfdbahrgbafhg;djkafgjhjrajfjdfghfhdfgajgad");
    }
}

mod catgirl_nonsense;
mod keysmash;
