// Copyright (C) 2026 Jorge Andre Castro
// GPL-2.0-or-later

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Le cfg_attr permet d'utiliser defmt seulement s'il est activé
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BuzzerError {
    /// Fréquence trop basse pour le matériel (registre TOP > 65535).
    FrequencyTooLow,
    /// Fréquence trop haute (risque d'endommager le piezo ou inaudible).
    FrequencyTooHigh,
}