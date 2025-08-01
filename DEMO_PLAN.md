# Public Demo Plan

Date generated: 2025-08-01

## Purpose
Show a working slice of the screen reader to early adopters and gather focused feedback that improves the MVP. Keep the install small, the demo short, and the calls to action clear.

## Scope of the demo
- Platform: Windows
- Input and output: keyboard, speech via eSpeak NG or Piper, one refreshable braille display from the test list
- Scenarios covered
  1. Focus change to speech in a desktop app
  2. Quick navigation on a simple web page
  3. Forms mode vs reading mode
  4. Say all on a long page with pause and resume
  5. Braille output following focus
  6. Language switch between en GB and sw

## Audience
- Screen reader users who can try a pre alpha build
- Developers and translators who can open issues or PRs
- Educators who can test with students in a lab setup

## Distribution
- Prebuilt signed installer if available or a zip with a portable build
- README section titled Demo build with steps to start and quit
- Test pages included as a folder and linked from the demo start menu

## Privacy and safety
- No telemetry by default
- Demo prompts user before any network call
- Bug report bundle strips personal data and is opt in

## Timeline
- Prep: 2025 08 01 to 2025 08 08
- Dry runs with 3 to 5 trusted testers: 2025 08 11 to 2025 08 15
- Public demo release 0.1.0 demo: 2025 08 19
- Community call and Q and A: 2025 08 26
- Feedback triage and fixes: 2025 08 20 to 2025 09 05
- Alpha go or no go review: 2025 09 12

## Artifacts to publish
- Demo build download
- DEMO script below as a PDF and Markdown in the repo
- Short screencast with narrated audio
- Survey link and instructions for filing issues
- Known issues list

## Roles
- Demo lead
- Release engineer
- Note taker for the community call
- Support triage owner for issues and discussions

## Demo script
1. Launch the screen reader and confirm a welcome message within 2 seconds.
2. Open File Explorer. Move focus across items. Expect speech to start within 150 ms.
3. Open testpages index.html in Chrome or Firefox. Use H to move across headings.
4. Press K to move across links. Use D to move across landmarks.
5. Press SR plus Space to enter forms mode on the forms page. Tab to a checkbox. Toggle it.
6. Start Say all with SR plus Down on the tables page. Pause, resume, then stop.
7. Connect a braille display. Move focus across tabs in the ARIA widgets page and watch routing.
8. Open Settings. Switch language from en GB to Kiswahili. Repeat a heading jump to confirm phrases.

## Logistics checklist
- Sign the build or document SmartScreen steps
- Ship test pages folder and a start shortcut to index.html
- Include a portable build in case the installer fails
- Add a recovery hotkey to exit the screen reader quickly
- Provide a bug report bundle command in the Help menu

## Calls to action
- Fill the short survey after 10 minutes of use
- File issues with the label demo where relevant
- Join the community call and share audio notes if possible
