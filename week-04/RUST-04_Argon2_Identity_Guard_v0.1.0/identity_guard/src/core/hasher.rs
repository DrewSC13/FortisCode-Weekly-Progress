use anyhow::{Result, anyhow};
use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use rand::rngs::OsRng;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct SecurityParams {
    pub memory_mib: u32,
    pub iterations: u32,
    pub parallelism: u32,
}

impl SecurityParams {
    pub fn sanitize(self) -> Result<Self> {
        if self.memory_mib < 64 {
            return Err(anyhow!("memory_mib must be >= 64"));
        }
        if self.iterations < 3 {
            return Err(anyhow!("iterations must be >= 3"));
        }
        if self.parallelism < 1 {
            return Err(anyhow!("parallelism must be >= 1"));
        }
        Ok(self)
    }
}

/// Env:
/// IG_MEMORY_MIB (>=64) default 64
/// IG_ITERATIONS (>=3) default 3
/// IG_PARALLELISM (>=1) default available_parallelism()
pub fn params_from_env() -> SecurityParams {
    let default_parallelism = std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(1);

    let memory_mib = std::env::var("IG_MEMORY_MIB")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(64);

    let iterations = std::env::var("IG_ITERATIONS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(3);

    let parallelism = std::env::var("IG_PARALLELISM")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(default_parallelism);

    SecurityParams {
        memory_mib: memory_mib.max(64),
        iterations: iterations.max(3),
        parallelism: parallelism.max(1),
    }
}

fn build_argon2(p: SecurityParams) -> Result<Argon2<'static>> {
    let p = p.sanitize()?;

    // Argon2 Params usa KiB
    let mem_kib = p
        .memory_mib
        .checked_mul(1024)
        .ok_or_else(|| anyhow!("memory_mib overflow"))?;

    let params = Params::new(mem_kib, p.iterations, p.parallelism, None)
        .map_err(|e| anyhow!("argon2 params invalid: {e}"))?;

    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

pub fn hash_password(password: &str) -> Result<String> {
    let params = params_from_env();
    let argon2 = build_argon2(params)?;

    // Salt seguro por usuario
    let salt = SaltString::generate(&mut OsRng);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow!("hash_password failed: {e}"))?
        .to_string();

    Ok(hash)
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(v) => v,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Benchmark + tuning:
/// si <100ms, sube memoria progresivamente hasta >=100ms o IG_MAX_MEMORY_MIB (default 1024)
pub fn benchmark_and_tune(mut params: SecurityParams) -> Result<(SecurityParams, Duration)> {
    params = params.sanitize()?;

    let max_mib = std::env::var("IG_MAX_MEMORY_MIB")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1024)
        .max(64);

    let steps: [u32; 7] = [64, 96, 128, 192, 256, 384, 512];

    let mut mem = params.memory_mib;
    let probe_password = "benchmark-probe-password";

    loop {
        let try_params = SecurityParams {
            memory_mib: mem.min(max_mib),
            ..params
        };

        let argon2 = build_argon2(try_params)?;
        let salt = SaltString::generate(&mut OsRng);

        let start = Instant::now();
        let _ = argon2
            .hash_password(probe_password.as_bytes(), &salt)
            .map_err(|e| anyhow!("benchmark hash failed: {e}"))?;
        let dur = start.elapsed();

        if dur >= Duration::from_millis(100) {
            return Ok((try_params, dur));
        }

        if mem >= max_mib {
            return Ok((try_params, dur));
        }

        let mut next = mem + 64;
        for s in steps {
            if s > mem {
                next = s;
                break;
            }
        }
        mem = next.min(max_mib);
    }
}
