#[derive(PartialEq, PartialOrd, Clone, Copy)]
enum LogLevel {
    Disabled = 0,
    Debug = 100,
}

static mut LOG_LEVEL: LogLevel = LogLevel::Disabled;

fn set_log_level(level: LogLevel) {
    unsafe { LOG_LEVEL = level };
}

pub fn enable_debug() {
    set_log_level(LogLevel::Debug);
}

pub fn disable() {
    set_log_level(LogLevel::Disabled);
}

pub struct Logger {
    namespace: String,
}

impl Logger  {
    // TODO this should be a macro
    pub fn new(namespace: String) -> Logger {
        Logger { namespace }
    }

    fn print_log(&self, level: LogLevel, msg: String) {
        if (unsafe { LOG_LEVEL } >= level) {
            println!("{}", format!("[{}][DEBUG] {msg}", self.namespace))
        }
    }

    pub fn debug(&self, msg: String) {
        self.print_log(LogLevel::Debug, msg);
    }
}
