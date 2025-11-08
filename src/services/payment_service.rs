//! Placeholder payment service (Stripe integration will replace this).

/// Create a payment intent placeholder.
pub async fn create_payment_intent(_amount_baht: u32) -> Result<String, &'static str> {
    // TODO: implement Stripe integration
    Ok("pi_placeholder_123".to_string())
}
