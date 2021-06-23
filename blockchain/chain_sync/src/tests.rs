
#[cfg(test)]
mod tests {
    use state_manager::StateManager;
    use std::sync::Arc;
    use chain::ChainStore;
    use rand::Rng;
    use std::str::FromStr;
    use blocks::TipsetKeys;
    use db::rocks;
    use fil_types::verifier::{ProofVerifier, MockVerifier};
    use std::marker::PhantomData;
    use ipld_blockstore::BlockStore;

    struct My<V> {
        verifier: PhantomData<V>,
    }

    impl<V> My<V>
    where
        V: ProofVerifier + Sync + Send + 'static,
    {
        pub fn new() -> Self {
            Self{
                verifier: Default::default()
            }
        }

        async fn _test_validate(&self) {
            let db = db::rocks::RocksDb::open("/Users/didi/.forest/db").unwrap();

            let db = Arc::new(db);

            // Initialize StateManager
            let chain_store = Arc::new(ChainStore::new(Arc::clone(&db)));

            let sm = Arc::new(StateManager::new(chain_store));
            let bs = sm.blockstore();
            let cs = sm.chain_store().clone();

            let init_cid = cid::Cid::from_str("bafy2bzacebl3zdyqdxpl4pq2ntxzeklqo3y3ph7dsnyfqjf2xmmznmks35ti6").unwrap();
            let mut oldest_parent = TipsetKeys::new(vec![init_cid]);
            let mut parent_tipsets = vec![];
            let mut blocks = std::collections::HashMap::new();
            let mut base_tipset ;
            loop {
                if let Ok(tipset) = cs.tipset_from_keys(&oldest_parent).await {
                    println!("chainstore has load {:?}", tipset.epoch());
                    tipset.cids().iter().for_each(|c| {
                        let bh:blocks::BlockHeader = sm.blockstore().get(c).unwrap().expect("no cid for block");
                        blocks.insert(c.clone(), bh);
                    });
                    oldest_parent = tipset.parents().clone();
                    parent_tipsets.push(tipset.clone());
                    if tipset.epoch() == 0 {
                        base_tipset = tipset.clone();
                        break;
                    }
                }
            }

            let headers: Vec<&blocks::BlockHeader> = parent_tipsets.iter().flat_map(|t| t.blocks()).collect();

            for tipset in parent_tipsets.iter().rev().skip(1) {
                let full = cs.fill_tipset(&tipset).unwrap();
                for b in full.blocks().iter() {
                    println!("block {:?}", b.header().epoch());
                    let header = b.header();
                    let base_tipset = cs
                        .tipset_from_keys(header.parents())
                        .await
                        .unwrap();
                    let (state_root, receipt_root) = sm.tipset_state::<V>(&base_tipset)
                        .await
                        .expect("tipset_state");
                    println!("parent epoch {:?} now epoch{:?} state_root {:?} receipt_root {:?}", base_tipset.epoch(), header.epoch(), state_root, receipt_root);
                    if &state_root != header.state_root() {
                            println!("Parent state root did not match computed state: {} (header), {} (computed)",
                            header.state_root(),
                            state_root);
                            return;
                    }
                    if &receipt_root != header.message_receipts() {
                        println!(
                            "Parent receipt root did not match computed root: {} (header), {} (computed)",
                            header.message_receipts(),
                            receipt_root
                        );
                        return;
                    }
                    println!("epoch state ok {:?}", header.epoch());
                }
            }
        }
    }

    #[async_std::test]
    async fn test_validate() {
        let my = My::<MockVerifier>::new();
        my._test_validate().await
    }
}
