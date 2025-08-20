r[type.alias]

Associated types and opaque types are considered *alias types*.

r[type.alias.rigid]

An alias may either get normalized. If doing so is not possible in the current environment, the alias is a *rigid* alias. 

In this case they behave like other types. Their equality is structural: rigid aliases are only equal to another type if they both have the same type constructor and equal corresponding arguments.

r[type.alias.normalization]

Alias types can be normalized to their underlying type.

r[type.alias.normalization.assoc-type]

For associated types this is the type provided by the impl used to satisfy the corresponding trait bound.
```rust
trait Trait {
    type Assoc;
}
impl Trait for Vec<T> {
    type Assoc = Box<T>;
}

// `Vec<u32>: Trait` can be satisfied using the above impl,
// causing the associated type to get normalized to `Box<u32>`.
fn normalize_me(x: <Vec<u32> as Trait>::Assoc) -> Box<u32> {
    x
}
```

r[type.alias.normalization.assoc-type.candidate-preference]

> This is purely descriptive. Candidate preference behavior may change in future releases and must not be relied upon for correctness or soundness.

Similar to how trait bounds get satisfied, associated types can be normalized via
multiple different candidates

- impl (also builtin)
- projection bound in the environment TODO: where do we talk about them
- alias bound of their self type

candidate preference:
- normalizing an alias relies on the candidate group used to prove their corresponding trait bound
- if corresponding trait bound has been proven via a where-bound or an alias-bound, we do not consider impls
    - if there is no remaining candidate, the associated type is rigid

For all applicable candidates we
- prefer where-bounds
- then alias bounds
- then impls
