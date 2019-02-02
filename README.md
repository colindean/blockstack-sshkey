# Blockstack SSH Key retriever

[Blockstack](https://blockstack.org) is a decentralized identity provider. It
allows users to attach an SSH key to their public profile in a
cryptographically-verifiable way. Using the public Blockstack API, it's possible
to retrieve that profile and extract the SSH key from it in a format that
enables a direct-to-`authorized_keys` pipeline:

    blockstack-sshkey colindean.id >> ~/.ssh/authorized_keys

## Building for release

    cargo build --release

Release automation forthcoming.

## ⚠ Production usage warning

This was built as a part of an interview process and may not be maintained. Do
not use this on a production system without evaluating the source code for
yourself.

## License

GPLv3, see LICENSE.md for details.