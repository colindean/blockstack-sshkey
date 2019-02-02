# Blockstack SSH Key retriever

[Blockstack](https://blockstack.org) is a decentralized identity provider. It
allows users to attach an SSH key to their public profile in a
cryptographically-verifiable way. Using the public Blockstack API, it's possible
to retrieve that profile and extract the SSH key from it in a format that
enables a direct-to-`authorized_keys` pipeline:

    blockstack-sshkey colindean.id >> ~/.ssh/authorized_keys
    
## Installation

[Homebrew](https://brew.sh) is the supported method of installing binaries.

Tap and install with:

    brew tap colindean/blockstack-sshkey https://github.com/colindean/blockstack-sshkey.git
    brew install blockstack-sshkey-bin

## Releasing

Releases are built when tags are pushed to CI.

    git tag -a ${VERSION:0.0.0}
    git push && git push --tags

## ⚠ Production usage warning

This was built as a part of an interview process and may not be maintained. Do
not use this on a production system without evaluating the source code for
yourself.

## License

GPLv3, see LICENSE.md for details.
