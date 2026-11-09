# Copyright (C) 2026 Yu Prananda
# SPDX-License-Identifier: GPL-3.0-only
"""Rebuild Jikanda's original, sample-free mono PCM alarm signals."""
import math
from pathlib import Path
import struct
import wave

RATE = 44100
OUTPUT = Path(__file__).resolve().parents[1] / "assets" / "audio"
PRESETS = {
    "soft-bell": (1.0, [(0.0, 660.0, 0.9)]),
    "double-chime": (1.15, [(0.0, 660.0, 0.48), (0.40, 880.0, 0.65)]),
    "bright-tone": (0.8, [(0.0, 988.0, 0.7)]),
}


def generate():
    OUTPUT.mkdir(parents=True, exist_ok=True)
    for name, (duration, tones) in PRESETS.items():
        samples = []
        for index in range(round(duration * RATE)):
            t = index / RATE
            value = 0.0
            for start, frequency, length in tones:
                age = t - start
                if 0 <= age < length:
                    envelope = min(age / 0.018, 1.0) * min((length - age) / 0.07, 1.0)
                    envelope *= math.exp(-4.0 * age / length)
                    value += envelope * (math.sin(2 * math.pi * frequency * age)
                                         + 0.18 * math.sin(4 * math.pi * frequency * age))
            samples.append(value)
        peak = max(abs(sample) for sample in samples)
        pcm = b"".join(struct.pack("<h", round(sample / peak * 0.22 * 32767)) for sample in samples)
        with wave.open(str(OUTPUT / f"{name}.wav"), "wb") as output:
            output.setparams((1, 2, RATE, 0, "NONE", "not compressed"))
            output.writeframes(pcm)


if __name__ == "__main__":
    generate()
