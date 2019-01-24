# 0.2.2

- Applies a workaround to handle intermittent unavailability of `/dev/disk/by-` path operations.

# 0.2.1

- Mark `PartitionIdentifier` fields as public.
- Switch from `failure` error crate to `err-derive`
- Set `rust-toolchain` to `1.24.1`
- Derive `PartialEq` for `Error`
