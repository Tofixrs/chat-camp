mod user;
use core::panic;
use std::{cell::RefCell, collections::HashMap, vec};

use candid::Principal;
use ic_cdk::caller;
use user::USERS;

thread_local! {
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn get_chat(user_chat: Principal) -> Option<Vec<String>> {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("Anon")
    }

    let mut key = [user, user_chat];
    key.sort();
    CHAT.with_borrow(|chats| chats.get(&key).cloned())
}

#[ic_cdk::update]
fn add_chat_msg(msg: String, send_to: Principal) {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("Anon")
    }

    let is_caller_registered = USERS.with_borrow(|users| users.contains_key(&user));
    let is_recipient_registered = USERS.with_borrow(|users| users.contains_key(&send_to));

    if !is_caller_registered {
        panic!("Register")
    }

    if !is_recipient_registered {
        panic!("User doesnt exist")
    }

    let mut key = [user, send_to];
    key.sort();

    CHAT.with_borrow_mut(|chats| {
        let chat = chats.get_mut(&key);
        match chat {
            Some(chat) => {
                chat.push(msg);
            }
            None => {
                chats.insert(key, vec![msg]);
            }
        }
    })
}
