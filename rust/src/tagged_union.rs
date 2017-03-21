use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;

// This is a description of all of the messages that may be sent to a counter.
// Sending a value that is not produced by one of these enum constructors is a
// compile-time error.
#[derive(Clone, Copy, Debug)]
pub enum CounterInstruction {
    Increment(isize),  // `isize` is an integer type that matches the platform word size
    Reset,
    Terminate,
}

pub type CounterResponse = isize;

use self::CounterInstruction::*;

pub fn new_counter() -> (Sender<CounterInstruction>, Receiver<CounterResponse>) {
    let (instr_tx, instr_rx) = channel::<CounterInstruction>();
    let (resp_tx,  resp_rx)  = channel::<CounterResponse>();

    // Run the counter in a background thread
    thread::spawn(move || {
        let mut count: isize = 0;

        // If the channel has closed then `recv()` will produce an `Err` value
        // instead of `Ok`, and the loop will terminate.
        while let Ok(instr) = instr_rx.recv() {

            // We match on messages to pull out the possible distinct values and
            // types from channel messages. Because each `enum` type is sealed,
            // it is a compile-time error to omit any valid patterns.
            // This avoids potential run-time failures if we add message types
            // later, but forget to update the match here.
            match instr {
                Increment(amount) => count += amount,
                Reset             => count = 0,
                Terminate         => return
            }

            // `send` returns a `Result` because sending might fail in some
            // circumstances. But for this example we assign the result to `_`
            // to ignore it.
            let _ = resp_tx.send(count);
        };

    });

    // Return the instruction sender, and the response receiver
    (instr_tx, resp_rx)
}

#[test]
fn runs_a_counter() {
    let (tx, rx) = new_counter();
    let _ = tx.send(Increment(1));
    let _ = tx.send(Increment(1));
    let _ = tx.send(Terminate);

    let mut latest_count = 0;
    while let Ok(resp) = rx.recv() {
        latest_count = resp
    }

    assert_eq!(2, latest_count);
}
