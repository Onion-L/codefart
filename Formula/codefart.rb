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
      sha256 "b5e0ea0332e3224536f5c6ecc210a2bac61f9714651a42da32a8767081a3ef80"
    else
      url "https://github.com/Onion-L/codefart/releases/download/v0.1.0/codefart-x86_64-apple-darwin.tar.gz"
      sha256 "49b9c2f8a4a24009baa68f0b7629e4985eb0fa21f836dfb9c63c7da62ddde400"
    end
  end

  def install
    bin.install "codefart"
  end

  test do
    system "#{bin}/codefart", "--version"
  end
end
