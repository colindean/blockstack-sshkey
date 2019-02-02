class BlockstackSshkeyBin < Formula
    version '0.1.2'
    desc "Retrieve the SSH key for Blockstack users given their Blockstack ID"
    homepage "https://github.com/colindean/blockstack-sshkey"

    if OS.mac?
      url "https://github.com/colindean/blockstack-sshkey/releases/download/#{version}/blockstack-sshkey-#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "5f1326468516e56916ecc4da66bd022a4b1192f161292786386b731e9d9204ff"
    elsif OS.linux?
      url "https://github.com/colindean/blockstack-sshkey/releases/download/#{version}/blockstack-sshkey-#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "b751a8270cda5f4ae1b42b830f293d2ca0338d4e7a9cfaba372c91c3f03f1a85"
    end

    def install
      bin.install "blockstack-sshkey"
    end
end