# Logging

Simplelog is what I have used: https://crates.io/crates/simplelog. Notes on Simplelog are at the bottom of this page.

## The most common way is to use the `log` crate with an implementation like `env_logger`. Here's how:

1. Add to `Cargo.toml`:
```toml
[dependencies]
log = "0.4"
env_logger = "0.10"
```

2. Basic usage:
```rust
use log::{info, warn, error, debug};

fn main() {
    // Initialize logger (put this at start of program)
    env_logger::init();

    // Use different log levels
    error!("This is an error!");
    warn!("This is a warning");
    info!("This is info");
    debug!("This is debug info");
}
```

3. Run with logging enabled:
```bash
RUST_LOG=debug cargo run    # Show all logs
RUST_LOG=info cargo run     # Show info and above
RUST_LOG=error cargo run    # Show only errors
```

You can also set specific modules' log levels:
```bash
RUST_LOG=my_module=debug,other_module=info cargo run
```

## Want me to show more advanced logging configurations or other logging crates?

Here's a more comprehensive example showing different logging configurations and patterns:

```rust
use log::{info, warn, error, debug, LevelFilter};
use env_logger::{Builder, WriteStyle};
use std::env;

fn setup_logger() {
    // Method 1: Basic custom builder
    Builder::new()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .init();
}

fn setup_logger_with_timestamp() {
    // Method 2: Custom format with timestamps
    Builder::new()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(buf, 
                "{} [{}] - {}", 
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}

fn setup_logger_advanced() {
    // Method 3: More advanced configuration
    let env = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    
    Builder::new()
        .parse_env(&env)
        .format(|buf, record| {
            use std::io::Write;
            let level_style = buf.style()
                .set_bold(true)
                .set_color(match record.level() {
                    log::Level::Error => env_logger::fmt::Color::Red,
                    log::Level::Warn => env_logger::fmt::Color::Yellow,
                    log::Level::Info => env_logger::fmt::Color::Green,
                    log::Level::Debug => env_logger::fmt::Color::Blue,
                    log::Level::Trace => env_logger::fmt::Color::Cyan,
                });
            
            writeln!(
                buf,
                "{} {} [{}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_style.value(record.level()),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();
}

// Example using different logging crates
fn setup_alternative_loggers() {
    // Using slog (more powerful but more complex)
    use slog::{Logger, Drain, o, info};
    use slog_term::{TermDecorator, CompactFormat};
    use slog_async::Async;

    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")));
    
    info!(log, "Application started"; "time" => chrono::Local::now().to_rfc3339());
}

fn main() {
    // Choose one setup method
    setup_logger_advanced();

    // Add Cargo.toml dependencies for timestamps:
    // chrono = "0.4"
    // For alternative logger:
    // slog = "2.7"
    // slog-term = "2.9"
    // slog-async = "2.7"

    // Example usage
    error!("System error: Could not connect to database");
    warn!("Warning: Connection slow");
    info!("Server started on port 8080");
    debug!("Debug: Received request for /api/users");

    // Structured logging with additional context
    info!(
        target: "api_service",
        "Request processed: {} status: {}", 
        "/api/users", 
        200
    );

    // Log with dynamic levels
    let level = log::Level::Info;
    log!(level, "Dynamic log level message");

    // Conditional logging
    if cfg!(debug_assertions) {
        debug!("This will only be logged in debug builds");
    }
}

// Example module with its own logging
mod my_module {
    use log::{info, error};

    pub fn do_something() {
        info!("Starting operation in my_module");
        if let Err(e) = operation() {
            error!("Operation failed: {}", e);
        }
    }

    fn operation() -> Result<(), String> {
        // Some operation that might fail
        Ok(())
    }
}

// Tests can use logging too
#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    fn test_with_logging() {
        // Initialize logger for tests
        let _ = env_logger::builder()
            .is_test(true)
            .try_init();

        debug!("Starting test");
        assert_eq!(2 + 2, 4);
    }
}
```

Configure logging in different files:
```bash
# .env file
RUST_LOG=debug,hyper=info,reqwest=warn

# or in config.toml
[logger]
level = "debug"
format = "json"
output = "stdout"
```

Common patterns:
1. Use different log levels appropriately:
   - ERROR: Something failed
   - WARN: Something might fail
   - INFO: Important events
   - DEBUG: Detailed information
   - TRACE: Very detailed debugging

2. Add context to logs:
```rust
info!("User {} performed {}", user_id, action);
debug!(
    target: "performance",
    "Request processed in {}ms", 
    duration.as_millis()
);
```

3. Use logging macros that evaluate lazily:
```rust
debug!("Expensive computation: {}", expensive_function());
// The expensive_function is only called if debug logging is enabled
```

## Would you like me to explain more about structured logging or how to set up logging for specific use cases like web services or async applications?

Here's an example showing structured logging, especially useful for web services and async applications:

