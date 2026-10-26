//! Chapter 14: Traits and Composition

use std::fmt;

// ---------------------------------------------------------------- the trait
pub trait Notifier {
    fn notify(&self, message: &str) -> Result<(), NotifyError>;

    /// A default method: implementors get this for free, and may override it.
    fn name(&self) -> &'static str {
        "unnamed"
    }
}

#[derive(Debug)]
pub enum NotifyError {
    Transport(String),
}

impl fmt::Display for NotifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(why) => write!(f, "transport failed: {why}"),
        }
    }
}

impl std::error::Error for NotifyError {}

// ------------------------------------------------------------ two implementors
struct EmailNotifier {
    smtp: String,
}

impl Notifier for EmailNotifier {
    fn notify(&self, message: &str) -> Result<(), NotifyError> {
        println!("email via {}: {message}", self.smtp);
        Ok(())
    }
    fn name(&self) -> &'static str {
        "email"
    }
}

struct SlackNotifier {
    webhook: String,
}

impl Notifier for SlackNotifier {
    fn notify(&self, message: &str) -> Result<(), NotifyError> {
        if self.webhook.is_empty() {
            return Err(NotifyError::Transport("webhook is empty".into()));
        }
        println!("slack via {}: {message}", &self.webhook[..8.min(self.webhook.len())]);
        Ok(())
    }
    // name() is not overridden: it falls back to the default, "unnamed".
}

// ------------------------------------------------------- generic vs impl Trait
/// Two ways to say the same thing. Both are monomorphised: the compiler emits
/// one version per concrete type, so both are equally fast.
fn send_alert(n: &impl Notifier) {
    let _ = n.notify("disk is low");
}

fn send_alert2<N: Notifier>(n: &N) {
    let _ = n.notify("disk is low");
}

// --------------------------------------------------------------- composition
#[derive(Debug, Clone, PartialEq)]
struct Record {
    ts: u64,
    value: i32,
}

trait Store {
    fn save(&self, r: &Record) -> Result<(), String>;
}

trait Clock {
    fn now(&self) -> u64;
}

struct MemoryStore(std::cell::RefCell<Vec<Record>>);

impl Store for MemoryStore {
    fn save(&self, r: &Record) -> Result<(), String> {
        self.0.borrow_mut().push(r.clone());
        Ok(())
    }
}

struct FixedClock(u64);

impl Clock for FixedClock {
    fn now(&self) -> u64 {
        self.0
    }
}

/// Instead of "Logger extends ServiceBase extends Component", a service owns
/// the capabilities it needs. The three fields are the whole dependency graph.
struct Service {
    notifier: Box<dyn Notifier>,
    store: Box<dyn Store>,
    clock: Box<dyn Clock>,
}

impl Service {
    fn run(&self) -> Result<(), String> {
        let now = self.clock.now();
        self.store.save(&Record { ts: now, value: 42 })?;
        self.notifier
            .notify("saved")
            .map_err(|e| e.to_string())
    }
}

fn main() {
    let email = EmailNotifier {
        smtp: "mail.internal".into(),
    };

    println!("{}", email.name());
    send_alert(&email);
    send_alert2(&email);

    // A heterogeneous collection needs one shared size -- so store pointers.
    let notifiers: Vec<Box<dyn Notifier>> = vec![
        Box::new(EmailNotifier {
            smtp: "mail.internal".into(),
        }),
        Box::new(SlackNotifier {
            webhook: "https://hooks.example/T000".into(),
        }),
        Box::new(SlackNotifier {
            webhook: String::new(), // this one fails
        }),
    ];

    println!();
    for n in &notifiers {
        // Decided at run time, via a vtable.
        match n.notify("deploy finished") {
            Ok(()) => println!("  {} ok", n.name()),
            Err(e) => println!("  {} failed: {e}", n.name()),
        }
    }

    println!();
    let service = Service {
        notifier: Box::new(SlackNotifier {
            webhook: "https://hooks.example/T001".into(),
        }),
        store: Box::new(MemoryStore(Default::default())),
        clock: Box::new(FixedClock(1_700_000_000)),
    };
    match service.run() {
        Ok(()) => println!("service ran"),
        Err(e) => println!("service failed: {e}"),
    }
}
