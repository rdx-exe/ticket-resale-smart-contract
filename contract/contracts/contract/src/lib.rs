#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Address, Map, Symbol
};

#[contract]
pub struct TicketResaleContract;

#[contracttype] // ✅ REQUIRED
#[derive(Clone)]
pub struct Ticket {
    pub id: u32,
    pub owner: Address,
    pub price: i128,
    pub for_sale: bool,
}

#[contractimpl]
impl TicketResaleContract {

    // Create ticket
    pub fn create_ticket(env: Env, id: u32, owner: Address, price: i128) {
        owner.require_auth(); // ✅ security

        let key = symbol_short!("TICKETS");

        let mut tickets: Map<u32, Ticket> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let ticket = Ticket {
            id,
            owner: owner.clone(),
            price,
            for_sale: false,
        };

        tickets.set(id, ticket);
        env.storage().instance().set(&key, &tickets);
    }

    // List ticket for resale
    pub fn list_ticket(env: Env, id: u32, price: i128, owner: Address) {
        owner.require_auth(); // ✅ only owner can list

        let key = symbol_short!("TICKETS");

        let mut tickets: Map<u32, Ticket> =
            env.storage().instance().get(&key).unwrap();

        let mut ticket = tickets.get(id).unwrap();

        if ticket.owner != owner {
            panic!("Not ticket owner");
        }

        ticket.price = price;
        ticket.for_sale = true;

        tickets.set(id, ticket);
        env.storage().instance().set(&key, &tickets);
    }

    // Buy ticket
    pub fn buy_ticket(env: Env, id: u32, buyer: Address) {
        buyer.require_auth(); // ✅ auth

        let key = symbol_short!("TICKETS");

        let mut tickets: Map<u32, Ticket> =
            env.storage().instance().get(&key).unwrap();

        let mut ticket = tickets.get(id).unwrap();

        if !ticket.for_sale {
            panic!("Not for sale");
        }

        ticket.owner = buyer;
        ticket.for_sale = false;

        tickets.set(id, ticket);
        env.storage().instance().set(&key, &tickets);
    }

    // Get ticket
    pub fn get_ticket(env: Env, id: u32) -> Ticket {
        let key = symbol_short!("TICKETS");

        let tickets: Map<u32, Ticket> =
            env.storage().instance().get(&key).unwrap();

        tickets.get(id).unwrap()
    }
}