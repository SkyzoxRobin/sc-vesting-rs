#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Vesting {
	#[init]
    fn init(
        &self,
        vesting_period_in_epoch: u64,
		token: TokenIdentifier // in seconds
    ) -> SCResult<()> {
        require!(vesting_period_in_epoch > 0, "Vesting period cannot be set to zero");
        self.vesting_period().set(&vesting_period_in_epoch);
		self.token_vested().set(&token);
		Ok(())
	}

	#[payable("*")]
	#[endpoint(depositToken)]
	fn deposit_token(
		&self,
		#[payment_amount] amount: BigUint,
		token: TokenIdentifier
	) -> SCResult<()> {
		let token_vesting = self.token_vested().get();
		require!(token == token_vesting, "Token sent is wrong");

		let caller = self.blockchain().get_caller();
		let current_period = self.blockchain().get_block_epoch();

		self.user_start_vesting().insert(caller.clone(), current_period);

		self.user().insert(caller, amount);
		Ok(())
	}

	#[payable("*")]
	#[endpoint(claimTokens)]
	fn claim_tokens(&self) -> SCResult<()> {
		let caller = self.blockchain().get_caller();
		// vérifiez si le caller est dans la liste des gens qui ont deposit : did user deposit -> bool ou alors vérifiez si le tableau contient le caller
		// let user_state = self.user().get(&caller).unwrap();
		// require!(user_state == caller, "Nothing to claim");

		let token = self.token_vested().get();

		let user_amount = self.user().get(&caller).unwrap();
		let send_ratio = self.calculate_to_send();
		require!(send_ratio > 0, "Nothing to claim yet");

		let to_send = user_amount * BigUint::from(send_ratio);

		self.send().direct(&caller, &token, 0, &to_send, &[]);
		Ok(())
	}

	fn calculate_to_send(&self) -> u64 {
		let caller = self.blockchain().get_caller();
		let user_epoch = self.user_start_vesting().get(&caller).unwrap();
		let current_epoch = self.blockchain().get_block_epoch();

		let passed_epoch = current_epoch - user_epoch;

		let mut ratio = 0u64;

		if passed_epoch > (user_epoch + (180u64)) {  // 6 mois
			ratio = 1/4; 
		}
		// let ratio += 1/4
		if passed_epoch > (user_epoch + (360u64)) {  // 12 mois
			ratio = 1/4 + 1/4;
		}
		if passed_epoch > (user_epoch + (540u64)) {  // 18 mois
			ratio = 1/4 + 1/4 + 1/4;
		}
		if passed_epoch > (user_epoch + (720u64)) {  // 24 mois
			ratio = 1;
		}
		ratio
		}

	#[view(getRemainingVestingPeriod)]
	fn get_remaining_vesting_period(&self, address: ManagedAddress) -> u64 {
		let user_epoch = self.user_start_vesting().get(&address).unwrap();
		let vesting_period = self.vesting_period().get();
		let current_epoch = self.blockchain().get_block_epoch();

		let passed_epoch = current_epoch - user_epoch;

		vesting_period - passed_epoch
	}

	#[storage_mapper("user")]
	fn user(&self) -> MapMapper<ManagedAddress, BigUint>;

	#[storage_mapper("startVestingPeriod")]
	fn user_start_vesting(&self) -> MapMapper<ManagedAddress, u64>;

	#[view(getVestingPeriod)]
	#[storage_mapper("vestingPeriod")]
	fn vesting_period(&self) -> SingleValueMapper<u64>;

	#[view(getVestedToken)]
	#[storage_mapper("tokenVested")]
	fn token_vested(&self) -> SingleValueMapper<TokenIdentifier>;
}