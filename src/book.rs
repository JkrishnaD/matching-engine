use crate::states::{Fill, Order, Side, Snapshot, State};
use std::collections::{BTreeMap, VecDeque};

pub struct OrderBook {
    bids: BTreeMap<u64, VecDeque<Order>>,
    asks: BTreeMap<u64, VecDeque<Order>>,
}

impl OrderBook {
    // Initializing new OrderBook with empty maps
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    // Returns a snapshot of the current order book state
    pub fn snapshot(&self) -> Snapshot {
        // bids with highest price first
        let bids = self
            .bids
            .iter()
            .rev()
            .map(|(p, q)| State {
                price: *p,
                qty: q.iter().map(|o| o.qty).sum(),
            })
            .collect();

        // asks with lowest price first
        let asks = self
            .asks
            .iter()
            .map(|(p, q)| State {
                price: *p,
                qty: q.iter().map(|o| o.qty).sum(),
            })
            .collect();

        Snapshot { bids, asks }
    }

    pub fn match_orders(&mut self, taker: Order) -> Vec<Fill> {
        let fills = Vec::new();

        // matching order loop
        // loop {}

        // If taker has qty left, rest it on its own side
        if taker.qty > 0 {
            let side = match taker.side {
                Side::Buy => &mut self.bids,
                Side::Sell => &mut self.asks,
            };
            side.entry(taker.price).or_default().push_back(taker);
        }
        fills
    }
}

#[test]
pub fn resting_only_no_match() {
    let mut book = OrderBook::new();
    // bid order with no matching ask
    let fills = book.match_orders(Order {
        id: 1,
        side: Side::Buy,
        price: 100,
        qty: 5,
    });
    // assert no fills were generated as there is no matching ask
    assert!(fills.is_empty());
    // assert the bid order is resting on the book
    let snap = book.snapshot();
    assert_eq!(snap.bids.len(), 1);
    assert_eq!(snap.bids[0].price, 100);
    assert_eq!(snap.bids[0].qty, 5);
}
