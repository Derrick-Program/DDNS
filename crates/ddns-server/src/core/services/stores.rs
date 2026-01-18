use crate::core::traits::stores_trait::*;

pub struct DnsStoreService<S>
where
    S: Store,
{
    store: S,
}

impl<S: Store> DnsStoreService<S> {
    pub fn new(store: S) -> Self {
        Self { store }
    }
}
