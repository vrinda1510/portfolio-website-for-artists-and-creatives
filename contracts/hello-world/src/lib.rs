#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

// Constants
const ARTWORK_COUNT: Symbol = symbol_short!("AW_COUNT");

#[contracttype]
#[derive(Clone)]
pub struct Artwork {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum Artbook {
    Entry(u64),
}

#[contract]
pub struct ArtistPortfolioContract;

#[contractimpl]
impl ArtistPortfolioContract {
    // Add new artwork
    pub fn add_artwork(env: Env, title: String, description: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&ARTWORK_COUNT).unwrap_or(0);
        count += 1;

        let timestamp = env.ledger().timestamp();

        let new_artwork = Artwork {
            id: count,
            title,
            description,
            timestamp,
        };

        env.storage().instance().set(&Artbook::Entry(count), &new_artwork);
        env.storage().instance().set(&ARTWORK_COUNT, &count);
        count
    }

    // View specific artwork
    pub fn get_artwork(env: Env, id: u64) -> Artwork {
        env.storage()
            .instance()
            .get(&Artbook::Entry(id))
            .unwrap_or(Artwork {
                id: 0,
                title: String::from_str(&env, "Not Found"),
                description: String::from_str(&env, "Not Found"),
                timestamp: 0,
            })
    }

    // Get total number of artworks
    pub fn get_artwork_count(env: Env) -> u64 {
        env.storage().instance().get(&ARTWORK_COUNT).unwrap_or(0)
    }
}