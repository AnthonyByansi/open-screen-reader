
# MVP Spec — Screen Reader (Windows first)

## Goal
Deliver a free and open source screen reader that is fast, multilingual, and customizable. First target Windows. Android prototype can follow after alpha.

## Out of scope for MVP
- Voice cloning
- Cloud AI features
- OCR and math
- iOS and macOS

## Success criteria
- Focus to speech under 150 ms on a reference Windows laptop
- Say all reads long documents without stutter
- Works in Chrome and Firefox via our extension for basic reading and forms
- One refreshable braille display works end to end
- App is usable in English plus one more language

## Architecture
- Core in Rust or C++
- Adapters: Windows UIA
- Managers: speech, braille, nav, config
- Add-on system for future modules
- No telemetry by default

## Features in MVP
1. Speech
   - Drivers for eSpeak NG and Piper
   - Speech queue with interrupt rules
   - SSML subset support
2. Navigation
   - Quick nav for headings, links, landmarks
   - Forms mode and reading mode
   - Say all with pause and resume
3. Web
   - Browser extension scaffold
   - Verbosity rules for ARIA roles and states
4. Braille
   - Liblouis output
   - BRLTTY and HID devices on Windows
5. Settings and onboarding
   - Settings UI with search
   - First run tutorial
6. Localization
   - Gettext wiring and one extra language
7. Performance and quality
   - Latency tests and performance budget
   - Logging and bug report bundle

## Non functional targets
- Cold start under 1 second after warm cache
- Idle memory under 150 MB
- 95th percentile event to speech under 150 ms

## Test matrix
- Apps: File Explorer, Notepad, Word, Excel, Edge or Chrome, Firefox, Teams or Slack, VS Code
- Web pages: headings and landmarks page, forms page, tables page, ARIA widgets page

## Keymap draft
- Say all: NVDA+Down
- Stop speech: NVDA+Shift+S
- Quick nav next heading: H, previous heading: Shift+H
- Quick nav next link: K, previous link: Shift+K
- Next landmark: D, previous landmark: Shift+D
- Toggle forms mode: NVDA+Space
- Settings: NVDA+Control+S
- Help: NVDA+H
- Tutorial: NVDA+T

## Config schema sketch (TOML)
```toml
[speech]
engine = "espeakng"  # or "piper"
rate = 45
pitch = 50
volume = 80
voice = "en-us"

[braille]
enabled = true
table = "en-us-g2.ctb"
input = "computer"
cursor_blink = true

[navigation]
quicknav_enabled = true
forms_auto_mode = true
say_all_chunk_ms = 800

[verbosity]
mode = "short"  # or "verbose"

[ui]
language = "en"
theme = "system"
```

## Risks and mitigations
- UIA edge cases in certain apps. Mitigation: targeted shims and app profiles.
- Latency spikes due to audio stack. Mitigation: small audio buffer, thread priority, and warm caches.
- Braille device variety. Mitigation: focus on two devices first.

## Milestones
- M1 Foundation. Repo, core, UIA, speech, config.
- M2 Interaction. Nav, settings UI, tutorial.
- M3 Web and Braille. Web extension, ARIA rules, braille, perf tests.
- M4 Alpha readiness. Signed builds, localization, QA plan, privacy docs.


