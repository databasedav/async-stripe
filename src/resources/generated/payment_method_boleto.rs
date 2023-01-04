#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodBoleto {
    /// Uniquely identifies the customer tax id (CNPJ or CPF).
    pub tax_id: String,
}
