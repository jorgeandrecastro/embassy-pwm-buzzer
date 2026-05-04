embassy-pwm-buzzer

[![Crates.io](https://img.shields.io/crates/v/embassy-pwm-buzzer.svg)](https://crates.io/crates/embassy-pwm-buzzer)
[![Documentation](https://img.shields.io/docsrs/embassy-pwm-buzzer/latest.svg)](https://docs.rs/embassy-pwm-buzzer)
[![License](https://img.shields.io/crates/l/embassy-pwm-buzzer.svg)](LICENSE)

**Driver asynchrone `no_std` pour piloter un buzzer passif (piezo) via PWM matériel.**

Conçu pour l'écosystème [Embassy](https://embassy.dev), ce driver permet de générer des tonalités musicales et des mélodies sur RP2040 et RP2350 sans charger le processeur, en utilisant les périphériques PWM natifs.

---

## ⚡ Caractéristiques

- **Contrôle PWM matériel** : Calcul dynamique du diviseur pour une clarté sonore optimale avec un cycle de service de 50%.
- **Async natif** : Intégration totale avec `embassy-time` pour des durées précises sans attente active.
- **Dictionnaire de notes** : Fréquences musicales standard incluses (C4, D4, etc.).
- **Sécurité et Robustesse** : `#![forbid(unsafe_code)]` et gestion rigoureuse des limites physiques du PWM.
- **Agnostique** : Compatible avec n'importe quelle puce de la famille RP (Pico 1, Pico 2, etc.).

---

## 📦 Installation

Ajoutez ceci à votre `Cargo.toml` :
```toml
[dependencies]
embassy-pwm-buzzer = { version = "0.1.0", features = ["rp235xa"] } # ou "rp2040" ou "rp235xb"
embassy-rp = "0.10"
embassy-time = "0.5"
```
---

# 🚀 UtilisationJouer une simple noteRustuse 
````rust
embassy_pwm_buzzer::{Buzzer, notes::*};
use embassy_rp::pwm::{Config, Pwm};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // Initialisation du PWM sur la broche GP16
    let pwm = Pwm::new_free(p.PWM_CH0, p.PIN_16, Config::default());
    let mut buzzer = Buzzer::new(pwm, 125_000_000); // Horloge système 125MHz

    // Joue un "La" (440Hz) pendant 500ms
    if let Err(e) = buzzer.play_note(NOTE_A4, 500).await {
        defmt::error!("Erreur buzzer : {:?}", e);
    }
}

````

## Créer une mélodie d'alerte
````rust
let melody = [
    (NOTE_C5, 100),
    (SILENCE, 50),
    (NOTE_C5, 100),
    (NOTE_G4, 300),
];

for (note, duration) in melody {
    buzzer.play_note(note, duration).await?;
}
````
---

## 🔴 Câblage (Exemple RP2350 / Pico 2)

| Broche Buzzer | Connexion    | Note                                       |
| :------------ | :----------- | :----------------------------------------- |
| **VCC / +3.3V**   | GP18 (PWM)  | N'importe quelle broche compatible PWM     |
| **GND / -**   | GND           | Masse commune                              |


---

# 🛠️ Gestion des erreurs

Le driver utilise **l'énumération** BuzzerError pour assurer la stabilité du système :
- FrequencyTooLow : La fréquence demandée dépasse les capacités du registre 16 bits du PWM.
- FrequencyTooHigh : La fréquence dépasse 20 kHz (limite audible et matérielle).

---

# 📜 Licence
GPL-2.0-or-later — Copyright (C) 2026 Jorge Andre Castro.