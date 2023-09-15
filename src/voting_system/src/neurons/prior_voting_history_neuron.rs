use crate::{
  decimal_number_wrapper::DecimalNumberWrapper, external_data_provider_contract,
  types::VotingSystemError, VotingSystem,
};
use soroban_sdk::{Env, String, Address};
use soroban_sdk::FromVal;

pub fn oracle_function(
  env: Env,
  voter_id: String,
  _project_id: String,
) -> Result<(u32, u32), VotingSystemError> {
  let external_data_provider_address = VotingSystem::get_external_data_provider(env.clone())?;
  let external_data_provider_client =
    external_data_provider_contract::Client::new(&env, &Address::from_val(&env, external_data_provider_address.as_val()));

  let voter_active_rounds = external_data_provider_client.get_user_prior_voting_history(&voter_id);
  if voter_active_rounds.is_empty() {
    return Ok((0, 0));
  }
  let round_bonus_map = external_data_provider_client.get_round_bonus_map();
  let mut bonus_result = DecimalNumberWrapper::new(0, 0);
  for round in voter_active_rounds {
    let bonus: (u32, u32) = round_bonus_map
      .get(round)
      .ok_or(VotingSystemError::RoundNotFoundInRoundBonusMap)?;
    bonus_result = DecimalNumberWrapper::add(
      DecimalNumberWrapper::from(bonus_result),
      DecimalNumberWrapper::from(bonus),
    );
  }
  Ok(bonus_result.as_tuple())
}
