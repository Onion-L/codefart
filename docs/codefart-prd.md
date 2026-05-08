# CodeFart — Product Requirements Document

**Version:** 0.1  
**Status:** Draft  
**Last Updated:** April 2026

---

## 1. Overview

### Product Summary

CodeFart is a developer utility that plays a customizable sound notification when an AI coding tool finishes generating a response. It solves the "I walked away and forgot to come back" problem that every developer using AI tools experiences daily.

### Tagline

*"You'll never miss a response again."*

### One-liner for X / socials

*"Your AI finished thinking. It will let you know — loudly."*

---

## 2. Problem

Developers using AI coding tools like Claude Code and Cursor routinely wait 10–60+ seconds for a response. During that time, they switch tabs, pick up their phone, or start doing something else. By the time they return, they've lost context, flow, and momentum.

Existing notification systems (OS notifications, terminal bells) are either too subtle, require complex setup, or don't integrate naturally into a developer's workflow.

There is no tool today that is both **instantly installable** and **genuinely fun to use**.

---

## 3. Target Users

**Primary:** Developers and indie hackers who use Claude Code, Cursor, or similar AI coding tools daily, and who are comfortable installing tools via the command line.

**Secondary (post-GUI):** Anyone using AI tools who wants ambient audio feedback — including non-developers once a GUI is available.

**User mindset:** They don't want to configure things. They want it to work the moment it's installed. But they enjoy personalizing their setup once they've decided they like something.

---

## 4. Goals

- Eliminate the "missed response" problem for developers using AI coding tools
- Be the first tool in this space that people actually share with their friends because it's funny
- Build a reputation as a small, focused, delightful utility rather than a bloated product
- Generate organic word-of-mouth through the absurdity of the concept itself

---

## 5. Non-Goals

- This is not a full notification management system
- This is not a productivity suite or AI workflow tool
- This product does not analyze AI output or interact with AI in any way
- Desktop GUI is out of scope for v1

---

## 6. Product Phases

### Phase 1 — CLI (v1, current scope)

The core product. A command-line tool that wraps any terminal command and plays a sound when it finishes.

### Phase 2 — GUI (future)

A macOS menu bar app for users who prefer a visual interface. Allows point-and-click sound management without touching the terminal. Potential monetization surface.

---

## 7. Phase 1 Requirements

### 7.1 Core Behavior

- User runs any command through CodeFart
- When the command completes, CodeFart plays a sound
- Default behavior requires zero configuration after installation

### 7.2 Installation

- Available via install script — one command to install, one command to uninstall
- No additional dependencies required
- Works immediately after installation with no setup steps

### 7.3 Default Sound

- CodeFart ships with a built-in default sound (a fart noise)
- The default sound plays immediately with no configuration required
- The default sound should be memorable and shareable — it is part of the product's identity

### 7.4 Built-in Sound Themes

CodeFart ships with a small library of named sounds that users can switch between using a single command. Suggested themes:

| Name | Description |
|---|---|
| `classic` | The signature CodeFart sound |
| `wet` | A wetter, more dramatic variant |
| `tiny` | A small, polite notification fart |
| `squeaky` | High-pitched, brief |
| `thunder` | For those long CI runs |

Naming the themes is itself a product decision — these names are shareable content.

### 7.5 Custom Sound Support

- Users can point CodeFart to any audio file on their system and set it as their notification sound
- A single command handles this — no file editing required
- Users can revert to the default sound at any time with a single command

### 7.6 Sound Listing

- Users can view all available built-in sounds with a single command
- Output is human-readable and clearly labeled

### 7.7 Configuration Storage

- User preferences (custom sound path, chosen theme) persist between sessions
- Users never need to manually edit a configuration file

---

## 8. User Experience Principles

**Zero friction first.** The time between running the install script and hearing the first fart should be under 60 seconds, including the time it takes to run a command.

**One command for everything.** Every action — changing sounds, listing themes, resetting to default — should be a single command. No multi-step flows.

**Delightful by default.** The out-of-box experience should make someone smile or laugh. This is the hook that creates word-of-mouth.

**Fail silently on audio.** If for any reason the sound can't play (wrong file type, missing file, etc.), CodeFart should not interrupt or break the underlying command. The command's output always takes priority.

---

## 9. Success Metrics (v1)

| Metric | Target (30 days post-launch) |
|---|---|
| GitHub Stars | 500+ |
| CLI installs | 200+ |
| Organic X mentions / shares | 50+ |
| Issues / feature requests filed | Signal of engagement, not a target number |

---

## 10. Launch Strategy

**Pre-launch:** Build in public on X. Share the concept before it's done. The name and premise are the pitch — post it before the product exists.

**Launch:** Demo gif showing the full workflow — run a Claude Code command, walk away (implied), hear the fart, come back. Keep it under 15 seconds.

**Post-launch hooks:**
- Invite users to share their custom sounds
- Publish the "wet" vs "tiny" vs "thunder" debate as content
- Collect funny use cases ("my coworkers now know when my AI finishes")

---

## 11. Future Considerations (Out of Scope for v1)

- **Volume control** — adjust notification volume independently of system volume
- **Conditional sounds** — different sounds for success vs. failure exit codes
- **GUI (Phase 2)** — macOS menu bar app with drag-and-drop sound management
- **Duration-based sounds** — louder/longer fart for longer waits
- **Windows / Linux support** — post-validation, if demand exists
