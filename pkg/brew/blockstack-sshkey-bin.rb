class BlockstackSshkeyBin < Formula
    version '0.1.4'
    desc "Retrieve the SSH key for Blockstack users given their Blockstack ID"
    homepage "https://github.com/colindean/blockstack-sshkey"

    if OS.mac?
      url "https://github.com/colindean/blockstack-sshkey/releases/download/#{version}/blockstack-sshkey-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "25e2d08c1aa17c4f6a275586430a8d0c8257e34448346cd6467237878b25ec38"
    elsif OS.linux?
      url "https://github.com/colindean/blockstack-sshkey/releases/download/#{version}/blockstack-sshkey-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "4364271fa3be7af1af1c80a1f4b303171970e0543874c5bb5a6936925514a466"
    end

    def install
      bin.install "blockstack-sshkey"
    end
end