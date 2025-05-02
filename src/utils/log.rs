#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum LogLevel {
    Disabled = 0,
    Info = 90,
    Debug = 100,
}

static mut LOG_LEVEL: LogLevel = LogLevel::Disabled;

pub fn set_log_level(level: LogLevel) {
    unsafe { LOG_LEVEL = level };
}

pub struct Logger {
    namespace: &'static str,
}

impl Logger {
    pub fn new(namespace: &'static str) -> Logger {
        Logger { namespace }
    }

    pub fn debug(&self, msg: String) {
        if (unsafe { LOG_LEVEL } >= LogLevel::Debug) {
            println!("{}", format!("[{}][DEBUG] {msg}", self.namespace))
        }
    }

    pub fn info(&self, msg: String) {
        if (unsafe { LOG_LEVEL } >= LogLevel::Info) {
            println!("{}", format!("[{}][DEBUG] {msg}", self.namespace))
        }
    }
}
