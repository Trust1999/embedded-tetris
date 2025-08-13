use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::sys::EspError;
use esp_idf_hal::timer::{Timer, TimerConfig, TimerDriver};

pub struct Time<'d> {
    timer: TimerDriver<'d>,
    last_ms: u64,
    current_ms: u64,
}

impl game::time::Time for Time<'_> {
    fn now_ms(&self) -> u64 {
        self.current_ms
    }

    fn delta_time_ms(&self) -> u64 {
        self.current_ms - self.last_ms
    }
}

impl<'d> Time<'d> {
    pub fn setup(timer_peripheral: impl Peripheral<P = impl Timer> + 'd) -> Result<Self, EspError> {
        let config = TimerConfig::new().auto_reload(true).divider(8000);
        let timer = TimerDriver::new(timer_peripheral, &config)?;
        log::info!("Time timer tick rate: {}Hz", timer.tick_hz());

        Ok(Self {
            timer,
            last_ms: 0,
            current_ms: 0,
        })
    }

    pub fn start(&mut self) -> Result<(), EspError> {
        self.timer.set_counter(0)?;
        self.timer.enable(true)?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), EspError> {
        self.last_ms = self.current_ms;
        self.current_ms = self.timer.counter()? / 10;

        log::debug!("time = {}s", game::time::Time::now_ms(self) / 1000);
        log::debug!("delta time = {}ms", game::time::Time::delta_time_ms(self));

        Ok(())
    }
}
