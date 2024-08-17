use std::cell::RefCell;

use getrandom::Error;
use oorandom::Rand64;

thread_local! {
    static RAND: RefCell<Rand64> = RefCell::new(Rand64::new(ic_cdk::api::time() as u128));
}

fn get_random(dest: &mut [u8]) -> Result<(), Error> {
    let rand = RAND.with(|rand| {
        let mut rand = rand.borrow_mut();
        Rand64::new((rand.rand_u64() + ic_cdk::api::time()) as u128)
    });
    RAND.set(rand);

    RAND.with(|rand| {
        let mut rand = rand.borrow_mut();
        for i in 1..=dest.len() {
            dest[i - 1] = rand.rand_u64() as u8
        }
    });
    Ok(())
}

getrandom::register_custom_getrandom!(get_random);
