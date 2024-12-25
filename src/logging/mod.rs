mod setup;
mod types;

pub use types::Component;
use setup::init_logging;

#[inline]
pub fn init(component: Component) -> Result<(), Box<dyn std::error::Error>> {
    init_logging(component)
}

pub mod prelude {
    use super::*;

    #[inline]
    pub fn init_server() -> Result<(), Box<dyn std::error::Error>> {
        init(Component::Server)
    }
    
    #[inline]
    pub fn init_client() -> Result<(), Box<dyn std::error::Error>> {
        init(Component::Client)
    }
    
    #[inline]
    pub fn init_test() -> Result<(), Box<dyn std::error::Error>> {
        init(Component::Test)
    }
}

pub use prelude::*;