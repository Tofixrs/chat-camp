use std::{cell::RefCell, collections::HashMap};

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::user::USERS;

thread_local! {
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<Message>>> = RefCell::default();
}

#[derive(Debug, Clone, Deserialize, Serialize, CandidType)]
struct Message {
    timestamp: u64,
    author: Principal,
    content: String,
    id: String,
}

#[ic_cdk::query]
fn get_chat(user_chat: Principal) -> Option<Vec<Message>> {
    let user = ic_cdk::caller();
    if user == Principal::anonymous() {
        panic!("Anon")
    }

    let mut key = [user, user_chat];
    key.sort();
    CHAT.with_borrow(|chats| {
        let chat = chats.get(&key).cloned();
        return chat;
    })
}

#[ic_cdk::update]
fn add_chat_msg(content: String, send_to: Principal) {
    let user = ic_cdk::caller();
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
        let msg = Message {
            timestamp: ic_cdk::api::time(),
            author: user,
            content,
            id: Uuid::new_v4().hyphenated().to_string(),
        };
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
