use pairing::bn256::Fr;
use group::ff::Field;

pub fn add_fields(a: Fr, b: Fr) -> Fr {
    let mut result = a;
    result.add_assign(&b);
    result
}
