#!/usr/bin/env python3
"""Generate 5 placeholder WAV sounds for CodeFart themes."""
import math
import random
import struct
import wave

SAMPLE_RATE = 44100
OUT_DIR = "sounds"

def make_wav(name, samples):
    import os
    os.makedirs(OUT_DIR, exist_ok=True)
    path = f"{OUT_DIR}/{name}.wav"
    with wave.open(path, "w") as w:
        w.setnchannels(1)
        w.setsampwidth(2)  # 16-bit
        w.setframerate(SAMPLE_RATE)
        # Clamp to int16
        max_val = 32767
        data = b""
        for s in samples:
            s = max(-1.0, min(1.0, s))
            data += struct.pack("<h", int(s * max_val))
        w.writeframes(data)
    duration = len(samples) / SAMPLE_RATE
    print(f"  {path}  ({duration:.2f}s)")

def envelope(t, duration, attack=0.02, release=0.08):
    """Simple ADSR-like envelope."""
    if t < attack:
        return t / attack
    if t > duration - release:
        return (duration - t) / release
    return 1.0

def make_classic():
    """Low fart - descending tone with noise."""
    duration = 0.8
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        # Frequency descends from 150Hz to 60Hz
        freq = 150 - (t / duration) * 90
        phase = 2 * math.pi * freq * t
        tone = math.sin(phase) * 0.5
        # Add sub-bass
        sub = math.sin(2 * math.pi * (freq/2) * t) * 0.3
        # Add noise
        noise = (random.random() - 0.5) * 0.2
        env = envelope(t, duration)
        samples.append((tone + sub + noise) * env)
    make_wav("classic", samples)

def make_wet():
    """Wetter variant - more noise, bubbly."""
    duration = 0.7
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        freq = 120 - (t / duration) * 60
        phase = 2 * math.pi * freq * t
        tone = math.sin(phase) * 0.3
        # More noise
        noise = (random.random() - 0.5) * 0.5
        # Bubbly modulation
        bubble = math.sin(2 * math.pi * 30 * t) * 0.15
        env = envelope(t, duration, attack=0.01, release=0.1)
        samples.append((tone + noise + bubble) * env)
    make_wav("wet", samples)

def make_tiny():
    """Small polite beep."""
    duration = 0.15
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        freq = 800
        tone = math.sin(2 * math.pi * freq * t) * 0.4
        env = envelope(t, duration, attack=0.005, release=0.05)
        samples.append(tone * env)
    make_wav("tiny", samples)

def make_squeaky():
    """High pitched squeak."""
    duration = 0.25
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        # Rising then falling pitch
        if t < duration * 0.3:
            freq = 1200 + (t / (duration * 0.3)) * 800
        else:
            freq = 2000 - ((t - duration * 0.3) / (duration * 0.7)) * 1400
        tone = math.sin(2 * math.pi * freq * t) * 0.5
        env = envelope(t, duration, attack=0.01, release=0.03)
        samples.append(tone * env)
    make_wav("squeaky", samples)

def make_thunder():
    """Low rumble for long tasks."""
    duration = 1.5
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        # Very low rumble
        rumble = math.sin(2 * math.pi * 50 * t) * 0.4
        # Slow amplitude modulation
        mod = 0.5 + 0.5 * math.sin(2 * math.pi * 8 * t)
        # Heavy noise
        noise = (random.random() - 0.5) * 0.3
        env = envelope(t, duration, attack=0.05, release=0.3)
        samples.append((rumble * mod + noise) * env)
    make_wav("thunder", samples)

if __name__ == "__main__":
    print("Generating placeholder sounds...")
    make_classic()
    make_wet()
    make_tiny()
    make_squeaky()
    make_thunder()
    print("Done.")
