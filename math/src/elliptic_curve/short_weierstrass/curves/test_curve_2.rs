use crate::elliptic_curve::short_weierstrass::point::ShortWeierstrassProjectivePoint;
use crate::elliptic_curve::traits::IsEllipticCurve;
use crate::field::fields::u384_prime_field::{
    IsMontgomeryConfiguration, MontgomeryBackendPrimeField,
};
use crate::unsigned_integer::element::U384;
use crate::{
    elliptic_curve::short_weierstrass::traits::IsShortWeierstrass,
    field::{
        element::FieldElement,
        extensions::quadratic::{HasQuadraticNonResidue, QuadraticExtensionField},
    },
};

/// Order of the base field (e.g.: order of the coordinates)
pub const TEST_CURVE_2_PRIME_FIELD_ORDER: U384 = U384::from("150b4c0967215604b841bb57053fcb86cf");

/// Order of the subgroup of the curve.
pub const TEST_CURVE_2_MAIN_SUBGROUP_ORDER: U384 = U384::from("40a065fb5a76390de709fb229");

// FPBLS12381
#[derive(Clone, Debug)]
pub struct TestCurve2MontgomeryConfig;
impl IsMontgomeryConfiguration for TestCurve2MontgomeryConfig {
    const MODULUS: U384 = TEST_CURVE_2_PRIME_FIELD_ORDER;
    const MP: u64 = 1901108026836139985;
    const R2: U384 = U384::from("f60e53d42ca85ba186067660c4f2daa94");
}

type TestCurve2PrimeField = MontgomeryBackendPrimeField<TestCurve2MontgomeryConfig>;

/// In F59 the element -1 is not a square. We use this property
/// to construct a Quadratic Field Extension out of it by adding
/// its square root.
#[derive(Debug, Clone)]
pub struct TestCurve2QuadraticNonResidue;
impl HasQuadraticNonResidue for TestCurve2QuadraticNonResidue {
    type BaseField = TestCurve2PrimeField;

    fn residue() -> FieldElement<TestCurve2PrimeField> {
        -FieldElement::one()
    }
}

/// The description of the curve.
#[derive(Clone, Debug)]
pub struct TestCurve2;

impl IsEllipticCurve for TestCurve2 {
    type BaseField = QuadraticExtensionField<TestCurve2QuadraticNonResidue>;
    type PointRepresentation = ShortWeierstrassProjectivePoint<Self>;

    fn generator() -> Self::PointRepresentation {
        Self::PointRepresentation::new([
            FieldElement::new([
                FieldElement::new(U384::from("21acedb641ca6d0f8b60148123a999801")),
                FieldElement::new(U384::from("14d34d94f7de312859a8a0d9dbc67159d3")),
            ]),
            FieldElement::new([
                FieldElement::new(U384::from("2ac53e77afe8d841c8eb660761c4b873a")),
                FieldElement::new(U384::from("108a9e1c5514b0921cd5781a7f71130142")),
            ]),
            FieldElement::one(),
        ])
    }
}

impl IsShortWeierstrass for TestCurve2 {
    fn a() -> FieldElement<Self::BaseField> {
        FieldElement::from(0)
    }

    fn b() -> FieldElement<Self::BaseField> {
        FieldElement::from(1)
    }
}