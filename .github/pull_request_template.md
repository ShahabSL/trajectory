## Summary

- 

## Validation

- [ ] `cargo fmt --all --check`
- [ ] `cargo test -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --lib --bins`
- [ ] `cargo clippy -p trajectory-core -p trajectory-cli -p trajectory-vpn-bridge --all-targets -- -D warnings`
- [ ] Client or packaging checks when touched

## Notes

Do not include secrets, private resolver lists, access keys, signing material, or live endpoint credentials in the PR.
