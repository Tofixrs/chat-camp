use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Principal};
use ic_cdk::caller;
use serde::Deserialize;

thread_local! {
    pub static USERS: RefCell<HashMap<Principal, UserData>> = RefCell::default();
}

#[derive(CandidType, Debug, Deserialize, Clone)]
pub struct UserData {
    name: String,
    avatar_url: Option<String>,
}

impl UserData {
    pub fn new(name: String, avatar_url: Option<String>) -> Self {
        Self { name, avatar_url }
    }
}

#[ic_cdk::update]
fn register(nick: String) {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("Anon")
    }

    USERS.with_borrow_mut(|users| {
        let user_data = users.get(&user);
        if user_data.is_some() {
            panic!("Already registered")
        }
        users.insert(user, UserData::new(nick, None));
    })
}

#[ic_cdk::update]
fn update_user_data(nick: Option<String>, avatar_url: Option<String>) {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("Anon")
    }

    USERS.with_borrow_mut(|users| {
        let user_data = users.get_mut(&user);
        let Some(user_data) = user_data else {
            panic!("User doesnt exist");
        };
        user_data.avatar_url = avatar_url;
        if let Some(nick) = nick {
            user_data.name = nick;
        }
    })
}

#[ic_cdk::query]
fn get_user_data(principal: Principal) -> Option<UserData> {
    USERS.with_borrow(|users| users.get(&principal).cloned())
}

#[ic_cdk::query]
fn get_users() -> HashMap<Principal, UserData> {
    USERS.with_borrow(|users| users.clone())
}
