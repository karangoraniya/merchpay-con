#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

#[derive(Clone)]
#[contracttype]

pub struct Merchant {
    name: String,
    wallet: Address,
    points_ratio: u32, // How many points per 1 USDC
    total_points_issued: u32,
    redemption_rate: u32,
}

#[contract]
pub struct MerchPay;

#[contractimpl]
impl MerchPay {
    pub fn register_merchant(
        env: Env,
        merchant_wallet: Address,
        name: String,
        points_ratio: u32,
        redemption_rate: u32,
    ) {
        if env.storage().instance().has(&merchant_wallet) {
            panic!("merchant already registered");
        }

        // Require authorization from merchant wallet
        merchant_wallet.require_auth();

        let merchant = Merchant {
            name,
            wallet: merchant_wallet.clone(),
            points_ratio,
            total_points_issued: 0,
            redemption_rate,
        };

        env.storage().instance().set(&merchant_wallet, &merchant);
    }

    pub fn process_payment(
        env: Env,
        merchant_wallet: Address,
        customer_wallet: Address,
        token_address: Address,
        payment_amount: i128,
    ) -> u32 {
        customer_wallet.require_auth();

        let mut merchant: Merchant = env
            .storage()
            .instance()
            .get(&merchant_wallet)
            .unwrap_or_else(|| panic!("merchant not registered"));

        // Create token client
        let token_client = token::Client::new(&env, &token_address);

        // Transfer tokens from customer to merchant
        token_client.transfer(&customer_wallet, &merchant.wallet, &payment_amount);

        // Calculate points (convert payment amount to u32 for points calculation)
        let points_to_issue = (payment_amount as u32) * merchant.points_ratio;

        merchant.total_points_issued += points_to_issue;
        env.storage().instance().set(&merchant_wallet, &merchant);

        // Update customer points
        let current_points: u32 = env
            .storage()
            .persistent()
            .get(&customer_wallet)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&customer_wallet, &(current_points + points_to_issue));

        points_to_issue
    }

    // Get customer's points balance
    pub fn get_points(env: Env, customer_wallet: Address) -> u32 {
        env.storage()
            .persistent()
            .get(&customer_wallet)
            .unwrap_or(0)
    }

    // Get merchant details
    pub fn get_merchant(env: Env, merchant_wallet: Address) -> Merchant {
        env.storage()
            .instance()
            .get(&merchant_wallet)
            .unwrap_or_else(|| panic!("merchant not registered"))
    }

    // Redeem points for USDC rewards
    pub fn redeem(
        env: Env,
        merchant_wallet: Address,
        customer_wallet: Address,
        points_amount: u32,
        usdc_token: Address,
    ) {
        customer_wallet.require_auth();

        // Load merchant data
        let merchant: Merchant = env
            .storage()
            .instance()
            .get(&merchant_wallet)
            .unwrap_or_else(|| panic!("merchant not registered"));

        // Verify customer has enough points
        let current_points: u32 = env
            .storage()
            .persistent()
            .get(&customer_wallet)
            .unwrap_or_else(|| panic!("no points found"));

        if current_points < points_amount {
            panic!("insufficient points");
        }

        // Calculate USDC reward amount
        let usdc_reward = (points_amount / merchant.redemption_rate) as i128;

        if usdc_reward == 0 {
            panic!("points amount too low for redemption");
        }

        // Transfer USDC from merchant to customer
        let token_client = token::Client::new(&env, &usdc_token);
        token_client.transfer(&merchant.wallet, &customer_wallet, &usdc_reward);

        // Deduct points from customer
        env.storage()
            .persistent()
            .set(&customer_wallet, &(current_points - points_amount));
    }

    // Update points ratio and redemption rate
    pub fn update_rates(
        env: Env,
        merchant_wallet: Address,
        new_points_ratio: u32,
        new_redemption_rate: u32,
    ) {
        if new_points_ratio == 0 || new_redemption_rate == 0 {
            panic!("rates cannot be zero");
        }

        merchant_wallet.require_auth();

        let mut merchant: Merchant = env
            .storage()
            .instance()
            .get(&merchant_wallet)
            .unwrap_or_else(|| panic!("merchant not registered"));

        merchant.points_ratio = new_points_ratio;
        merchant.redemption_rate = new_redemption_rate;
        env.storage().instance().set(&merchant_wallet, &merchant);
    }
}

mod test;