```rust
use serde_json::json;
use tokio;
use tracing::{info, error, Level, span, instrument};
use tracing_subscriber::{fmt, EnvFilter};

// First, add these to Cargo.toml:
/*
[dependencies]
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
*/

#[derive(Debug)]
struct User {
    id: String,
    name: String,
}

// Setup structured logging
fn setup_logging() {
    // Create file appender
    let file_appender = tracing_appender::rolling::RollingFileAppender::new(
        tracing_appender::rolling::RollingFileAppender::builder()
            .rotation(tracing_appender::rolling::Rotation::DAILY)
            .filename_prefix("app")
            .filename_suffix("log")
            .build()
            .unwrap()
    );

    // Setup subscriber with JSON formatting
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_target(true)
        .json()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_level(true)
        .with_timer(fmt::time::ChronoUtc::rfc3339())
        .build();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");
}

// Using spans for request tracking
#[instrument(level = "info", skip(user_data))]
async fn handle_request(request_id: String, user_data: User) -> Result<(), String> {
    let span = span!(Level::INFO, "request", %request_id);
    let _enter = span.enter();

    info!(
        user_id = %user_data.id,
        user_name = %user_data.name,
        "Processing request"
    );

    // Simulate some work
    process_user_data(&user_data).await?;

    info!("Request completed successfully");
    Ok(())
}

#[instrument]
async fn process_user_data(user: &User) -> Result<(), String> {
    // Simulate database operation
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    info!(
        event = "data_processed",
        user_id = %user.id,
        "User data processed successfully"
    );
    
    Ok(())
}

// Example middleware for web framework
#[instrument(skip(next))]
async fn logging_middleware<F, Fut, Res>(
    request_id: String,
    path: String,
    next: F,
) -> Result<Res, String>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<Res, String>>,
{
    let start = std::time::Instant::now();

    info!(
        event = "request_started",
        path = %path,
        request_id = %request_id,
    );

    let result = next().await;

    let duration = start.elapsed();

    match &result {
        Ok(_) => {
            info!(
                event = "request_completed",
                path = %path,
                request_id = %request_id,
                duration_ms = %duration.as_millis(),
            );
        }
        Err(e) => {
            error!(
                event = "request_failed",
                path = %path,
                request_id = %request_id,
                duration_ms = %duration.as_millis(),
                error = %e,
            );
        }
    }

    result
}

#[tokio::main]
async fn main() {
    setup_logging();

    let user = User {
        id: "123".to_string(),
        name: "Alice".to_string(),
    };

    // Simulate web request
    let request_id = uuid::Uuid::new_v4().to_string();

    // Use middleware
    let result = logging_middleware(
        request_id.clone(),
        "/api/users".to_string(),
        || async {
            handle_request(request_id.clone(), user).await
        },
    ).await;

    match result {
        Ok(_) => info!("Main process completed successfully"),
        Err(e) => error!(error = %e, "Main process failed"),
    }
}
```

This code demonstrates:
1. Structured JSON logging
2. Request tracking with spans
3. Async operation logging
4. Middleware logging pattern
5. File rotation
6. Context preservation across async boundaries

The logs would look like:
```json
{
  "timestamp": "2024-11-08T12:00:00Z",
  "level": "INFO",
  "target": "app",
  "span": {
    "request_id": "123e4567-e89b-12d3-a456-426614174000"
  },
  "fields": {
    "path": "/api/users",
    "user_id": "123",
    "duration_ms": 105
  },
  "event": "request_completed"
}
```

## Would you like to see more about error handling patterns, log aggregation, or monitoring integration?

Here's an example combining error handling, log aggregation, and monitoring integration using popular crates:

