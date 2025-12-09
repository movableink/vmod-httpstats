use std::sync::atomic::AtomicU64;

use varnish::{Vsc, VscMetric};

/// HTTP response code statistics for backend responses
#[derive(VscMetric)]
#[repr(C)]
pub struct BackendStats {
    /// Backend 2xx responses (successful)
    #[counter]
    resp_2xx: AtomicU64,

    /// Backend 3xx responses (redirects)
    #[counter]
    resp_3xx: AtomicU64,

    /// Backend 4xx responses (client errors)
    #[counter]
    resp_4xx: AtomicU64,

    /// Backend 5xx responses (server errors)
    #[counter]
    resp_5xx: AtomicU64,
}

/// HTTP response code statistics for frontend (client-facing) responses
#[derive(VscMetric)]
#[repr(C)]
pub struct FrontendStats {
    /// Frontend 2xx responses (successful)
    #[counter]
    resp_2xx: AtomicU64,

    /// Frontend 3xx responses (redirects)
    #[counter]
    resp_3xx: AtomicU64,

    /// Frontend 4xx responses (client errors)
    #[counter]
    resp_4xx: AtomicU64,

    /// Frontend 5xx responses (server errors)
    #[counter]
    resp_5xx: AtomicU64,
}

#[allow(non_camel_case_types)]
pub struct stats {
    backend: Vsc<BackendStats>,
    frontend: Vsc<FrontendStats>,
}

#[varnish::vmod(docs = "API.md")]
mod httpstats {
    use std::sync::atomic::Ordering::Relaxed;

    use varnish::Vsc;

    use super::{stats, BackendStats, FrontendStats};

    impl stats {
        /// Create a new stats instance for tracking HTTP response codes
        #[allow(clippy::new_without_default)]
        pub fn new() -> Self {
            let name = "default";
            let backend = Vsc::<BackendStats>::new("httpstats.backend", name);
            let frontend = Vsc::<FrontendStats>::new("httpstats.frontend", name);
            Self { backend, frontend }
        }

        /// Record a backend response by status code
        pub fn record_backend(&self, status: i64) {
            match status {
                200..=299 => self.backend.resp_2xx.fetch_add(1, Relaxed),
                300..=399 => self.backend.resp_3xx.fetch_add(1, Relaxed),
                400..=499 => self.backend.resp_4xx.fetch_add(1, Relaxed),
                500..=599 => self.backend.resp_5xx.fetch_add(1, Relaxed),
                _ => 0,
            };
        }

        /// Record a frontend response by status code
        pub fn record_frontend(&self, status: i64) {
            match status {
                200..=299 => self.frontend.resp_2xx.fetch_add(1, Relaxed),
                300..=399 => self.frontend.resp_3xx.fetch_add(1, Relaxed),
                400..=499 => self.frontend.resp_4xx.fetch_add(1, Relaxed),
                500..=599 => self.frontend.resp_5xx.fetch_add(1, Relaxed),
                _ => 0,
            };
        }

        /// Get the count of backend 2xx responses
        pub fn backend_2xx(&self) -> i64 {
            self.backend.resp_2xx.load(Relaxed) as i64
        }

        /// Get the count of backend 3xx responses
        pub fn backend_3xx(&self) -> i64 {
            self.backend.resp_3xx.load(Relaxed) as i64
        }

        /// Get the count of backend 4xx responses
        pub fn backend_4xx(&self) -> i64 {
            self.backend.resp_4xx.load(Relaxed) as i64
        }

        /// Get the count of backend 5xx responses
        pub fn backend_5xx(&self) -> i64 {
            self.backend.resp_5xx.load(Relaxed) as i64
        }

        /// Get the count of frontend 2xx responses
        pub fn frontend_2xx(&self) -> i64 {
            self.frontend.resp_2xx.load(Relaxed) as i64
        }

        /// Get the count of frontend 3xx responses
        pub fn frontend_3xx(&self) -> i64 {
            self.frontend.resp_3xx.load(Relaxed) as i64
        }

        /// Get the count of frontend 4xx responses
        pub fn frontend_4xx(&self) -> i64 {
            self.frontend.resp_4xx.load(Relaxed) as i64
        }

        /// Get the count of frontend 5xx responses
        pub fn frontend_5xx(&self) -> i64 {
            self.frontend.resp_5xx.load(Relaxed) as i64
        }
    }
}

#[cfg(test)]
mod tests {
    varnish::run_vtc_tests!("tests/*.vtc");
}
