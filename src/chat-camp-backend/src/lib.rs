use core::panic;
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    vec,
};

use candid::{CandidType, Deserialize, Principal};
use ic_cdk::caller;

thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default()
}

#[ic_cdk::query]
fn get_notes(user: Principal) -> Option<Vec<String>> {
    NOTES.with_borrow(|notes| notes.get(&user).cloned())
}

#[ic_cdk::update]
fn add_note(note: String) {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("Anon")
    }

    NOTES.with_borrow_mut(|notes_hash| {
        let notes = notes_hash.get_mut(&user);
        match notes {
            Some(notes_vec) => {
                notes_vec.push(note);
            }
            None => {
                notes_hash.insert(user, vec![note]);
            }
        }
    })
}
