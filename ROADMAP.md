# KerGameAIEngine 路线图 / Roadmap

**Fantasy定位**: Terraria × Noita (4px像素) × Diablo

## 核心架构原则 / Core Architecture Principles

1. **Rust自研引擎 + wgpu** (不使用Bevy作为引擎)
2. **固定时间步 (Fixed Timestep)**: 模拟与渲染解耦
3. **确定性设计**: Seeded RNG + 可序列化Command层
4. **回放验证**: 所有游戏行为可重放，带哈希校验
5. **无头运行**: Demo可在无显示环境运行（CI友好）
6. **短压测Demo**: 典型5–15秒，验证功能正确性，不做长期压力测试

## CLI接口规范

```bash
engine run-demo <ID> --headless --seed <N> --report <PATH.json>
```

所有Demo必须支持：
- `--headless`: 无窗口运行
- `--seed N`: 确定性随机种子
- `--report PATH`: 输出JSON格式测试报告

报告格式：
```json
{
  "demo_id": "M0",
  "seed": 42,
  "success": true,
  "tick_count": 600,
  "elapsed_real_ms": 150,
  "elapsed_sim_ms": 10000,
  "replay_hash": "abc123...",
  "message": "Demo completed successfully"
}
```

## 里程碑详细设计 / Milestone Breakdown

### M0: 脚手架 + CLI + 无头测试框架 ✓ COMPLETE (2026-09-23)
**目标**: 验证基础设施可运行
- Rust workspace结构
- CLI: `engine run-demo M0 --headless --seed 42 --report out.json`
- M0 Demo: 空转600帧(10秒@60TPS)，生成通过报告
- ROADMAP.md, AGENTS.md, README.md就位

**验收**: `cargo build`成功，M0 Demo生成passing报告

---

### M1: 时间循环 + 命令层 + 回放哈希 ✓ COMPLETE (2026-09-23)
**目标**: 建立确定性Command架构
- 固定时间步循环 (60 TPS)
- Command枚举 (所有游戏动作)
- CommandBuffer每帧收集
- ReplayHasher: SHA256哈希所有Command序列
- M1 Demo: 模拟1000个Command操作，验证相同seed产生相同hash

**验收**: 
- 两次运行相同seed，hash一致
- 不同seed，hash不同

---

### M2: ECS压力测试 ✓ COMPLETE (2026-09-23)
**目标**: 验证ECS性能
- 轻量级自研ECS (或使用hecs/bevy_ecs库)
- 组件: Transform, Velocity, Health
- 系统: Movement, Collision
- M2 Demo: 生成10,000实体，运行600帧(10秒)，测量帧时间

**验收**: 
- 平均帧时间 < 2ms (60fps目标)
- 报告包含fps统计

---

### M3: 渲染管线 ✓ COMPLETE (2026-09-23)
**目标**: wgpu渲染基础
- wgpu初始化 (支持headless: texture target)
- 基础sprite渲染 (instanced quad batching)
- 相机系统
- M3 Demo: 渲染1000个sprite，10秒，生成截图保存到报告

**验收**: 
- Headless模式生成有效PNG截图
- 渲染性能 > 60fps

---

### M4: 像素/区块世界 ✓ COMPLETE (2026-09-23)
**目标**: Noita风格像素世界

**🔒 LOCKED DECISION: 4-pixel cells**
- Cell size: 4×4 screen pixels (NOT 1px)
- Rationale: Performance (fewer cells), visual clarity, Terraria-scale feel
- Each cell = single material (Air/Sand/Stone/Water...)

**实现**:
- Material枚举 (Air, Sand, Stone, Water, 可扩展)
- Chunk系统 (128×128 cells per chunk = 512×512 screen pixels)
- 稀疏chunk存储 (按需加载/卸载)
- Seeded Perlin噪声地形生成
- Query/Set cell APIs (get_cell, set_cell, dig, place)
- M4 Demo: 流式加载chunk，移动焦点，10秒

**验收**: 
- 世界生成确定性 (相同seed相同地形)
- Chunk稀疏加载正确
- 截图展示地形

---

