# FairBid - a Fullstack Auction dApp on Internet Computer - Hackathon Project

This is a Fullstack Auction Dapp with Rsut backend and React frontend built on the Internet Computer that allows users to create, bid on, and manage  auctions. Internet Identity is used for authorization.

Conducting auctions on the decentralized platform ICP creates an immutable record of participants and the winning bidder. This provides verifiable documentation for high-value item sales or digital assets like NFTs, ensuring transparency and accountability throughout the transaction process.

## How it works

The canister uses a StableBTreeMap to store auctions. Each auction is identified by a unique ID. The auction data includes the item being auctioned, bid history, and the remaining time.

When a user creates an auction, the canister creates a new entry in the StableBTreeMap. The canister also uses the `ic_cdk_timers::set_timer` function to schedule the `end_auction` function to be called after the specified duration when a new auction is created.

The `convert::convert_to_usd` function is used to convert a specified amount from a specified currency to USD. This function makes an HTTP call to an external service to perform the conversion. Platform is using USD as base currency for bidding. This function helps users to easily convert their local currency to USD before bidding. Please check `convert.rs` module.

When a user bids on an item, the canister checks to see if the bid is higher than the current highest bid and if auction is ended or not.

## Data Structures

The dApp uses several data structures:

- `Item`: Represents an item being auctioned. It has a title, description, and an image represented as a byte vector.
- `Bid`: Represents a bid made on an item. It includes the bid price, the time the bid was made, and the originator of the bid.
- `AuctionId`: A unique identifier for each auction.
- `AuctionOverview`: Provides a summary of an auction, including its ID and the item being auctioned.
- `AuctionDetails`: Provides detailed information about an auction, including the item, bid history, and remaining time.
- `Auction`: Represents an auction. It includes the auction ID, the item being auctioned, the bid history, and the remaining time.

## Functions

This Auction dApp provides several functions:

- `new_auction(item: Item, duration: u64)`: Creates a new auction with the specified item and duration. Duration is in seconds.
- `get_auction(id: AuctionId) -> Option<Auction>`: Retrieves the auction with the specified ID.
- `make_bid(id: AuctionId, price: u64) -> Result<(), &'static str>`: Makes a bid on the auction with the specified ID. The bid must be higher than the current highest bid.
- `end_auction(id: AuctionId) -> Result<(), &'static str>`: Ends the auction with the specified ID. Only the canister itself can call this function.
- `get_auction_details(id: AuctionId) -> Option<AuctionDetails>`: Retrieves detailed information about the auction with the specified ID.
- `get_auctions_overview() -> Vec<AuctionOverview>`: Retrieves a list of  auctions overview.
- `get_active_auctions() -> Vec<AuctionOverview>`: Retrieves a list of all active auctions.
- `get_ended_auctions() -> Vec<AuctionOverview>`: Retrieves a list of all ended auctions.
- `get_all_auctions() -> Vec<AuctionOverview>`: Retrieves a list of all auctions.
- `get_all_bids(id: AuctionId) -> Vec<Bid>`: Retrieves all bids for the auction with the specified ID.
- `get_highest_bid_details(id: AuctionId) -> Option<Bid>`: Retrieves the highest bid for the auction with the specified ID.
- `get_remaining_time(id: AuctionId) -> Option<u64>`: Retrieves the remaining time (in nanoseconds) for the auction with the specified ID.


## Running the project locally

If you want to test project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`. 
You can use generated CandidUI to test functions.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.

If you are making frontend changes, you can start a development server with

```bash
npm start
```

Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.

### Examples
#### UI:
![new_auction](https://github.com/mervanerenci/fullstack_auction_dapp_icp/assets/101268022/76ae1b71-a32a-4884-a1f3-378f792fa2ad)

![auction_list](https://github.com/mervanerenci/fullstack_auction_dapp_icp/assets/101268022/e5c8dc80-e8ef-4ecb-8bda-74a36e6ed086)

![image](https://github.com/mervanerenci/fullstack_auction_dapp_icp/assets/101268022/28c84050-95b6-4d03-be9d-5dff1a8ecfb7)



#### Auction functionality from CandidUI:

![auction_rust_tests_image](https://github.com/mervanerenci/auction_dapp_icp/assets/101268022/b79a0da3-158f-4252-a131-3ab37e890e2c)

-----



To learn more, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/current/developer-docs/setup/deploy-locally)
- [SDK Developer Tools](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Rust Canister Development Guide](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/current/developer-docs/backend/candid/)


