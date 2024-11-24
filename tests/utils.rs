#[cfg(test)]
mod tests {
    use crate::utils::{to_hex, to_hex_trimmed};
    use starknet::core::types::FieldElement;

    #[test]
    fn test_to_hex() {
        let zero = FieldElement::ZERO;
        assert_eq!(
            to_hex(zero),
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        );

        let small = FieldElement::from(42);
        assert_eq!(
            to_hex(small),
            "0x000000000000000000000000000000000000000000000000000000000000002a"
        );

        let large = FieldElement::from_hex_be("0x123456789abcdef0").unwrap();
        assert_eq!(
            to_hex(large),
            "0x0000000000000000000000000000000000000000000000000123456789abcdef0"
        );
    }

    #[test]
    fn test_to_hex_trimmed() {
        let zero = FieldElement::ZERO;
        assert_eq!(to_hex_trimmed(zero), "0x0");

        let small = FieldElement::from(42);
        assert_eq!(to_hex_trimmed(small), "0x2a");

        let large = FieldElement::from_hex_be("0x123456789abcdef0").unwrap();
        assert_eq!(to_hex_trimmed(large), "0x123456789abcdef0");
    }
}
