use std::any::TypeId;
use std::sync::Mutex;

pub struct Debugger {
    logs: Vec<Log>,
}

impl Debugger {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn log(
        &mut self,
        culprit: Option<TypeId>,
        severity: LogSeverity,
        message: String,
    ) {
        self.logs.push(Log {
            culprit,
            severity,
            message,
        });
    }
}

struct Log {
    culprit: Option<TypeId>,
    severity: LogSeverity,
    message: String,
}

pub enum LogSeverity {
    Info,
    Minor,
    Major,
    Critical,
}

pub static DEBUGGER: Mutex<Debugger> = Mutex::new(Debugger {
    logs: Vec::new(),
});

#[macro_export]
macro_rules! log {
    ($culprit:ty, $level:ident, $($arg:tt)+) => {
        $crate::DEBUGGER
            .lock()
            .expect("Debugger mutex poisoned")
            .log(
                Some(std::any::TypeId::of::<$culprit>()),
                $crate::LogSeverity::$level,
                format!($($arg)+),
            )
    };

    ($level:ident, $($arg:tt)+) => {
        $crate::DEBUGGER
            .lock()
            .expect("Debugger mutex poisoned")
            .log(
                None,
                $crate::LogSeverity::$level,
                format!($($arg)+),
            )
    };
}