### M5: 挖掘/建造 + 角色运动 ✓ COMPLETE (2026-09-23)
**目标**: 玩家交互基础
- 角色CharacterMotor: AABB碰撞，爬梯，跳跃
- DigBuildSystem: 鼠标点击改变像素Material
- M5 Demo: AI角色挖掘隧道，移动100格，10秒

**验收**: 
- 截图显示挖掘路径
- 角色碰撞正确

---

### M6: 沙/流体/反应 ✓ COMPLETE (2026-09-23)
**目标**: Noita风格物理
- 落沙物理 (cellular automata)
- 流体模拟 (水流扩散)
- 材料反应 (水+熔岩=石头)
- M6 Demo: 倒100格水，观察流动，10秒

**验收**: 
- 流体行为自然
- 反应触发正确
- 性能 > 60fps

---

### M7: 光照系统 ✓ COMPLETE (2026-09-23)
**目标**: 动态光照
- Tile-based光传播
- 点光源 (火把，法术)
- 光照渲染 (light map overlay)
- M7 Demo: 50个移动点光源，10秒

**验收**: 
- 光照效果截图
- 性能 > 60fps

---

### M8: 物品系统 ✓ COMPLETE (2026-09-23)
**目标**: Diablo风格物品
- Item定义 (武器，装备，消耗品)
- Inventory系统
- 掉落/拾取
- M8 Demo: 生成100个随机物品，AI拾取，10秒

**验收**: 
- 物品序列化正确
- 背包容量限制生效

---

### M9: 战斗/掉落
**目标**: Diablo风格战斗
- Combat系统 (伤害计算，暴击，元素伤害)
- 敌人AI (追逐，攻击)
- 掉落表 (稀有度权重)
- M9 Demo: AI角色击杀50个敌人，收集掉落，10秒

**验收**: 
- 战斗伤害log正确
- 掉落符合稀有度分布

---

### M10: 魔法组合
**目标**: Noita风格法术编辑
- Spell组件系统 (投射物+修饰符+触发器)
- SpellCaster
- M10 Demo: AI释放20种组合法术，10秒

**验收**: 
- 法术效果截图
- 组合逻辑正确

---

### M11: NPC系统
**目标**: 任务/对话
- NPC定义
- 对话树
- 简单任务系统
- M11 Demo: AI与5个NPC对话，接受任务，10秒

**验收**: 
- 对话log正确
- 任务状态追踪

---

### M12: 剧情/事件
**目标**: 动态叙事
- 事件触发器
- 剧情节点
- Boss战触发
- M12 Demo: 触发Boss战，AI击败Boss，10秒

**验收**: 
- Boss战log
- 剧情状态记录

---

### M13: 音频/UI
**目标**: 完善交互
- 音效播放 (rodio)
- UI框架 (egui)
- HUD显示
- M13 Demo: UI显示血条/背包，10秒

**验收**: 
- UI截图
- 音效log (headless静音)

---

### M14: 存档/性能优化/发布
**目标**: 生产就绪
- 存档系统 (世界+玩家状态序列化)
- 性能profiling (tracy)
- 打包脚本
- M14 Demo: 保存/加载完整游戏状态，10秒

**验收**: 
- 存档往返一致
- 发布包可运行

---

## Demo设计原则

1. **短而精**: 5–15秒典型运行时长
2. **自验证**: Demo内部断言关键不变量
3. **确定性**: 相同seed必须产生相同结果
4. **报告驱动**: 所有验证指标输出到JSON
5. **AI自测**: 人类不手动测试中间产物，AI全自动验证

## 技术债务管理

- 每个里程碑完成后，Agent需自检：
  - 是否有TODO未处理？
  - 是否有硬编码magic number？
  - 是否有性能瓶颈？
- 重构在M14前完成

## 成功标准

每个里程碑：
1. `cargo build --release` 通过
2. `cargo test` 全绿
3. Demo headless运行生成passing报告
4. 代码有基础注释 (中文或英文均可)

最终M14：
- 完整游戏可玩 (windowed模式)
- 所有Demo通过
- README/ROADMAP/AGENTS文档齐全
