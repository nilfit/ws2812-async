#![no_std]

use embedded_hal_async::spi::{ErrorType, Operation, SpiDevice};
use smart_leds::RGB8;

const PATTERNS: [u8; 2] = [0b1110_0000, 0b1111_1000];

/// N = 24 * NUM_LEDS
pub struct Ws2812<SPI: SpiDevice<u8>, const N: usize> {
    spi: SPI,
    data: [u8; N],
}

impl<SPI: SpiDevice<u8>, const N: usize> Ws2812<SPI, N> {
    pub fn new(spi: SPI) -> Self {
        Self { spi, data: [0; N] }
    }

    pub async fn write(
        &mut self,
        iter: impl Iterator<Item = RGB8>,
    ) -> Result<(), <SPI as ErrorType>::Error> {
        for (led_bytes, RGB8 { r, g, b }) in self.data.chunks_mut(24).zip(iter) {
            for (i, mut color) in [g, r, b].into_iter().enumerate() {
                for ii in 0..8 {
                    led_bytes[i * 8 + ii] = PATTERNS[((color & 0b1000_0000) >> 7) as usize];
                    color <<= 1;
                }
            }
        }
        let blank = [0_u8; 280];
        self.spi
            .transaction(&mut [Operation::Write(&self.data), Operation::Write(&blank)])
            .await
    }
}
