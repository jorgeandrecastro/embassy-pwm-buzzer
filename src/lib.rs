// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

//! # embassy-pwm-buzzer
//!
//! Driver pour buzzer passif (piezo) utilisant le PWM matériel du RP2040/RP2350.
//! Optimisé pour l'écosystème asynchrone Embassy.

#![no_std]
#![forbid(unsafe_code)]

pub mod error;
pub mod notes;

use error::BuzzerError;
use embassy_rp::pwm::{Config, Pwm}; // Instance a été retiré dans Embassy 0.10.0
use embassy_time::Timer;

/// Driver pour buzzer passif.
pub struct Buzzer<'d> {
    pwm: Pwm<'d>,
    clock_freq: u32,
}

impl<'d> Buzzer<'d> {
    /// Crée un nouveau driver Buzzer.
    /// `clock_freq` est la fréquence système (ex: 125_000_000 pour 125MHz).
    pub fn new(pwm: Pwm<'d>, clock_freq: u32) -> Self {
        Self { pwm, clock_freq }
    }

    /// Joue une note à une fréquence précise (Hz) pendant une durée (ms).
    pub async fn play_note(&mut self, freq: u32, duration_ms: u64) -> Result<(), BuzzerError> {
        if freq == 0 {
            self.stop();
        } else {
            self.apply_frequency(freq)?;
        }

        Timer::after_millis(duration_ms).await;
        self.stop();
        Ok(())
    }

    /// Applique une fréquence au PWM sans bloquer l'exécution.
    pub fn apply_frequency(&mut self, freq: u32) -> Result<(), BuzzerError> {
        // Limites de l'audition humaine et protection matérielle
        if freq < 20 { return Err(BuzzerError::FrequencyTooLow); }
        if freq > 20_000 { return Err(BuzzerError::FrequencyTooHigh); }

        let mut config = Config::default();
        
        // Calcul pour obtenir la fréquence : freq = clock / (div * top)
        // Utilisation d'un diviseur fixe de 16 pour une bonne résolution audio.
        const DIV: u32 = 16;
        config.divider = (DIV as u8).into(); 
        
        let top = (self.clock_freq / DIV) / freq;
        
        // Vérification des limites du registre TOP (16 bits)
        if top > 0xFFFF || top == 0 { 
            return Err(BuzzerError::FrequencyTooLow); 
        }
        
        config.top = top as u16;
        config.compare_a = (top / 2) as u16; // Duty cycle 50% = clarté sonore maximale
        
        self.pwm.set_config(&config);
        Ok(())
    }

    /// Coupe le signal PWM immédiatement.
    pub fn stop(&mut self) {
        let mut config = Config::default();
        config.compare_a = 0;
        self.pwm.set_config(&config);
    }
}