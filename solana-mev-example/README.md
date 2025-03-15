# Solana MEV 示例

本项目是一个简单的 Rust 演示程序，用于展示 Solana 区块链上的 MEV (Maximal Extractable Value，最大可提取价值) 概念。

## 什么是 MEV？

MEV (最大可提取价值) 是指区块生产者或网络用户可以通过重新排序、插入或审查交易来获取的额外价值。虽然 MEV 最初在以太坊生态系统中被广泛讨论，但它同样存在于 Solana 等其他区块链中。

在 Solana 上，MEV 机会主要表现为：

1. **套利**: 利用不同 DEX 之间的价格差异进行交易获利
2. **前台运行（Frontrunning）**: 监控内存池中的交易，抢先执行类似交易
3. **夹子攻击（Sandwich Attack）**: 在大额交易前后插入自己的交易，操纵价格获利
4. **清算套利**: 抢先执行借贷协议中的清算操作获得奖励

## Solana MEV 的特点

Solana 区块链的 MEV 与以太坊等区块链的 MEV 有一些不同：

- **更快的出块时间**: Solana 的出块时间极短 (约400ms)，MEV 策略执行时间窗口更小
- **代写者/领导者排序**: Solana 的区块生产者有权决定交易顺序
- **优先级费用**: Solana 引入了优先级费用机制，允许用户支付额外费用提高交易优先级
- **原子交易**: Solana 支持多个指令在一个交易中原子执行

## 本示例包含的 MEV 策略

本示例程序实现了一个简单的套利机器人，展示了 Solana MEV 的基本概念：

1. 监控不同 DEX 之间的价格差异
2. 当发现套利机会时，模拟执行套利交易
3. 展示 MEV 相关的概念，如前台运行、交易打包和优先级费用

## 如何运行

```bash
# 编译项目
cargo build --release

# 运行示例 (使用默认设置)
cargo run --release

# 使用自定义参数运行
cargo run --release -- --rpc-url https://your-rpc-url.com --target-spread 1.0 --interval-ms 5000
```

## 参数说明

- `--rpc-url`: Solana RPC 节点 URL
- `--target-spread`: 套利目标差价百分比 (默认 0.5%)
- `--interval-ms`: 监控间隔（毫秒，默认 3000ms）

## 免责声明

本项目仅用于教育目的，展示 MEV 的概念。在实际环境中部署 MEV 策略需要更加复杂的实现和风险管理。 