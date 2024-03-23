use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig as _;
use opentelemetry_sdk::Resource;
use tracing_subscriber::{layer::SubscriberExt, prelude::*};

pub fn setup_tracing(service_name: &'static str, endpoint: &str) {
    opentelemetry::global::set_text_map_propagator(
        opentelemetry_jaeger_propagator::Propagator::new(),
    );

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(endpoint),
        )
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_resource(Resource::new(vec![KeyValue::new("service.name", service_name)])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("couldn't create tracer");

    let fmt_layer = tracing_subscriber::fmt::layer();

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(fmt_layer)
        .with(telemetry_layer)
        .init();
}