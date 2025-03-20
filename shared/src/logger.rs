static INIT: std::sync::Once = std::sync::Once::new();

#[derive(derive_new::new)]
pub struct Logger {
    level: tracing::Level,
}

impl Logger {
    pub fn init(&self) {
        INIT.call_once(|| {
            tracing_subscriber::fmt().with_max_level(self.level).init();
            tracing::info!("logger initialized Level:{:?}", self.level);
        });
    }
}

pub fn logger_init_info() {
    let logger = Logger::new(tracing::Level::INFO);
    logger.init();
}
