# Philosophy

## The gap nobody filled

The Rabbit R1 and Humane AI Pin both shipped in 2024 with the same pitch: replace your phone with something smarter and smaller. Both failed, but not because the idea was wrong. They failed because they tried to be everything and delivered theater instead.

The R1 ran an Android VM behind a custom launcher. The AI Pin projected a laser onto your palm so you could pretend you didn't need a screen. Both devices cost hundreds of dollars, required monthly subscriptions, and couldn't reliably do the one thing they promised -- talk to an AI and get a useful answer back. They were demos that shipped as products.

At the other end sits the Kindle. Single purpose, e-ink, excellent battery life. It does one thing well. But it's locked into Amazon's ecosystem, it doesn't let you own your books in any meaningful sense, and it has no ambition beyond selling you more content.

Rune sits between these two failures. It is not trying to replace your phone. It is not trying to be a general-purpose computer. It does five things -- voice queries, ePub reading, music playback, phone notifications, and photo capture -- and it does them on a 3.7" monochrome e-ink display in a device that fits in your pocket.

That's it. That's the product.

## Why e-ink

The display choice is philosophical, not just technical.

An e-ink screen cannot show a feed. It cannot play video. It cannot render animations smoothly enough to be addictive. It refreshes slowly, it's black and white, and it looks like paper. These are not limitations we're working around. They are the point.

Every modern device is optimized to capture and hold your attention. Rune is optimized to answer your question and then get out of the way. The screen technology enforces this at the hardware level. You cannot doomscroll on e-ink. You cannot watch TikTok. The display physically will not cooperate with attention-harvesting design patterns.

This also has practical consequences. E-ink draws zero power when displaying a static image. The display only consumes energy during a refresh. This means Rune can show you a book page, a notification, or a now-playing screen indefinitely without draining the battery. A 1000mAh cell lasts days, not hours.

## Why open source

Rune's firmware is AGPL-3.0. The hardware is CERN-OHL-S 2.0. Every schematic, every line of code, every PCB layout is public.

This isn't idealism. It's a trust architecture.

When you carry a device with a microphone, a camera, and a network connection, you should be able to verify what it does with those things. Closed-source firmware asks you to trust the manufacturer. Open-source firmware lets you verify. You can read the code that handles your voice data. You can confirm that the camera only activates when you press the shutter button. You can see exactly what gets sent to the cloud and what stays on device.

Open source also means the project can outlive any single company. If we disappear, the schematics and code remain. Anyone can manufacture the hardware. Anyone can fork the firmware. The device doesn't become e-waste because a startup ran out of funding.

Community contributions matter too. The T113-S4 is a capable but under-documented chip. The ESP32-S3 ecosystem is broad but messy. Getting Linux mainline support, writing clean drivers, and building a stable userspace application is real work. More eyes, more hands, better results.

## Designed to be put down

Most consumer electronics are designed to maximize engagement. More screen time, more interactions, more data, more ad impressions. The entire incentive structure of modern tech pushes toward making devices stickier.

Rune inverts this. The goal is to be useful for 30 seconds and then go back in your pocket. Ask a question, get an answer, done. Check a notification, dismiss it, done. Start a playlist, put the device down, listen.

This principle drives concrete product decisions:

**No app store.** Third-party apps would inevitably optimize for engagement. The five built-in functions are the product. If a sixth function is important enough, it goes into the firmware through the open-source contribution process.

**No browser.** A browser is a portal to everything, which means it's a portal to every attention trap ever built. Rune doesn't have one.

**No video.** The e-ink display can't do it well, and we wouldn't want it to. Video is the most effective attention-capture medium ever created. It has no place on a device designed to be put down.

**No touchscreen.** Physical buttons create a deliberate interaction model. You press a button to do a specific thing. There's no infinite canvas to swipe through, no pull-to-refresh gesture, no "just one more" scroll.

**No on-device app metrics.** Rune does not track how long you use it, how often you pick it up, or what features you engage with. There is no analytics pipeline. The device does not care whether you use it.

## What this is, plainly

Rune is a small, open, e-ink device that talks to the cloud for AI queries, reads your books, plays your music, shows your phone notifications, and takes photos. It runs Linux. You can build one yourself. You can read every line of source code. It is not trying to replace your phone, and it is not trying to become your phone.

It's a tool, not a platform. It does a few things well and then gets out of the way.
