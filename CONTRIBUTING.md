# Contributing to Open Screen Reader

Thank you for helping build an accessible screen reader for all users.

## Code of conduct
We follow the Contributor Covenant. Please read CODE_OF_CONDUCT.md. Be kind and assume good intent.

## Ways to contribute
- Report bugs and regressions
- Help with translations
- Improve docs and examples
- Write tests
- Implement features from the roadmap
- Try nightly builds and share feedback

## Project goals
- Fast response from focus to speech
- Low memory footprint
- Strong language support
- Clear defaults with room to customize

## License for contributions
By contributing, you agree that your work is licensed under **GPL-3.0-or-later**.  
Add this header at the top of new source files:

```
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (c) 2025 Open Screen Reader contributors
```

## Developer setup
- Windows 10 or later for the first target
- Git, Python 3.11 or later, and Node.js LTS for tools
- Rust stable and or MSVC C++ toolchain
- eSpeak NG or Piper installed for local TTS tests

Quick start:
```
git clone https://github.com/your-org/your-repo.git
cd your-repo
# Rust path
cargo build
cargo test
# C++ path
cmake -S . -B build
cmake --build build --config Release
ctest --test-dir build
```

## Style and tools
- Rust: `rustfmt`, `clippy`
- C++: `clang-format`, `clang-tidy`
- Python: `ruff` and `black`
- Markdown: `markdownlint`

Run linters before you push. CI will run them too.

## Branches and pull requests
- Fork the repo and create a branch from `main`
- Keep changes focused on one topic
- Write or update tests
- Update docs if behavior changes

### PR checklist
- [ ] Linked an issue
- [ ] Tests pass locally
- [ ] Added or updated tests
- [ ] Updated docs and changelog if needed

## Commit messages
Use Conventional Commits.
Examples:
- `feat(nav): add quick heading navigation`
- `fix(speech): lower latency on say all`
- `docs: update build steps for Windows`
- `test: add UIA latency test`

## Tests
- Unit tests for core and adapters
- Integration tests for speech, nav, and braille
- Latency tests that measure event to speech start

## Translations
We use gettext. Strings live in `/po`. Add or update `.po` files and run the l10n scripts.

## Security
If you find a vulnerability, email security@your-project.org. Please do not open a public issue first.

## DCO sign off
All commits must be signed off to confirm that you have the right to submit the work.

Example sign off line:
```
Signed-off-by: Your Name <you@example.com>
```

You can add it with:
```
git commit -s -m "feat: your change"
```

## Contacts
- Issues: https://github.com/your-org/your-repo/issues
- Discuss: GitHub Discussions or your preferred forum
