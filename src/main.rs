use lambdaworks_math::{cyclic_group::IsGroup, elliptic_curve::{short_weierstrass::curves::bls12_381::curve::BLS12381Curve, 
    traits::IsEllipticCurve}};

fn main() {
    let g = BLS12381Curve::generator();
    let sk: u128 = 0x6C616D6264617370;
    let _pk = g.operate_with_self(sk);
}
