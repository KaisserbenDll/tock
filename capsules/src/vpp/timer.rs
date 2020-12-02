use core::cell::Cell;
use kernel::hil::time::{self, Alarm, Frequency, Ticks, Ticks32};
use kernel::{AppId, Callback, Driver, Grant, ReturnCode};
use kernel::debug;


#[derive(Copy, Clone)]
pub struct TimerData {
    callback: Option<Callback>
}
impl Default for TimerData {
    fn default() -> TimerData {
        TimerData {
            callback: None,
        }
    }
}
pub struct Timer<'a, A: Alarm<'a>> {
    alarm: &'a A,
    app_alarms: Grant<TimerData>,
}

impl<'a, A: Alarm<'a>> Timer<'a, A> {
    pub const fn new(alarm: &'a A, grant: Grant<TimerData>) -> Timer<'a, A> {
        Timer {
            alarm: alarm,
            app_alarms: grant,
        }
    }
}

impl<'a, A: Alarm<'a>> time::AlarmClient for Timer<'a, A> {
    fn alarm(&self) {
        debug!("Got them");
        self.app_alarms.each(|alarm| {
            alarm.callback.map(|mut cb| {cb.schedule(0,0,0)});
        });
    }
}
