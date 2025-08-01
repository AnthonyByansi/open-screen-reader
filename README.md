# Open Screen Reader

Fast, multilingual, and customizable screen reader for everyone.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/your-org/your-repo/ci.yml?label=CI)](.github/workflows/ci.yml)
[![Lint](https://img.shields.io/github/actions/workflow/status/your-org/your-repo/lint.yml?label=lint)](.github/workflows/lint.yml)
[![CodeQL](https://img.shields.io/github/actions/workflow/status/your-org/your-repo/codeql.yml?label=CodeQL)](.github/workflows/codeql.yml)

> Goal: a free and open source screen reader that is quick to respond, light on resources, and friendly to new and advanced users.



## Contents
- [Why this project](#why-this-project)
- [Features in the MVP](#features-in-the-mvp)
- [Roadmap](#roadmap)
- [Getting started](#getting-started)
- [Build from source](#build-from-source)
- [Configuration](#configuration)
- [Keymap draft](#keymap-draft)
- [Contributing](#contributing)
- [Localization](#localization)
- [Security](#security)
- [Privacy](#privacy)
- [License](#license)
- [Credits](#credits)
- [Community](#community)

## Why this project
Many screen readers are expensive or heavy on system resources, and some have limited language options or few ways to tailor the experience. This project focuses on speed, control, and broad language support with a permissive contribution model.

## Features in the MVP
- **Speech**
  - eSpeak NG and Piper drivers for offline voices
  - Speech queue with priorities and interrupt rules
  - SSML subset for emphasis, pauses, and prosody
- **Navigation**
  - Quick navigation for headings, links, and landmarks
  - Forms mode and reading mode
  - Say all with pause and resume
- **Web**
  - Browser extension scaffold for Chrome and Firefox
  - ARIA role and state verbosity rules
- **Braille**
  - Liblouis output
  - BRLTTY and HID devices on Windows
- **Settings and onboarding**
  - Settings window with search
  - First run tutorial
- **Localization**
  - Gettext wiring plus one extra language
- **Quality targets**
  - Focus to speech under 150 ms on a reference Windows laptop
  - Cold start near 1 second after warm cache
  - Idle memory near 150 MB

## Roadmap
See [MVP_SPEC.md](MVP_SPEC.md) and the GitHub issues imported from `mvp_issues.csv`.
Milestones:
- M1 Foundation
- M2 Interaction
- M3 Web and Braille
- M4 Alpha readiness

## Getting started
1. Download a release from the Releases page when available. For now, build from source.
2. On first run, a short audio tutorial explains basic keys.
3. Open Settings to adjust speech rate, pitch, volume, and verbosity.

## Build from source

### Prerequisites on Windows
- Windows 10 or later
- Git
- Rust toolchain or MSVC C++ toolchain
- Python 3.11 or later for scripts
- Node.js LTS for tooling
- eSpeak NG or Piper installed for local TTS tests
- Visual Studio Build Tools if building C++

### Rust path
```
cargo build
cargo test
```

### CMake path
```
cmake -S . -B build
cmake --build build --config Release
ctest --test-dir build --output-on-failure -C Release
```

### Browser extension
The extension communicates with the host via Native Messaging. Instructions will land in `/extensions` once the scaffold is pushed.

## Configuration
Config files use TOML. Example:

```toml
[speech]
engine = "espeakng"    # or "piper"
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
mode = "short"         # or "verbose"

[ui]
language = "en"
theme = "system"
```

## Keymap draft
- Say all: NVDA+Down
- Stop speech: NVDA+Shift+S
- Next heading: H, previous heading: Shift+H
- Next link: K, previous link: Shift+K
- Next landmark: D, previous landmark: Shift+D
- Toggle forms mode: NVDA+Space
- Settings: NVDA+Control+S
- Help: NVDA+H
- Tutorial: NVDA+T

## Contributing
We welcome issues, ideas, and pull requests. Start with:
- [CONTRIBUTING.md](CONTRIBUTING.md)
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)
- Good first issues are labeled `good first issue`

All contributions are licensed under GPL-3.0-or-later. Use DCO sign off in your commits.

## Localization
We use gettext. Strings will live in `/po`. Translators can open a PR with updated `.po` files or join the translation platform once available.

## Security
See [SECURITY.md](SECURITY.md). Report issues to [yet] or by using GitHub Security Advisories if enabled.

## Privacy
No telemetry by default. Any cloud features are opt in and clearly labeled.

## License
GPL-3.0-or-later. See [LICENSE](LICENSE).

## Credits
This project builds upon great work by eSpeak NG, Piper, Liblouis, BRLTTY, and accessibility APIs such as UI Automation.

## Community
- Issues: https://github.com/anthonybyansi/issues
- Discussions: https://github.com/anthonybyansi/open-screen-reader/discussions
- Conduct: https://github.com/anthonybyansi/open-screen-reader/blob/main/CODE_OF_CONDUCT.md
- Security: https://github.com/anthonybyansi/open-screen-reader/blob/main/SECURITY.md
