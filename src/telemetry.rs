use std::time::Duration;

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_otlp::WithExportConfig;
use tracing::{Instrument, Level, error, info, info_span, span};
use tracing_opentelemetry;
use tracing_subscriber::Registry;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, fmt};
use uuid::Uuid;

use crate::APP_CONFIG;

pub fn init_telemetry() {
    let log_level = APP_CONFIG
        .get()
        .expect("Unable to load configuraton")
        .server
        .log_level
        .as_str();
    let otlp_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic() // Utilisation de tonic pour gRPC
        .with_endpoint("http://127.0.0.1:4317")
        .with_timeout(Duration::from_secs(3))
        .build()
        .expect("Erreur lors de la création de l'exportateur OTLP");
    let otlp_provider = opentelemetry_sdk::trace::SdkTracerProvider::builder()
        .with_simple_exporter(otlp_exporter)
        .build();
    let tracer = otlp_provider.tracer("newsletter-rs");

    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
    let fmt_layer = fmt::Layer::default()
        .with_target(true)
        .with_file(true)
        .with_line_number(true);
    let filter_layer = EnvFilter::try_new(log_level)
        .unwrap_or_else(|_| EnvFilter::from_default_env())
        .add_directive("opentelemetry=trace".parse().unwrap()) // Niveau pour OpenTelemetry
        .add_directive("opentelemetry_otlp=debug".parse().unwrap()) // Niveau pour OTLP exporter
        .add_directive("opentelemetry_sdk=debug".parse().unwrap()); // Niveau pour OpenTelemetry SDK

    let subscriber = Registry::default()
        .with(telemetry_layer)
        .with(fmt_layer)
        .with(filter_layer);

    tracing::subscriber::set_global_default(subscriber)
        .expect("Erreur lors de la définition du subscriber global");

    let root = span!(Level::TRACE, "app_start", work_units = 2);
    let _enter = root.enter();
    error!("This event will be logged in the root span.");
}

pub async fn request_id_middleware(req: Request<axum::body::Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();
    let user_agent = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let span = info_span!(
        "request",
        request_id = %request_id,
        method = %req.method(),
        uri = %req.uri(),
        user_agent = %user_agent,
    );
    info!(parent: &span, "Début de la requête");

    let res = next.run(req).instrument(span.clone()).await;
    info!(parent: &span, "Fin de la requête");
    res
}
