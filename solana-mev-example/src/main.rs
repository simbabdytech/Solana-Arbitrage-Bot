use anyhow::Result;
use clap::Parser;
use std::time::Duration;

// 引入模块
mod sandwich;
mod arbitrage;

/// Solana MEV示例 - 监控不同策略的 MEV 机会
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Solana RPC节点URL
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,

    /// 套利目标差价百分比
    #[arg(short, long, default_value_t = 0.5)]
    target_spread: f64,

    /// 监控间隔（毫秒）
    #[arg(short, long, default_value_t = 3000)]
    interval_ms: u64,

    /// MEV策略类型 (arbitrage 或 sandwich)
    #[arg(short, long, default_value = "arbitrage")]
    strategy: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("=== Solana MEV 演示程序 ===");
    println!("RPC URL: {}", args.rpc_url);
    
    match args.strategy.as_str() {
        "arbitrage" => {
            println!("执行套利 MEV 策略演示");
            
            // 调用套利模块的函数
            arbitrage::demonstrate_arbitrage(&args.rpc_url, args.target_spread, args.interval_ms).await?;
        },
        "sandwich" => {
            println!("执行夹子攻击 MEV 策略演示");
            
            // 调用夹子攻击示例
            sandwich::demonstrate_sandwich_attack()?;
        },
        _ => {
            println!("不支持的策略: {}。请使用 'arbitrage' 或 'sandwich'", args.strategy);
        }
    }
    
    Ok(())
} 