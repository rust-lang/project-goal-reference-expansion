r[concurrency]
# Concurrency

r[concurrency.intro]
Rust provides language and library features for writing [concurrent programs]. These features are designed to prevent [data races] --- situations in which multiple threads access the same memory without proper synchronization, with at least one of the accesses modifying that memory.

This chapter describes the traits, types, and concepts that Rust uses to express and enforce safe concurrency.

r[concurrency.send-and-sync]
## Send and Sync

r[concurrency.send-and-sync.intro]
The [`Send`] and [`Sync`] [auto traits] are [unsafe traits] used by the Rust type system to track which types can be safely used across thread boundaries.

These traits are [marker traits] with no methods. Implementing them (whether manually or via the compiler's automatic implementation) asserts that a type has the intrinsic properties required for safe concurrent use.

Library functions that require types that can be sent across threads require [`Send`].
```rust
// This will compile successfully
// A type is `Send` if it can be safely transferred to another thread.
fn assert_send<T: Send>() {}

fn main() {
    assert_send::<i32>(); // primitive types are Send
    assert_send::<Vec<u8>>(); // Vec<T> is Send if T is Send
}
```

Something that awaits something that isn't sync will produce a [`core::future::Future`] that is not [`Send`].
```rust,compile_fail
// This will not compile
// A type containing `Rc<T>` is not `Send`.
use std::rc::Rc;

// This async function awaits something that uses an Rc (which is !Send)
async fn not_send_future() {
    let rc = Rc::new(42);

    // Simulate an async operation that captures `rc`
    let _ = async {
        println!("{}", rc);
    }.await;
}

// A helper function that requires its argument to be Send
fn assert_send<T: Send>(_: T) {}

fn main() {
    let fut = not_send_future();

    assert_send(fut); //~ error[E0277] `*const i32` cannot be shared between threads safely
}
```

Library functions that require types that can be accessed concurrently from multiple threads require [`Sync`].
```rust
// This will compile successfully
// A type is `Sync` if it can be safely referenced from multiple threads.
fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<i32>(); // i32 is Sync
    assert_sync::<&'static str>(); // string slices are Sync
    assert_sync::<std::sync::Arc<i32>>(); // Arc<T> is Sync if T is Sync
}
```

```rust,compile_fail
// This will not compile
// A type containing `Cell<T>` is not `Sync`.
use std::cell::Cell;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<Cell<i32>>(); //~ error[E0277] `*const i32` cannot be shared between threads safely
}
```

r[concurrency.atomics]
## Atomics

r[concurrency.atomics.intro]
[Atomic types] allow multiple threads to safely read and write shared values without using explicit locks by providing atomic operations such as atomic loads, stores, and read-modify-write with configurable memory ordering.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    let handles: Vec<_> = (0..10)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    COUNTER.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```

The semantics of these operations are defined by Rust’s [memory model].

r[concurrency.atomics.thread-safety]
Atomic operations are guaranteed to be indivisible: no other thread can observe a value half-written or perform a conflicting update in the middle of an atomic operation. Correct use of atomic types can prevent [data races], but misuse may still cause higher-level concurrency bugs such as deadlocks or livelocks.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    static VALUE: AtomicUsize = AtomicUsize::new(0);

    let t1 = thread::spawn(|| VALUE.store(1, Ordering::Relaxed));
    let t2 = thread::spawn(|| VALUE.store(2, Ordering::Relaxed));

    t1.join().unwrap();
    t2.join().unwrap();

    // VALUE is guaranteed to be either 1 or 2, never a corrupted mix.
    println!("{}", VALUE.load(Ordering::Relaxed));
}
```

r[concurrency.atomics.mapping]
The following table lists the atomic types and the corresponding primitive types they represent:

| Primitive Type | Atomic Type                           |
| -------------- | ------------------------------------- |
| `bool`         | [`core::sync::atomic::AtomicBool`]    |
| `i8`           | [`core::sync::atomic::AtomicI8`]      |
| `i16`          | [`core::sync::atomic::AtomicI16`]     |
| `i32`          | [`core::sync::atomic::AtomicI32`]     |
| `i64`          | [`core::sync::atomic::AtomicI64`]     |
| `isize`        | [`core::sync::atomic::AtomicIsize`]   |
| `u8`           | [`core::sync::atomic::AtomicU8`]      |
| `u16`          | [`core::sync::atomic::AtomicU16`]     |
| `u32`          | [`core::sync::atomic::AtomicU32`]     |
| `u64`          | [`core::sync::atomic::AtomicU64`]     |
| `usize`        | [`core::sync::atomic::AtomicUsize`]   |
| `*mut T`       | [`core::sync::atomic::AtomicPtr<T>`]  |

r[concurrency.atomics.usage]
Atomic types are [`Sync`], meaning references to them can be safely shared between threads. Using atomic operations correctly may require careful reasoning about memory ordering.

```rust
use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64,
    AtomicIsize, AtomicU8, AtomicU16, AtomicU32, AtomicU64,
    AtomicUsize,
};

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<AtomicBool>();
    assert_sync::<AtomicI8>();
    assert_sync::<AtomicI16>();
    assert_sync::<AtomicI32>();
    assert_sync::<AtomicI64>();
    assert_sync::<AtomicIsize>();
    assert_sync::<AtomicU8>();
    assert_sync::<AtomicU16>();
    assert_sync::<AtomicU32>();
    assert_sync::<AtomicU64>();
    assert_sync::<AtomicUsize>();
}
```

[concurrent programs]: glossary.md#concurrent-program
[data races]: glossary.md#data-race
[`Send`]: special-types-and-traits.md#send
[`Sync`]: special-types-and-traits.md#sync
[auto traits]: special-types-and-traits.md#auto-traits
[unsafe traits]: items/traits.md#unsafe-traits
[marker traits]: glossary.md#marker-trait
[`core::future::Future`]: https://doc.rust-lang.org/stable/core/future/trait.Future.html
[Atomic types]: glossary.md#atomic-types
[memory model]: memory-model.md#memory-model
[`core::sync::atomic::AtomicBool`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicBool.html
[`core::sync::atomic::AtomicI8`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicI8.html
[`core::sync::atomic::AtomicI16`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicI16.html
[`core::sync::atomic::AtomicI32`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicI32.html
[`core::sync::atomic::AtomicI64`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicI64.html
[`core::sync::atomic::AtomicIsize`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicIsize.html
[`core::sync::atomic::AtomicU8`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicU8.html
[`core::sync::atomic::AtomicU16`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicU16.html
[`core::sync::atomic::AtomicU32`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicU32.html
[`core::sync::atomic::AtomicU64`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicU64.html
[`core::sync::atomic::AtomicUsize`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicUsize.html
[`core::sync::atomic::AtomicPtr<T>`]: https://doc.rust-lang.org/stable/core/sync/atomic/struct.AtomicPtr.html
