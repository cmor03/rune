# Rune UI Animation Notes

The production display is e-ink, so animation should be used sparingly. These
motions are for state transitions, LCD prototyping, and future partial-refresh
experiments. Every animation should degrade to a static frame cleanly.

## Frame Model

- Canvas: 480x280 grayscale
- Default preview rate: 12 fps
- E-ink target rate: event-based, not continuous
- Animation input: `FrameContext { frame, total, battery_pct }`
- Render output: full grayscale frame, with optional 1-bit packing through
  `display::pack_1bpp`

## Current Animations

| Name | Screen | Purpose | Frames |
|------|--------|---------|--------|
| Wake raven | `wake` | Companion crosses screen after wake | 24 |
| Voice pulse | `voice` | Listening state, microphone active | 24 |
| Camera scan | `camera` | Viewfinder scan line | 24 |
| Launcher focus | `home` | Selected tile preview movement | 48 |
| Music progress | `music` | Playback progress preview | 48 |

## E-Ink Rules

- Sleep, reader, music, notification, and home screens should normally render as
  static frames.
- Wake and voice animation can be shown on the Raspberry Pi LCD, but final
  hardware should collapse them to a small number of partial-refresh frames.
- Avoid full-screen flashing unless the panel needs a full refresh to clear
  ghosting.
- Keep animated regions small when possible.

## Porting Targets

The same screen renderer should support:

1. PGM frames for local visual inspection.
2. Raspberry Pi LCD output through `fbdev` or `drm`.
3. Final e-ink output through an SPI panel backend.

Do not put timing sleeps inside screen render functions. Timing belongs in the
app loop or display backend.
