use cosmwasm_std::{StdError, Uint128};

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

#[cfg(test)]
mod tests {
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
}
