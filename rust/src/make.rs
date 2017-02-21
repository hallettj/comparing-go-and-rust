use std::sync::mpsc::{Receiver, Sender, channel};

// A `trait` declares some common behavior
trait Make {
    fn make() -> Self;
    fn make_with_capacity(capacity: usize) -> Self;
}

// An `impl` provides implementations for trait methods for given concrete types.
// Note that we can implement `Make` for the standard `Vec<T>` type right here.
// It is not necessary to change the implementation of `Vec<T>`.
impl <T> Make for Vec<T> {
    fn make() -> Vec<T> {
        Vec::new()
    }

    fn make_with_capacity(capacity: usize) -> Vec<T> {
        Vec::with_capacity(capacity)
    }
}

// `Sender` and `Receiver` are Rust's standard channel types.
// This trait implementation constructs a connected sender/receiver pair.
impl <T> Make for (Sender<T>, Receiver<T>) {
    // Rust channels do not have a fixed buffer size - a channel is either non-blocking (with an
    // unlimited buffer), or is blocking. But a non-blocking channel has a different type;
    // so that will require a different trait impl.
    fn make() -> (Sender<T>, Receiver<T>) {
        channel()
    }

    fn make_with_capacity(_: usize) -> (Sender<T>, Receiver<T>) {
        Make::make()
    }
}

#[test]
fn makes_a_vec() {
    // We specify a type for the variable that holds the output of `make`.
    // This instructs the compiler which implementation of `make` to call.
    let mut v: Vec<&str> = Make::make();
    v.push("some string");
    assert_eq!("some string", v[0]);
}

#[test]
fn makes_a_sized_vec() {
    let v: Vec<isize> = Make::make_with_capacity(4);
    assert_eq!(4, v.capacity());
}

#[test]
fn makes_a_channel() {
    // Or we can just let the compiler infer the type that we expect.
    let (sender, receiver) = Make::make();
    let msg    = "hello";
    let _      = sender.send(msg);
    let result = receiver.recv().expect("a successfully received value");
    assert_eq!(msg, result);
}
