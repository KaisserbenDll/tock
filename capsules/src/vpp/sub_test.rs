use kernel::{AppId, Callback, Driver, Grant, ReturnCode};
use kernel::common::cells::OptionalCell;
use kernel::debug;
pub const DRIVER_NUM: usize = 0x9000A;

pub struct App{
    callback: Option<Callback>,
    // flag: bool,
}
pub struct Test{
    apps: Grant<App>,
    activeapp: OptionalCell<AppId>
}
impl Default for App {
    fn default() -> App {
        App {
            callback: None,
            // flag: false
        }
    }
}
impl Test {
    pub fn new (grant: Grant<App>)-> Test {
        Test {
            apps: grant ,
            activeapp: OptionalCell::empty(),
        }
    }
    pub fn trigger_callback (&self) -> Option<()> {
        self.activeapp.map(|app_id|{
        let _ = self.apps.enter(*app_id, |app,_|{
            app.callback.map(| mut cb| {cb.schedule(0,0,0)});
        });
        })
    }
}
impl Driver for Test {
    fn subscribe(
        &self,
        subscribe_num: usize,
        callback: Option<Callback>,
        app_id: AppId,
    ) -> ReturnCode {
        self.activeapp.set(app_id);

       self.apps.enter(app_id, |app, _| {
           match subscribe_num {

               1 => {app.callback = callback;
                   debug!("Accessing Subscribe Syscall");
               },
                   //self.trigger_callback();},
               _ => return ReturnCode::ENOSUPPORT,
           }
           ReturnCode::SUCCESS })
           .unwrap_or_else(|err| err.into())
        }
}