// use kernel::{Callback, Driver,Grant, ReturnCode,AppId};
use kernel::debug;
use kernel::{Driver,ReturnCode,AppId};

pub const DRIVER_NUM: usize = 0x90003 ;


// pub struct App {
//     callback: Option<Callback>
// }
pub struct ProcessManager;
// {
//     apps: Grant<App>
// }
// impl Default for App {
//     fn default() -> App {
//         App {
//             callback: None,
//         }
//     }
// }
impl ProcessManager{
    pub fn new()-> ProcessManager {
        ProcessManager {   }
    }
}

impl Driver for ProcessManager {
    fn command(&self,
               command_num: usize,
               _data: usize,
               _data2: usize,
               _app: AppId) -> ReturnCode {
        match command_num {
           0 => ReturnCode::SUCCESS,
           1 => {debug!("Tested");
            ReturnCode::SUCCESS
           },
           _ => ReturnCode::ENOSUPPORT,

        }
    }
}