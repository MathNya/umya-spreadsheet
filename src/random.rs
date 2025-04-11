pub(crate) fn getrandom(buf: &mut [u8]) -> Result<(), String> {
    #[cfg(feature = "runtime-rng")]
    {
        extern crate getrandom;
        match getrandom::getrandom(buf) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("getrandom error: {}", e)),
        }
    }
    #[cfg(not(feature = "runtime-rng"))]
    {
        use rand::{RngCore, SeedableRng};
        let seed = b"No Runtime RNG HERE!!!!!!!!!!111";
        let len = buf.len();
        let mut rng = rand::rngs::SmallRng::from_seed(*seed);
        for byte in buf.iter_mut() {
            *byte = rng.next_u32() as u8;
        }
        Ok(())
    }
}
