# FAQ

---

**Why not just use a phone?**

You can. Rune doesn't replace your phone. It's a companion for the moments when you want to ask a question, read a book, or listen to music without pulling out a device that also has Twitter, email, Slack, and every other attention trap on it. If you're fine with your phone, you don't need a Rune.

---

**Why e-ink?**

Three reasons. First, it draws zero power when displaying a static image, which means multi-day battery life from a 1000mAh cell. Second, it's readable in direct sunlight. Third -- and most importantly -- it can't show a feed. E-ink is too slow for scrolling, video, or animations. This is a feature. The display technology physically prevents the device from becoming addictive.

---

**Why not OLED?**

An OLED screen would make the device faster and more responsive. It would also make it possible to add video, smooth animations, and every attention-capture pattern that exists on phones. E-ink is a deliberate constraint. See [philosophy.md](philosophy.md) for the full reasoning.

---

**Why cloud AI instead of on-device?**

The T113-S4 has dual Cortex-A7 cores and 128MB of RAM. That's enough to run Linux and a userspace application comfortably. It is not enough to run a useful speech recognition model or a language model. On-device inference would be slow, inaccurate, and would drain the battery in minutes. Cloud AI (OpenAI Realtime API by default) gives fast, high-quality results. The trade-off is that voice queries require a network connection.

---

**Why no touchscreen?**

Touchscreens invite gestures that lead to infinite scrolling. Physical buttons create deliberate interactions: press a button, something specific happens. There's no infinite canvas, no pull-to-refresh, no "just one more" swipe. Five buttons (up, down, select, back, action) are enough for five functions.

---

**Why Allwinner T113-S4?**

It's cheap (~$3-4 USD), it has 128MB DDR3 in the package (no external RAM needed, which simplifies the PCB), it runs mainline Linux, and it has the peripherals Rune needs: SPI, I2S, UART, USB host, I2C, and GPIO. The dual Cortex-A7 cores are more than enough for e-ink rendering, ePub layout, and audio decoding. The final design uses a BGA package and will need proper PCB assembly; early prototyping uses MangoPi dev boards so nobody has to hand-solder the SoC.

---

**Why an ESP32 co-processor instead of putting WiFi on the main board?**

The T113-S4 doesn't have WiFi or Bluetooth. Rather than adding a WiFi module on the main SPI/SDIO bus and dealing with driver complexity in the Linux kernel, we use an ESP32-S3 as a self-contained network co-processor. It runs its own firmware, handles all WiFi and BLE operations, and communicates with the T113 over a simple UART protocol. This keeps the Linux side clean and puts all wireless complexity in the ESP32's well-supported ecosystem.

---

**Can I use a different AI provider?**

Yes. The voice query system sends audio to a WebSocket endpoint and receives responses back. The default is OpenAI's Realtime API, but the endpoint URL and authentication are configurable. Any service with a compatible WebSocket API for streaming audio will work. You can also point it at a self-hosted model if you run one.

---

**Is this a Rabbit R1 clone?**

No. The R1 is an Android device running a custom launcher that pretends to be something new. Rune is a custom Linux board with a custom PCB, custom firmware, and an e-ink display. The R1 tried to replace your phone. Rune explicitly does not. The R1 is closed source. Rune is AGPL-3.0 and CERN-OHL-S 2.0. The only similarity is "small device that talks to AI," which describes a lot of things.

---

**What about battery life?**

Depends on usage. The e-ink display draws zero power when showing a static image, so reading and idle states are nearly free. Active WiFi usage (voice queries, Spotify streaming) draws the most power. Rough estimates for the 1000mAh battery:

- Standby with static display: days
- Active ePub reading (occasional page turns): 20+ hours
- Intermittent voice queries (a few per hour): 10-15 hours
- Continuous Spotify streaming: 4-6 hours

These are estimates. Real numbers will come from hardware testing.

---

**Can I buy one?**

Not yet. Rune is in active development. A Kickstarter campaign is planned. In the meantime, you can build one yourself from the open-source hardware files -- see [getting-started.md](getting-started.md).

---

**Is this really open source?**

Yes. All of it.

- Firmware: [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.html). Every line of code running on the T113-S4 and ESP32-S3 is in this repository.
- Hardware: [CERN-OHL-S 2.0](https://ohwr.org/cern_ohl_s_v2.txt). Schematics, PCB layouts, and enclosure files are in the `hardware/` directory. You can manufacture the board yourself.
- No proprietary blobs in the critical path. The T113-S4 bootloader (boot0/U-Boot) uses Allwinner's public SDK. The ESP32-S3 WiFi stack includes Espressif's binary WiFi library, which is standard for all ESP32 projects -- there is no open-source alternative for the WiFi radio firmware.
