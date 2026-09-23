# Agent开发指南 / AI Agent Development Guide

本文档为自主AI Agent提供KerGameAIEngine的开发约束与指导。

## 核心原则

1. **AI-First设计**: 此项目为AI自主开发优化，人类不参与中间测试
2. **自验证**: Agent必须完全自测所有功能
3. **确定性优先**: 所有随机行为必须可seed控制
4. **短反馈循环**: Demo设计为5–15秒，快速验证

## Agent工作流

### 每个里程碑的标准流程

1. **理解需求**: 阅读ROADMAP.md对应里程碑定义
2. **设计接口**: 先定义公共API和Command类型
3. **实现核心**: 编写功能代码
4. **编写Demo**: 创建对应MX Demo
5. **自测验证**: 
   - `cargo build --release`
   - `cargo test`
   - `cargo run --bin engine -- run-demo MX --headless --seed 42 --report test.json`
   - 检查report.json中`success: true`
6. **生成报告**: 记录以下内容
   - 实现的功能列表
   - Demo运行结果 (tick_count, elapsed_ms, replay_hash)
   - 性能指标 (fps, 内存使用)
   - 下一里程碑建议

### 代码规范

- **语言**: Rust 2021 edition
- **注释**: 中文或英文均可，关键算法必须注释
- **测试**: 每个模块至少1个单元测试
- **错误处理**: 使用`anyhow::Result`，不要unwrap生产代码
- **性能**: 避免不必要的clone，优先使用引用

### Demo设计规范

每个Demo必须：
- 实现`Demo` trait
- 注册到`DemoRegistry`
- 运行时长控制在5–15秒 (典型600–900帧@60TPS)
- 内部assert关键不变量
- 成功时返回`Ok(())`，失败时返回`Err`

示例：
```rust
pub struct M2Demo;

impl Demo for M2Demo {
    fn id(&self) -> &str { "M2" }
    
    fn description(&self) -> &str {
        "ECS stress test: 10k entities, 600 ticks"
    }
    
    fn run(&self, engine: &mut Engine) -> Result<()> {
        // Spawn 10k entities
        for i in 0..10_000 {
            engine.spawn_entity(/* ... */);
        }
        
        // Run 600 ticks (10 seconds)
        for _ in 0..600 {
            engine.tick()?;
        }
        
        // Assert invariants
        assert_eq!(engine.entity_count(), 10_000);
        
        Ok(())
    }
}
```

### 确定性验证

关键原则：
- 所有RNG使用`rand_chacha::ChaCha8Rng::seed_from_u64(seed)`
- 所有游戏逻辑通过Command执行
- ReplayHasher记录每个Command
- 相同seed必须产生相同replay_hash

验证方法：
```bash
# 运行两次相同seed
cargo run --bin engine -- run-demo M2 --headless --seed 42 --report run1.json
cargo run --bin engine -- run-demo M2 --headless --seed 42 --report run2.json

# 对比hash
diff <(jq .replay_hash run1.json) <(jq .replay_hash run2.json)
# 应该无输出 (相同)
```

### 性能基准

目标：
- **模拟**: 60 TPS (16.67ms/tick)
- **渲染**: 60 FPS (16.67ms/frame，M3+)
- **实体数**: 10k+ entities with basic components
- **像素数**: 512×512 active pixels (M4+)

如果性能不达标：
1. 使用`cargo flamegraph`分析热点
2. 优化算法复杂度
3. 考虑并行化 (rayon)
4. 记录到报告中，下一里程碑处理

### 常见陷阱

1. **不要假设显示环境**: 所有Demo必须支持--headless
2. **不要依赖时钟**: 使用固定timestep，不要读取系统时间
3. **不要硬编码路径**: 使用相对路径或CLI参数
4. **不要忽略错误**: 所有Result必须处理
5. **不要过度优化**: 先跑通，再优化

### Git提交规范

- **提交频率**: 每完成一个子功能就提交
- **消息格式**: `[MX] 功能简述`
  - 例: `[M2] Implement ECS entity spawning`
  - 例: `[M2] Add M2Demo with 10k entities stress test`
- **原子提交**: 每个commit保持编译通过

### 里程碑完成检查清单

- [ ] 功能代码实现
- [ ] 单元测试通过
- [ ] Demo实现并注册
- [ ] Demo headless运行成功
- [ ] report.json显示success: true
- [ ] 性能符合基准或记录差距
- [ ] 代码有必要注释
- [ ] Git提交历史清晰
- [ ] ROADMAP.md对应里程碑标记✓
- [ ] 生成最终报告给人类审查

## 自主决策权限

Agent被授权：
- 选择合适的Rust库 (在workspace.dependencies中声明)
- 设计内部数据结构
- 调整Demo参数 (tick数，实体数) 在合理范围内
- 重构代码以提高可维护性
- 修复发现的bug

Agent需要请示人类的情况：
- 偏离ROADMAP.md的重大架构变更
- 引入新的外部依赖 (非Cargo生态)
- 性能无法达标且无明显优化方向
- Demo失败无法自行修复

## 中文支持

- 代码中可使用中文注释
- ROADMAP.md/AGENTS.md允许中英混合
- Commit message推荐英文，但中文可接受
- 变量名/函数名必须英文

## 反馈循环

每完成一个里程碑，Agent应生成报告：
```markdown
# MX 里程碑完成报告

## 实现功能
- 功能1: 描述
- 功能2: 描述

## Demo结果
```json
{
  "demo_id": "MX",
  "success": true,
  "tick_count": 600,
  ...
}
```

## 性能指标
- 平均tick时间: X ms
- 峰值内存: X MB

## 技术债务
- 待优化: ...
- 已知限制: ...

## 下一步建议
- M(X+1)关键依赖: ...
```

---

**记住**: 你的目标是完全自主完成开发，无需人类中间干预。设计代码时优先考虑可测试性和确定性。
