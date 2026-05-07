# Release

CodeFart 发版完整流程：CLI 由 GitHub Actions 自动构建，Desktop DMG 本地打包 + 公证后上传。

## 发布检查清单

- [ ] 1. 版本号同步（3 个文件）
- [ ] 2. Commit + Push
- [ ] 3. 打 Tag + Push（触发 CLI Release Workflow）
- [ ] 4. 等待 CLI Workflow 完成
- [ ] 5. 本地打包 Desktop DMG（Apple Silicon + Intel）
- [ ] 6. 公证 + Staple + 验证
- [ ] 7. 上传 DMG 到 GitHub Release
- [ ] 8. 更新 landing page 下载链接 + Push

---

## 1. 版本号同步

以下 3 个文件的版本号必须一致：

| 文件 | 字段 |
|------|------|
| `Cargo.toml` | `[workspace.package].version` |
| `desktop/src-tauri/tauri.conf.json` | `version` |
| `desktop/package.json` | `version` |

一键检查：

```bash
rg 'version.*=.*"0\.' Cargo.toml
rg '"version"' desktop/src-tauri/tauri.conf.json desktop/package.json
```

## 2. Commit + Push

```bash
git add Cargo.toml desktop/src-tauri/tauri.conf.json desktop/package.json
git commit -m "chore: bump version to X.Y.Z"
git push
```

## 3. 打 Tag（触发 CLI Release）

推送 `v*` tag 会触发 `.github/workflows/release.yml`，自动构建并发布 Linux/macOS/Windows CLI 产物。

```bash
VERSION="X.Y.Z"
TAG="v${VERSION}"
git tag "$TAG"
git push origin "$TAG"
```

## 4. 等待 CLI Workflow

在 Actions 页面确认 `Release` workflow 跑完，新 release 包含 CLI 的 5 个产物：

- `codefart-aarch64-apple-darwin.tar.gz`
- `codefart-x86_64-apple-darwin.tar.gz`
- `codefart-x86_64-unknown-linux-gnu.tar.gz`
- `codefart-aarch64-unknown-linux-gnu.tar.gz`
- `codefart-x86_64-pc-windows-msvc.zip`

## 5. 本地打包 Desktop DMG

### Apple Credentials

本地签名需要 Developer ID Application 证书存储在 Keychain 中：

```text
Developer ID Application: Your Name (TEAMID1234)
```

公证需要 App Store Connect API Key：

```bash
export APPLE_API_KEY_ID="YOUR_KEY_ID"
export APPLE_API_ISSUER="YOUR_ISSUER_ID"
export APPLE_API_KEY_PATH="$HOME/AuthKey_${APPLE_API_KEY_ID}.p8"
```

**不要 commit `.p8`、`.p12` 或 base64 编码的证书。**

### 一次性环境准备

```bash
rustup target add aarch64-apple-darwin x86_64-apple-darwin
cd desktop && npm ci && cd ..
```

### 构建

```bash
VERSION="X.Y.Z"
SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID1234)"

# Apple Silicon
cd desktop
npx tauri build --ci --target aarch64-apple-darwin \
  --config "{\"bundle\":{\"macOS\":{\"signingIdentity\":\"$SIGNING_IDENTITY\"}}}"
cd ..
A_DMG="target/aarch64-apple-darwin/release/bundle/dmg/CodeFart_${VERSION}_aarch64.dmg"

# Intel
cd desktop
npx tauri build --ci --target x86_64-apple-darwin \
  --config "{\"bundle\":{\"macOS\":{\"signingIdentity\":\"$SIGNING_IDENTITY\"}}}"
cd ..
X_DMG="target/x86_64-apple-darwin/release/bundle/dmg/CodeFart_${VERSION}_x64.dmg"
```

不要构建 `universal-apple-darwin`，分开构建更快、更容易排查问题。

## 6. 公证 + Staple

```bash
xcrun notarytool submit "$DMG" \
  --key "$APPLE_API_KEY_PATH" \
  --key-id "$APPLE_API_KEY_ID" \
  --issuer "$APPLE_API_ISSUER" \
  --wait \
  --timeout 30m
```

Staple + 验证：

```bash
xcrun stapler staple -v "$DMG"
xcrun stapler validate -v "$DMG"
spctl -a -vv -t install "$DMG"
```

预期输出：

```text
accepted
source=Notarized Developer ID
```

## 7. 上传到 GitHub Release

```bash
TAG="v${VERSION}"
gh release upload "$TAG" "$A_DMG" "$X_DMG" --clobber
```

确认产物：

```bash
gh release view "$TAG" --json assets --jq '.assets[].name'
```

## 8. 更新 Landing Page

更新 `page/index.html` 中的下载链接：

```html
<a class="download-btn" data-arch="silicon"
   href="https://github.com/Onion-L/codefart/releases/download/vX.Y.Z/CodeFart_X.Y.Z_aarch64.dmg">Apple Silicon</a>
<a class="download-btn" data-arch="intel"
   href="https://github.com/Onion-L/codefart/releases/download/vX.Y.Z/CodeFart_X.Y.Z_x64.dmg">Intel</a>
```

```bash
git add page/index.html
git commit -m "chore: update download links to vX.Y.Z"
git push
```

---

## GitHub Actions Desktop Build（备选）

如果本地打包不便，也可以用 `.github/workflows/desktop-release.yml` 走 Actions 构建：

```bash
gh workflow run desktop-release.yml -f tag="vX.Y.Z"
```

需要以下 GitHub Secrets：

- `APPLE_CERTIFICATE`: base64 编码的 `.p12`
- `APPLE_CERTIFICATE_PASSWORD`: `.p12` 密码
- `APPLE_API_ISSUER`: App Store Connect Issuer ID
- `APPLE_API_KEY`: App Store Connect Key ID
- `APPLE_API_KEY_P8_BASE64`: base64 编码的 `.p8`

如果 Actions 公证超时或卡住，回到上面的本地构建流程。

## 删除旧 Assets

```bash
gh release delete-asset "$TAG" "CodeFart_X.Y.Z_universal.dmg" -y
```
