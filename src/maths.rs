use cosmwasm_std::{Decimal, StdError, StdResult, Uint128};

pub const DEFAULT_STRATEGY_TOKENS_PER_DEPOSITED_BASE_TOKEN: Uint128 = Uint128::new(1_000_000);

pub fn calculate_strategy_tokens(
    base_tokens: Uint128,
    total_staked_amount: Uint128,
    strategy_token_supply: Uint128,
) -> Result<Uint128, StdError> {
    let strategy_tokens = if total_staked_amount.is_zero() {
        base_tokens.checked_mul(DEFAULT_STRATEGY_TOKENS_PER_DEPOSITED_BASE_TOKEN)?
    } else {
        strategy_token_supply.multiply_ratio(base_tokens, total_staked_amount)
    };

    Ok(strategy_tokens)
}

pub fn calculate_fee_and_remainder(
    amount: Uint128,
    fee_rate: Decimal,
) -> StdResult<(Uint128, Uint128)> {
    let remainder = amount * (Decimal::one() - fee_rate);
    let fee = amount.checked_sub(remainder)?;
    Ok((fee, remainder))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_strategy_tokens_with_zero_staked_amount() {
        let base_tokens = Uint128::new(100);
        let total_staked_amount = Uint128::new(0);
        let strategy_token_supply = Uint128::new(200); // This value shouldn't matter in this test

        let result =
            calculate_strategy_tokens(base_tokens, total_staked_amount, strategy_token_supply)
                .unwrap();
        assert_eq!(result, Uint128::new(100_000_000)); // Since 100 * 1,000,000
    }

    #[test]
    fn test_strategy_tokens_with_non_zero_staked_amount() {
        let base_tokens = Uint128::new(100);
        let total_staked_amount = Uint128::new(1000);
        let strategy_token_supply = Uint128::new(1_000_000);

        let result =
            calculate_strategy_tokens(base_tokens, total_staked_amount, strategy_token_supply)
                .unwrap();
        assert_eq!(result, Uint128::new(100_000)); // Proportional to the ratio 100/1000 of the supply
    }

    #[test]
    fn test_strategy_tokens_with_exact_match_staked_amount() {
        let base_tokens = Uint128::new(500);
        let total_staked_amount = Uint128::new(500);
        let strategy_token_supply = Uint128::new(1_000_000);

        let result =
            calculate_strategy_tokens(base_tokens, total_staked_amount, strategy_token_supply)
                .unwrap();
        assert_eq!(result, Uint128::new(1_000_000)); // Should match the strategy token supply because ratio is 1:1
    }

    #[test]
    fn test_calculate_fee_and_remainder_with_basic_fee() {
        let fee_rate = Decimal::percent(10);
        let amount = Uint128::new(1000);
        let expected_fee = Uint128::new(100);
        let expected_remainder = Uint128::new(900);
        let (fee, remainder) = calculate_fee_and_remainder(amount, fee_rate).unwrap();
        assert_eq!(fee, expected_fee);
        assert_eq!(remainder, expected_remainder);
    }

    #[test]
    fn test_calculate_fee_and_remainder_with_zero_fee_rate() {
        let fee_rate = Decimal::zero();
        let amount = Uint128::new(1000);
        let expected_fee = Uint128::new(0);
        let expected_remainder = Uint128::new(1000);
        let (fee, remainder) = calculate_fee_and_remainder(amount, fee_rate).unwrap();
        assert_eq!(fee, expected_fee);
        assert_eq!(remainder, expected_remainder);
    }

    #[test]
    fn test_calculate_fee_and_remainder_with_100_percent_fee_rate() {
        let fee_rate = Decimal::percent(100);
        let amount = Uint128::new(1000);
        let expected_fee = Uint128::new(1000);
        let expected_remainder = Uint128::zero();
        let (fee, remainder) = calculate_fee_and_remainder(amount, fee_rate).unwrap();
        assert_eq!(fee, expected_fee);
        assert_eq!(remainder, expected_remainder);
    }

    #[test]
    fn test_calculate_fee_and_remainder_with_small_fractional_fee_rate() {
        let fee_rate = Decimal::from_str("0.05").unwrap();
        let amount = Uint128::new(1000);
        let expected_fee = Uint128::new(50);
        let expected_remainder = Uint128::new(950);
        let (fee, remainder) = calculate_fee_and_remainder(amount, fee_rate).unwrap();
        assert_eq!(fee, expected_fee);
        assert_eq!(remainder, expected_remainder);
    }
}
