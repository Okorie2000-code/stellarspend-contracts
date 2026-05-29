#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Budget(BytesN<32>),
}