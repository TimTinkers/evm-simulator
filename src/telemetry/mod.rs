use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
  fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry,
};

/**
  Create a tracing subscriber set to the provided `log_level`.

  # Arguments
  * `name` - A string name for this tracing subscriber.
  * `log_level` - A value specifying the logging visibility level to use for
    this subscriber.
  * `sink` - The log output destination of this tracing subscriber.

  # Returns
  A tracing subscriber that outputs spans at the given `log_level`, logs to the
  given `sink`, and uses a logging format consumable by the bunyan tool.
*/
pub fn create_subscriber<Sink>(
  name: impl Into<String>,
  log_level: impl ToString,
  sink: Sink,
) -> impl Subscriber + Send + Sync
where
  Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
  let env_filter = EnvFilter::new(log_level.to_string());
  let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);
  Registry::default()
    .with(env_filter)
    .with(JsonStorageLayer)
    .with(formatting_layer)
}

/**
  Initialize a provided tracing subscriber.

  # Arguments
  * `subscriber` - A tracing subcriber to initialize, with an associated log.
*/
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
  LogTracer::init().expect("Failed to set logger.");
  set_global_default(subscriber).expect("Failed to set subscriber.");
}
