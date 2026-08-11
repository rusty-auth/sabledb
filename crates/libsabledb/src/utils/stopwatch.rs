use crate::{SableError, Telemetry};

pub struct StopWatch {
    start: u128,
}

#[derive(Default)]
pub struct IoDurationStopWatch {
    stop_watch: StopWatch,
}

#[derive(Default)]
pub struct CommandDurationStopWatch {
    stop_watch: StopWatch,
}

#[derive(Default)]
pub struct LockAcquisitionDurationStopWatch {
    stop_watch: StopWatch,
}

#[derive(Default)]
pub struct StringGetDurationStopWatch {
    stop_watch: StopWatch,
}

#[derive(Default)]
pub struct ResponseWriteDurationStopWatch {
    stop_watch: StopWatch,
}

#[derive(Default)]
pub struct CommandPreflightDurationStopWatch {
    stop_watch: StopWatch,
}

#[derive(Default)]
pub struct StringCommandDurationStopWatch {
    stop_watch: StopWatch,
}

impl StopWatch {
    fn now_as_micros() -> Result<u128, SableError> {
        let Ok(timestamp_micros) =
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        else {
            return Err(SableError::OtherError(
                "failed to retrieve std::time::UNIX_EPOCH".to_string(),
            ));
        };
        Ok(timestamp_micros.as_micros())
    }

    pub fn elapsed_micros(&self) -> Result<u128, SableError> {
        let now = Self::now_as_micros()?;
        Ok(now.saturating_sub(self.start))
    }
}

impl Default for StopWatch {
    fn default() -> Self {
        StopWatch {
            start: Self::now_as_micros().unwrap_or_default(),
        }
    }
}

impl Drop for IoDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_io_duration(elapsed);
        }
    }
}

impl Drop for CommandDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_command_duration(elapsed);
        }
    }
}

impl Drop for LockAcquisitionDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_lock_acquisition_duration(elapsed);
        }
    }
}

impl Drop for StringGetDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_string_get_duration(elapsed);
        }
    }
}

impl Drop for ResponseWriteDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_response_write_duration(elapsed);
        }
    }
}

impl Drop for CommandPreflightDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_command_preflight_duration(elapsed);
        }
    }
}

impl Drop for StringCommandDurationStopWatch {
    fn drop(&mut self) {
        if let Ok(elapsed) = self.stop_watch.elapsed_micros() {
            Telemetry::inc_total_string_command_duration(elapsed);
        }
    }
}
