use std::{any::TypeId, collections::HashMap, sync::{LazyLock, RwLock}};

pub struct Debugger {
    logs: RwLock<Vec<Log>>,
    debug_values: RwLock<HashMap<String, String>>
}

impl Debugger {
    pub fn new() -> Self {
        Self { logs: RwLock::default(), debug_values: RwLock::default() }
    }

    pub fn log(
        &self,
        culprit: Option<TypeId>,
        severity: LogSeverity,
        message: String,
    ) {
        if let Ok(mut logs) = self.logs.write() {
            logs.push(Log {
                culprit,
                severity,
                message,
            });
        } else {
            panic!("Failed to writelock logs...");
        }
        
    }

    pub fn get_logs(&self) -> Vec<Log> {
        if let Ok(logs) = self.logs.read() {
            logs.clone()
        } else {
            panic!("Failed to readlock logs...");
        }
    }
}

#[derive(Clone, Debug)]
pub struct Log {
    pub culprit: Option<TypeId>,
    pub severity: LogSeverity,
    pub message: String,
}

#[derive(Clone, Debug)]
pub enum LogSeverity {
    Info,
    Minor,
    Major,
    Critical,
}

pub static DEBUGGER: LazyLock<Debugger> = LazyLock::new(Debugger::new);

#[macro_export]
macro_rules! log {
    ($culprit:ty, $level:ident, $($arg:tt)+) => {
        $crate::DEBUGGER
            .log(
                Some(std::any::TypeId::of::<$culprit>()),
                $crate::LogSeverity::$level,
                format!($($arg)+),
            )
    };
    ($level:ident, $($arg:tt)+) => {
        $crate::DEBUGGER
            .log(
                None,
                $crate::LogSeverity::$level,
                format!($($arg)+),
            )
    };
}
