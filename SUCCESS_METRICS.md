# Success metrics

## Quality and performance
- Focus to speech latency p95 <= 150 ms on a reference Windows laptop
- Cold start time <= 1.0 s after warm cache
- Idle memory <= 150 MB on the reference machine
- Say all reads a 50 KB HTML page without underruns
- Crash rate during demo period <= 0.5 per active tester per day

## Accessibility and coverage
- One refreshable braille display fully usable end to end
- en-GB and sw speech phrases are correct for 30 common roles and states
- Keyboard only operation covers the full demo script
- At least one screen magnifier user confirms the app does not interfere with magnification

## Community and delivery
- 30 or more unique testers download and run the demo
- 20 or more survey responses
- Time to first response on issues under 48 hours during the demo period
- Stars or watchers are secondary but track them for trend

## Product understanding
- 80 percent of survey respondents can name at least two strengths and one weakness
- 70 percent report they would try the next build

## How to measure
- Manual timing script for latency using a UIA test harness
- Windows Performance Recorder baseline to cross check CPU and memory
- Crash logs in the user folder with a one click zip command
- Survey responses collected in a CSV exported from the chosen form tool
