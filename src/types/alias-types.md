<unstable-text>
r[type.alias]

- associated types
- opaque types
    - link from "impl-trait type" to this or param

r[type.alias.rigid]

Aliases might be treated as *rigid* in their current environment. In this case they behave like other types.
Their equality is structural, *rigid* aliases are only equal if both have the same type constructor and equal corresponding arguments.

r[type.alias.normalization]

Alias types can be normalized to their underlying type.
- for associated types this is the type provided by the corresponding impl
- opaque types

r[type.alias.normalization.assoc-type]

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

</unstable-text>
