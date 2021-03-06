# Blockstack SSH Key retriever

[Blockstack](https://blockstack.org) is a decentralized identity provider. It
allows users to attach an SSH key to their public profile in a
cryptographically-verifiable way. Using the public Blockstack API, it's possible
to retrieve that profile and extract the SSH key from it in a format that
enables a direct-to-`authorized_keys` pipeline:

    blockstack-sshkey colindean.id >> ~/.ssh/authorized_keys

## Configuration

By default, the endpoint used is `https://core.blockstack.org`. To use a
different endpoint, set the `ENDPOINT` environment variable. See
`src/retrieve.rs` for URLs used.

## Installation

[Homebrew](https://brew.sh) is the supported method of installing binaries.

Tap and install with:

    brew tap colindean/blockstack-sshkey https://github.com/colindean/blockstack-sshkey.git
    brew install blockstack-sshkey-bin

Alternatively, download the latest release [here](https://github.com/colindean/blockstack-sshkey/releases/latest).

For Windows, you can use the MSVC version if you already have the 2015 Visual C++ Runtime already installed. Otherwise, try the GNU version.

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
