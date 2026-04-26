# Enclosure

3D-printed enclosure for Rune. Target dimensions: 78x78x13mm.

## Current Status

Iterating on prototype prints. Design is not finalized.

## Printer Requirements

Any FDM printer will work if it meets these minimums:

- **Nozzle:** 0.4mm (standard)
- **Build volume:** 200x200x200mm or larger
- **Heated bed:** Required for PETG, optional for PLA

## Materials

- **PLA** -- Fine for prototyping and test fits. Cheap, easy to print, but brittle and warps in heat (car dashboard, direct sun).
- **PETG** -- Recommended for daily-use enclosures. Slightly harder to print, but tougher and more heat-resistant than PLA.

## Print Settings

| Parameter | Value |
|-----------|-------|
| Layer height | 0.2mm |
| Infill | 20% |
| Supports | None needed (design avoids overhangs > 45 degrees) |
| Perimeters | 3 |
| Top/bottom layers | 4 |
| Print speed | 50-60mm/s (PLA), 40-50mm/s (PETG) |

## Heat-Set Inserts

The enclosure uses M2 brass heat-set inserts (M2x3mm OD x 3.5mm length) for the screw bosses. You need 4 inserts total.

To install:

1. Place the insert on the hole in the bottom shell.
2. Press in with a soldering iron set to 200-220C (PLA) or 240-260C (PETG).
3. Push straight down until the insert sits flush with the surface.
4. Let cool for 10 seconds before handling.

Use a dedicated insert tip for your soldering iron if you have one. A standard conical tip works but is less consistent.

## Assembly

The enclosure is two halves: a top shell (holds the display) and a bottom shell (holds the PCB and battery).

1. Install heat-set inserts into the 4 bosses in the bottom shell.
2. Place the PCB into the bottom shell. It snap-fits on alignment tabs.
3. Route the e-ink FPC cable through the slot between shells.
4. Connect the display to the FPC cable.
5. Seat the top shell onto the bottom shell. The edges snap together.
6. Secure with 4x M2x6mm screws through the top shell into the inserts.

## STL Files

See the [stl/](stl/) subdirectory. Files will be published when the design is finalized.
