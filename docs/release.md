# Release

This note documents the local desktop release flow and upload steps for GitHub Releases.

## Version

Keep these versions in sync before publishing:

- `Cargo.toml` -> `[workspace.package].version`
- `desktop/src-tauri/tauri.conf.json` -> `version`

Example for `0.2.20`:

```bash
rg 'version = "|\"version\":' Cargo.toml desktop/src-tauri/tauri.conf.json
```

Commit and push the version bump before building release artifacts.

## Apple Credentials

Local desktop signing needs a Developer ID Application certificate installed in Keychain.

Current signing identity:

```text
Developer ID Application: Xiang Li (AQMMZQ3MDX)
```

Local notarization also needs the App Store Connect API key file:

```bash
export APPLE_API_KEY_ID="T6ACT69T9A"
export APPLE_API_ISSUER="728c5653-1d7c-445e-9ffb-698b6f71b6f3"
export APPLE_API_KEY_PATH="$HOME/AuthKey_${APPLE_API_KEY_ID}.p8"
```

Do not commit `.p8`, `.p12`, or base64-encoded certificate values.

## Local Desktop Build

Install prerequisites once:

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cd desktop
npm ci
cd ..
```

Build Apple Silicon:

```bash
export VERSION="0.2.20"
export SIGNING_IDENTITY="Developer ID Application: Xiang Li (AQMMZQ3MDX)"

cd desktop
npx tauri build --ci --target aarch64-apple-darwin \
  --config "{\"bundle\":{\"macOS\":{\"signingIdentity\":\"$SIGNING_IDENTITY\"}}}"
cd ..

export DMG="target/aarch64-apple-darwin/release/bundle/dmg/CodeFart_${VERSION}_aarch64.dmg"
```

Build Intel:

```bash
export VERSION="0.2.20"
export SIGNING_IDENTITY="Developer ID Application: Xiang Li (AQMMZQ3MDX)"

cd desktop
npx tauri build --ci --target x86_64-apple-darwin \
  --config "{\"bundle\":{\"macOS\":{\"signingIdentity\":\"$SIGNING_IDENTITY\"}}}"
cd ..

export DMG="target/x86_64-apple-darwin/release/bundle/dmg/CodeFart_${VERSION}_x64.dmg"
```

Do not build `universal-apple-darwin` unless there is a specific reason. Separate Apple Silicon and Intel builds are faster and easier to debug.

## Notarize And Staple

Submit the DMG to Apple:

```bash
xcrun notarytool submit "$DMG" \
  --key "$APPLE_API_KEY_PATH" \
  --key-id "$APPLE_API_KEY_ID" \
  --issuer "$APPLE_API_ISSUER" \
  --wait \
  --timeout 30m
```

Staple and validate the ticket:

```bash
xcrun stapler staple -v "$DMG"
xcrun stapler validate -v "$DMG"
spctl -a -vv -t install "$DMG"
```

Expected `spctl -t install` result:

```text
accepted
source=Notarized Developer ID
```

Optional mounted app check:

```bash
hdiutil attach -nobrowse -readonly "$DMG"
codesign --verify --deep --strict --verbose=2 "/Volumes/CodeFart/CodeFart.app"
spctl -a -vv "/Volumes/CodeFart/CodeFart.app"
hdiutil detach "/Volumes/CodeFart"
```

If the mounted volume is named `CodeFart 1`, use `/Volumes/CodeFart 1/CodeFart.app` and detach `/Volumes/CodeFart 1`.

## Upload To GitHub Release

The GitHub Release must exist first. The CLI release workflow creates it when pushing a `v*` tag.

Create and push a tag:

```bash
export VERSION="0.2.20"
export TAG="v${VERSION}"

git tag "$TAG"
git push origin "$TAG"
```

If the tag/release already exists, upload the local DMG directly:

```bash
export TAG="v0.2.20"
gh release upload "$TAG" "$DMG" --clobber
```

Recommended release asset names:

```bash
export APPLE_SILICON_DMG="target/aarch64-apple-darwin/release/bundle/dmg/CodeFart_${VERSION}_aarch64.dmg"
cp "$APPLE_SILICON_DMG" "CodeFart_${VERSION}-apple-silicon.dmg"
gh release upload "$TAG" "CodeFart_${VERSION}-apple-silicon.dmg" --clobber

export INTEL_DMG="target/x86_64-apple-darwin/release/bundle/dmg/CodeFart_${VERSION}_x64.dmg"
cp "$INTEL_DMG" "CodeFart_${VERSION}-intel.dmg"
gh release upload "$TAG" "CodeFart_${VERSION}-intel.dmg" --clobber
```

Check assets:

```bash
gh release view "$TAG" --json assets --jq '.assets[].name'
```

Delete stale DMGs from the same release if needed:

```bash
gh release delete-asset "$TAG" "CodeFart_0.2.14_universal.dmg" -y
```

## GitHub Actions Desktop Build

The manual desktop workflow is `.github/workflows/desktop-release.yml`.

Run it from CLI:

```bash
gh workflow run desktop-release.yml -f tag="v0.2.20"
```

Required GitHub Secrets:

- `APPLE_CERTIFICATE`: base64-encoded `.p12`
- `APPLE_CERTIFICATE_PASSWORD`: `.p12` password
- `APPLE_API_ISSUER`: App Store Connect issuer ID
- `APPLE_API_KEY`: App Store Connect key ID
- `APPLE_API_KEY_P8_BASE64`: base64-encoded `.p8`

If GitHub Actions notarization stalls or times out, use the local build, notarize, staple, and `gh release upload` flow above.
