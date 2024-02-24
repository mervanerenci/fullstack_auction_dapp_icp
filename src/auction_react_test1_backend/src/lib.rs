// use ic_cdk::export::candid::Principal;
use ic_cdk::*;
use ic_cdk_timers::set_timer;
use std::time::Duration;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

// mod convert;

// DATA STRUCTURES //

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

#[derive(CandidType, Deserialize, Clone)]
pub struct Item {
    title: String,
    description: String,
    image: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Bid {
    price: u64,
    time: u64,
    originator: Principal,
}

pub type AuctionId = u64;

#[derive(CandidType)]
pub struct AuctionOverview {
    id: AuctionId,
    item: Item,
}

#[derive(CandidType)]
pub struct AuctionDetails {
    item: Item,
    bid_history: Vec<Bid>,
    end_time: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Auction {
    id: AuctionId,
    item: Item,
    bid_history: Vec<Bid>,
    end_time: u64,
    remaining_time: u64,
}

impl Storable for Auction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Auction {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static AUCTION_MAP: RefCell<StableBTreeMap<AuctionId, Auction, Memory>> = RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0)))));
}

// FUNCTIONS //

// Duration is in seconds
#[ic_cdk::update]
fn new_auction(item: Item, duration: u64) {
    AUCTION_MAP.with(|am| {
        let mut auction_map = am.borrow_mut();
        let id = auction_map.len() as AuctionId;

        let auction = Auction {
            id: id,
            item: item,
            bid_history: Vec::new(),
            end_time: api::time() + duration* 1_000_000_000,
            remaining_time: duration,
        };

        // Clone the auction before inserting it into the map
        let auction_clone = auction.clone();
        auction_map.insert(id, auction_clone);

        // Schedule the end_auction function to be called after duration seconds
        set_timer(Duration::from_secs(duration), move || {
            end_auction(auction.id).unwrap();
        });
    });
}

#[ic_cdk::query]
fn get_auction(id: AuctionId) -> Option<Auction> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map.get(&id).clone()
    })
}

#[ic_cdk::update]
fn end_auction(id: AuctionId) -> Result<(), &'static str> {
    if api::caller() != api::id() {
        return Err("Only the canister itself can call this function");
    }

    AUCTION_MAP.with(|am| {
        let mut auction_map = am.borrow_mut();
        if let Some(mut auction) = auction_map.get(&id).clone() {
            auction.remaining_time = 0;
            auction_map.insert(id, auction);
        }
    });

    Ok(())
}

#[ic_cdk::update]
fn make_bid(id: AuctionId, price: u64) -> Result<(), &'static str> {
    AUCTION_MAP.with(|am| {
        let mut auction_map = am.borrow_mut();
        if let Some(mut auction) = auction_map.get(&id).clone() {
            if auction.remaining_time == 0 {
                return Err("Auction has ended");
            }

            if let Some(highest_bid) = auction.bid_history.last() {
                if price <= highest_bid.price {
                    return Err("Bid must be higher than the current highest bid");
                }
            }

            let bid = Bid {
                price: price,
                time: api::time(),
                originator: api::caller(),
            };

            auction.bid_history.push(bid);
            auction_map.insert(id, auction);
            Ok(())
        } else {
            Err("Auction not found")
        }
    })
}

// 
#[ic_cdk::query]
fn get_auction_details(id: AuctionId) -> Option<AuctionDetails> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map.get(&id).map(|auction| AuctionDetails {
            item: auction.item.clone(),
            bid_history: auction.bid_history.clone(),
            end_time: auction.end_time,
        })
    })
}


// Returns in Nanoseconds
#[ic_cdk::query]
fn get_remaining_time(id: AuctionId) -> Option<u64> {

    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map.get(&id).map(|auction|

            if auction.remaining_time == 0 {
                0
            } else {
                auction.end_time - api::time()
            }
            
            )

    })
}

#[ic_cdk::query]
fn get_overview_list() -> Vec<AuctionOverview> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map
            .iter()
            .map(|(id, auction)| AuctionOverview {
                id: id, // remove the dereference operator here
                item: auction.item.clone(),
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_active_auctions() -> Vec<AuctionOverview> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map
            .iter()
            .filter(|(_, auction)| auction.remaining_time > 0)
            .map(|(id, auction)| AuctionOverview {
                id: id, // remove the dereference operator here
                item: auction.item.clone(),
            })
            .collect()
    })
}

#[ic_cdk::query]
fn get_ended_auctions() -> Vec<AuctionOverview> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map
            .iter()
            .filter(|(_, auction)| auction.remaining_time == 0)
            .map(|(id, auction)| AuctionOverview {
                id: id, // remove the dereference operator here
                item: auction.item.clone(),
            })
            .collect()
    })
}


#[ic_cdk::query]
fn get_all_auctions() -> Vec<AuctionOverview> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map
            .iter()
            .map(|(id, auction)| AuctionOverview {
                id: id, // remove the dereference operator here
                item: auction.item.clone(),
            })
            .collect()
    })
}



#[ic_cdk::query]
fn get_highest_bid_details(id: AuctionId) -> Option<Bid> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map
            .get(&id)
            .and_then(|auction| auction.bid_history.last().map(|bid| bid.clone()))
    })
}


// #[ic_cdk::query]
// fn get_highest_bidder(id: AuctionId) -> Option<Principal> {
//     AUCTION_MAP.with(|am| {
//         let auction_map = am.borrow();
        
//         auction_map
//             .get(&id)
//             .and_then(|auction| auction.bid_history.last().map(|bid| bid.originator))
//     })
// }


// #[ic_cdk::query]
// fn get_highest_bid(id: AuctionId) -> Option<u64> {
//     AUCTION_MAP.with(|am| {
//         let auction_map = am.borrow();
//         auction_map
//             .get(&id)
//             .and_then(|auction| auction.bid_history.last().map(|bid| bid.price))
//     })
// }


#[ic_cdk::query]
fn get_all_bids(id: AuctionId) -> Option<Vec<Bid>> {
    AUCTION_MAP.with(|am| {
        let auction_map = am.borrow();
        auction_map.get(&id).map(|auction| auction.bid_history.clone())
    })
}


// http call to convert local currency to usd, using convert.rs http call
// #[ic_cdk::update]
// async fn get_conversion_to_usd(from: String, amount: f64) -> String {
//     let conversion = convert::convert_to_usd(from, amount).await;
//     conversion
// }

