// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Result;
use serde::Deserialize;

use crate::system::System;

use super::ExtDevice;

#[derive(Debug, Deserialize, Default)]
pub struct UsartProbeConfig {
    pub peripheral: String,
}

#[derive(Default)]
pub struct UsartProbe {
    pub config: UsartProbeConfig,
    name: String,
    rx: Vec<u8>,
    tx: Option<Vec<u8>>,
}

impl UsartProbe {
    pub fn new(config: UsartProbeConfig) -> Result<Self> {
        Ok(Self { config, ..Self::default() })
    }
}

impl ExtDevice<(), u8> for UsartProbe {
    fn connect_peripheral(&mut self, peri_name: &str) -> String {
        self.name = format!("{} usart-probe", peri_name);
        self.name.clone()
    }
    
    fn read(&mut self, _sys: &System, _addr: ()) -> u8 {
        if let Some(tx) = self.tx.as_mut() {
            tx.pop().unwrap_or(0)
        } else {
            0
        }
    }

    fn write(&mut self, _sys: &System, _addr: (), v: u8) {
        if v == 0x0a {
            // EOL
            let line = String::from_utf8_lossy(&self.rx);
            let line = line.trim();
            info!("{} '{}'", self.name, line);
            self.rx.clear();
        } else {
            self.rx.push(v);
        }
    }
}
