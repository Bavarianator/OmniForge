//! Hardware abstraction: GPU vendor, VRAM, and conservative training presets.

use omniforge_common::error::OmniForgeError;
use omniforge_common::types::GpuInfo;
use std::process::Command;
use tracing::{info, warn};

/// Snapshot of hardware capabilities used for automatic quantization and batch sizing.
#[derive(Debug, Clone)]
pub struct HardwareProfile {
    /// Primary GPU metadata when detection succeeds.
    pub gpu: GpuInfo,
    /// System RAM in gigabytes (rounded up for UI copy).
    pub ram_gb: u32,
    /// Recommended `QuantizationMode` name for the UI (string to avoid coupling).
    pub recommended_quantization: &'static str,
}

/// Probes the local machine and returns a [`HardwareProfile`].
///
/// This is intentionally heuristic: it prefers `nvidia-smi` and `rocm-smi` when present
/// and falls back to CPU-only guidance without failing hard.
pub async fn detect_hardware() -> Result<HardwareProfile, OmniForgeError> {
    let ram_gb = detect_ram_gb().unwrap_or(16);

    if let Some(gpu) = detect_nvidia().await {
        info!(vendor = %gpu.vendor, model = ?gpu.model, vram_mb = ?gpu.vram_mb, "detected NVIDIA GPU");
        return Ok(HardwareProfile {
            recommended_quantization: recommend_quant_for_vram(gpu.vram_mb),
            ram_gb,
            gpu,
        });
    }

    if let Some(gpu) = detect_amd().await {
        info!(vendor = %gpu.vendor, model = ?gpu.model, vram_mb = ?gpu.vram_mb, "detected AMD GPU");
        return Ok(HardwareProfile {
            recommended_quantization: recommend_quant_for_vram(gpu.vram_mb),
            ram_gb,
            gpu,
        });
    }

    warn!("no discrete GPU detected; using CPU-oriented defaults");
    Ok(HardwareProfile {
        gpu: GpuInfo {
            vendor: "CPU".to_string(),
            model: None,
            vram_mb: None,
            driver_version: None,
        },
        ram_gb,
        recommended_quantization: "4bit",
    })
}

fn recommend_quant_for_vram(vram_mb: Option<u32>) -> &'static str {
    let Some(mb) = vram_mb else {
        return "4bit";
    };
    match mb {
        0..=4095 => "4bit",
        4096..=8191 => "5bit_or_8bit_lora",
        8192..=16383 => "8bit_or_fp16_lora",
        _ => "fp16",
    }
}

async fn detect_nvidia() -> Option<GpuInfo> {
    let output = tokio::task::spawn_blocking(|| {
        Command::new("nvidia-smi")
            .args([
                "--query-gpu=name,memory.total,driver_version",
                "--format=csv,noheader,nounits",
            ])
            .output()
    })
    .await
    .ok()
    .and_then(|io_res| io_res.ok())?;

    if !output.status.success() {
        return None;
    }

    let line = String::from_utf8_lossy(&output.stdout);
    let first = line.lines().next()?.trim();
    let mut parts = first.split(',').map(str::trim);
    let name = parts.next()?.to_string();
    let mem_mib = parts.next()?.parse::<u32>().ok();
    let driver = parts.next().map(std::string::ToString::to_string);

    Some(GpuInfo {
        vendor: "NVIDIA".to_string(),
        model: Some(name),
        vram_mb: mem_mib,
        driver_version: driver,
    })
}

async fn detect_amd() -> Option<GpuInfo> {
    let output = tokio::task::spawn_blocking(|| Command::new("rocm-smi").arg("--showmeminfo").output())
        .await
        .ok()
        .and_then(|io_res| io_res.ok())?;

    if !output.status.success() {
        return None;
    }

    Some(GpuInfo {
        vendor: "AMD".to_string(),
        model: None,
        vram_mb: None,
        driver_version: None,
    })
}

fn detect_ram_gb() -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        let raw = std::fs::read_to_string("/proc/meminfo").ok()?;
        for line in raw.lines() {
            if let Some(rest) = line.strip_prefix("MemTotal:") {
                let kb: u64 = rest
                    .split_whitespace()
                    .next()?
                    .parse()
                    .ok()?;
                let gb = (kb / 1024 / 1024).max(1) as u32;
                return Some(gb);
            }
        }
    }
    None
}
