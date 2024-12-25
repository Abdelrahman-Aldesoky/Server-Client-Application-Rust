use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone, Copy)]
pub enum Component {
    Server,
    Client,
    Test,
}

impl Component {
    pub(crate) fn config(&self) -> (&'static str, LevelFilter) {
        match self {
            Component::Server => ("server", LevelFilter::INFO),
            Component::Client => ("client", LevelFilter::INFO),
            Component::Test  => ("test",   LevelFilter::TRACE),
        }
    }
}