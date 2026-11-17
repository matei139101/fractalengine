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

/// The enum used to describe the "severity" of a log message.
#[derive(Clone, Debug)]
pub enum LogSeverity {
    /// Any information that is generally not useful for debugging but logged regardless.
    Info,
    /// Any information that is might be useful to report in a log.
    Minor,
    /// Any information that is useful to note at all times.
    Major,
    /// Any information usually relted to panics, unintended behavior or temporary debugging information.
    Critical,
}

/// The static struct of the engine's [`Debugger`]. Prefferably not used directly, but instead used
/// through the provided macros.
pub static DEBUGGER: LazyLock<Debugger> = LazyLock::new(Debugger::new);

/// This macro can be used by any other crate to log a line of information into the [`DEBUGGER`].
///
/// The macro has 2 "modes" which it can switch between depending on the provided arguments. If a
/// type is provided, the type will be added as a "culprit" alongside the message for additional
/// information. Otherwise no culprit will be known. This is useful to track where logs and errors
/// originate from.
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
