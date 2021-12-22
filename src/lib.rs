#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Vesting
 {
	#[init]
	fn init(
		&self,
		token: TokenIdentifier,
		vesting_period: u64,
	) {
		self.token_vested().set(&token);
		self.vesting_period().set(&vesting_period)
	}

	#[endpoint(depositToken)]
	fn deposit_token(
		&self,
		#[payment_amount] amount: BigUint,
		token: TokenIdentifier
		) -> SCResult<()> {

		let token_vesting = self.token_vested().get();
		require!(token == token_vesting, "Token send is not good")
		let caller = self.blockchain().get_caller();
		// periode quand la personne rentre
		initial_period = self.blockchain().get_timestamp();
		self.start_vesting_period().insert(&initial_period);

		self.user().insert(&caller, &amount)
		Ok(())
	}

	#[endpoint(claimTokens)]
	fn claim_tokens(&self) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		let user_state = self.user().get(&caller);
		require!(user_state == caller, "User has nothing to claim")

		let token = self.token_vested().get();
		let vesting_period = self.calculate_vesting_time();
		let user_period = self.start_vesting_period().get(&time)
		require!(vesting_period <= user_period, "Your vesting time is not finish")

		self.send().direct(&caller, &token_id, 0, &amount, &[])
		Ok(())
	}

	fn calculate_vesting_time(&self) -> SCResult<()> {

		vesting_period = self.vesting_period().get();

		let mut vesting_time = 0;

		//if > 6 mois 
		//if > 12 et plus grand que 6 mois
		//if > 18 et plus grand que 12 mois
		//if < 24 et plus grand que 24 mois 

	}

	#[storage_mapper("user")]
	fn user(&self) -> MapMapper<ManagedAddress, BigUint>;

	#[storage_mapper("startVestingPeriod")]
	fn start_vesting_period(&self) -> MapMapper<ManagedAddress, u64>

	fn vesting_period(&self) -> SingleValueMapper<u64>;

	#[storage_mapper("tokenVested")]
	fn token_vested(&self) -> SingleValueMapper<TokenIdentifier>;
}