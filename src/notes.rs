// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

//! # Dictionnaire de Notes Musicales
//! 
//! Ce module contient les fréquences (en Hz) des notes de musique standard
//! basées sur le tempérament égal avec le La4 à 440 Hz.

/// Absence de son
pub const SILENCE: u32 = 0;

// --- Octave 3 ---
pub const NOTE_C3: u32 = 131;
pub const NOTE_D3: u32 = 147;
pub const NOTE_E3: u32 = 165;
pub const NOTE_F3: u32 = 175;
pub const NOTE_G3: u32 = 196;
pub const NOTE_A3: u32 = 220;
pub const NOTE_B3: u32 = 247;

// --- Octave 4 (Octave centrale) ---
pub const NOTE_C4: u32 = 262; // Do
pub const NOTE_CS4: u32 = 277; // Do#
pub const NOTE_D4: u32 = 294; // Re
pub const NOTE_DS4: u32 = 311; // Re#
pub const NOTE_E4: u32 = 330; // Mi
pub const NOTE_F4: u32 = 349; // Fa
pub const NOTE_FS4: u32 = 370; // Fa#
pub const NOTE_G4: u32 = 392; // Sol
pub const NOTE_GS4: u32 = 415; // Sol#
pub const NOTE_A4: u32 = 440; // La (Référence)
pub const NOTE_AS4: u32 = 466; // La#
pub const NOTE_B4: u32 = 494; // Si

// --- Octave 5 ---
pub const NOTE_C5: u32 = 523;
pub const NOTE_CS5: u32 = 554;
pub const NOTE_D5: u32 = 587;
pub const NOTE_DS5: u32 = 622;
pub const NOTE_E5: u32 = 659;
pub const NOTE_F5: u32 = 698;
pub const NOTE_FS5: u32 = 740;
pub const NOTE_G5: u32 = 784;
pub const NOTE_GS5: u32 = 831;
pub const NOTE_A5: u32 = 880;
pub const NOTE_AS5: u32 = 932;
pub const NOTE_B5: u32 = 988;

// --- Octave 6 (Aigu) ---
pub const NOTE_C6: u32 = 1047;
pub const NOTE_D6: u32 = 1175;
pub const NOTE_E6: u32 = 1319;