```rust
use anyhow::{Context, Result};
use metrics::{counter, gauge, histogram};
use opentelemetry::{global, sdk::export::metrics::aggregation};
use sentry;
use thiserror::Error;
use tracing::{error, info, instrument, warn, Level};

// Add to Cargo.toml:
/*
[dependencies]
anyhow = "1.0"
thiserror = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-opentelemetry = "0.22"
opentelemetry = { version = "0.21", features = ["metrics"] }
opentelemetry-prometheus = "0.14"
metrics = "0.21"
sentry = "0.31"
prometheus = "0.13"
*/

// Custom error types
#[derive(Error, Debug)]
enum AppError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
}

// Setup monitoring and logging
fn setup_monitoring() -> Result<()> {
    // Initialize Sentry for error tracking
    let _guard = sentry::init(("YOUR_SENTRY_DSN", sentry::ClientOptions {
        release: sentry::release_name!(),
        ..Default::default()
    }));

    // Setup OpenTelemetry metrics
    let prometheus_exporter = opentelemetry_prometheus::exporter()
        .with_aggregation_selector(aggregation::cumulative_temporality_selector())
        .init();

    // Setup metrics
    metrics::describe_counter!(
        "api_requests_total",
        "Total number of API requests"
    );
    metrics::describe_gauge!(
        "active_connections",
        "Number of active connections"
    );
    metrics::describe_histogram!(
        "request_duration_seconds",
        "Request duration in seconds"
    );

    Ok(())
}

// Middleware for tracking metrics
#[instrument(level = "info", skip(handler))]
async fn metrics_middleware<F, Fut, T>(
    path: &str,
    handler: F,
) -> Result<T, AppError>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, AppError>>,
{
    let start = std::time::Instant::now();
    
    // Increment request counter
    counter!("api_requests_total", 1, "path" => path.to_string());
    
    // Update active connections
    gauge!("active_connections", 1.0, "path" => path.to_string());
    
    let result = handler().await;
    
    // Record request duration
    let duration = start.elapsed().as_secs_f64();
    histogram!("request_duration_seconds", duration, "path" => path.to_string());
    
    // Update active connections
    gauge!("active_connections", -1.0, "path" => path.to_string());
    
    match &result {
        Ok(_) => {
            info!(
                path = %path,
                duration_ms = %duration,
                "Request completed successfully"
            );
        }
        Err(e) => {
            // Log error and send to Sentry
            error!(
                path = %path,
                duration_ms = %duration,
                error = %e,
                "Request failed"
            );
            
            sentry::capture_error(&e);
        }
    }
    
    result
}

// Example database operation with error handling
#[instrument(level = "debug", err)]
async fn db_operation(id: &str) -> Result<String, AppError> {
    // Simulate database operation
    if id == "error" {
        error!(id = %id, "Database operation failed");
        return Err(AppError::Database("Record not found".into()));
    }
    
    Ok("data".into())
}

// Rate limiting with metrics
struct RateLimiter {
    counter: std::sync::atomic::AtomicUsize,
}

impl RateLimiter {
    #[instrument(level = "debug")]
    fn check_rate_limit(&self) -> Result<(), AppError> {
        let count = self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        if count > 100 {
            warn!("Rate limit exceeded");
            gauge!("rate_limit_exceeded", 1.0);
            return Err(AppError::RateLimit("Too many requests".into()));
        }
        
        gauge!("current_request_count", count as f64);
        Ok(())
    }
}

// Example API endpoint
#[instrument(level = "info", skip(rate_limiter))]
async fn handle_request(
    id: String,
    rate_limiter: &RateLimiter,
) -> Result<String, AppError> {
    // Check rate limit
    rate_limiter.check_rate_limit()?;
    
    // Perform database operation
    let data = db_operation(&id)
        .await
        .context("Failed to fetch data")
        .map_err(|e| AppError::Database(e.to_string()))?;
    
    Ok(data)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup monitoring
    setup_monitoring()?;
    
    let rate_limiter = RateLimiter {
        counter: std::sync::atomic::AtomicUsize::new(0),
    };

    // Example request handling
    let result = metrics_middleware("/api/data", || async {
        handle_request("123".into(), &rate_limiter).await
    }).await;

    match result {
        Ok(data) => info!("Request successful: {}", data),
        Err(e) => error!("Request failed: {}", e),
    }

    // Expose metrics endpoint for Prometheus
    use prometheus::{Encoder, TextEncoder};
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    println!("Metrics:\n{}", String::from_utf8(buffer).unwrap());

    Ok(())
}
```

Key features demonstrated:
1. Custom error types with `thiserror`
2. Error context with `anyhow`
3. Structured logging with `tracing`
4. Metrics collection with `metrics`
5. Error tracking with Sentry
6. Prometheus metrics exposition
7. OpenTelemetry integration
8. Rate limiting with metrics
9. Async operation tracking
10. Error propagation patterns

This could be extended with:
1. Grafana dashboard configuration
2. ELK stack integration
3. Custom metric aggregations
4. Alert configurations
5. Distributed tracing

## Logging to files. Let me create a practical example showing some common logging approaches.

The Rust `log` crate itself does not have the capability to write logs to a file. It’s a lightweight, logging **interface** crate that provides macros like `info!`, `warn!`, and `error!`, but it does not handle the actual logging output. Instead, `log` relies on external logging backends (called *loggers*) to handle where and how logs are recorded.

To write logs to a file, you would typically pair `log` with a compatible logging backend crate, such as:

1. **env_logger**: A popular logging backend that outputs logs to stdout or stderr, but it does not support file logging directly. However, it’s useful for quick setups, especially in development.

2. **flexi_logger**: A more flexible logger that allows output to both files and the console. It has options for file rotation, custom log formats, and more.
   ```toml
   [dependencies]
   log = "0.4"
   flexi_logger = "0.22"
   ```
   Example setup:
   ```rust
   use flexi_logger::{Logger, WriteMode};

   fn main() {
       Logger::try_with_str("info")
           .unwrap()
           .log_to_file()
           .write_mode(WriteMode::BufferAndFlush)
           .start()
           .unwrap();

       log::info!("This log entry will go to a file!");
   }
   ```

3. **simplelog**: Another logging backend that supports writing to files. It offers several loggers like `SimpleLogger`, `WriteLogger`, and `CombinedLogger` for different use cases.
   ```toml
   [dependencies]
   log = "0.4"
   simplelog = "0.12"
   ```
   Example setup:
   ```rust
   use simplelog::*;
   use std::fs::File;

   fn main() {
       WriteLogger::init(LevelFilter::Info, Config::default(), File::create("app.log").unwrap()).unwrap();
       log::info!("This log entry will go to a file!");
   }
   ```

By using one of these backends with `log`, you can set up file logging with minimal code adjustments.
