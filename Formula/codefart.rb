class Codefart < Formula
  desc "💨 Play a sound when your AI finishes thinking"
  homepage "https://github.com/Onion-L/codefart"
  url "https://github.com/Onion-L/codefart/releases/download/v0.1.0/codefart-aarch64-apple-darwin.tar.gz"
  version "0.1.0"
  sha256 ""
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/Onion-L/codefart/releases/download/v0.1.0/codefart-aarch64-apple-darwin.tar.gz"
      sha256 ""
    else
      url "https://github.com/Onion-L/codefart/releases/download/v0.1.0/codefart-x86_64-apple-darwin.tar.gz"
      sha256 ""
    end
  end

  def install
    bin.install "codefart"
  end

  test do
    system "#{bin}/codefart", "--version"
  end
end
