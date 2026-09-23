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

### M9: 战斗/掉落 ✓ COMPLETE (2026-09-23)
**目标**: Diablo风格战斗
- Combat系统 (伤害计算，暴击，元素伤害)
- 敌人AI (追逐，攻击)
- 掉落表 (稀有度权重)
- M9 Demo: AI角色击杀50个敌人，收集掉落，10秒

**验收**: 
- 战斗伤害log正确
- 掉落符合稀有度分布

---

### M10: 魔法组合 ✓ COMPLETE (2026-09-23)
**目标**: Noita风格法术编辑
- Spell组件系统 (投射物+修饰符+触发器)
- SpellCaster
- M10 Demo: AI释放20种组合法术，10秒

**验收**: 
- 法术效果截图
- 组合逻辑正确

---

### M11: NPC系统 ✓ COMPLETE (2026-09-23)
**目标**: 任务/对话
- NPC定义
- 对话树
- 简单任务系统
- M11 Demo: AI与5个NPC对话，接受任务，10秒

**验收**: 
- 对话log正确
- 任务状态追踪

---

### M12: 剧情/事件 ✓ COMPLETE (2026-09-23)
**目标**: 动态叙事
- 事件触发器
- 剧情节点
- Boss战触发
- M12 Demo: 触发Boss战，AI击败Boss，10秒

**验收**: 
- Boss战log
- 剧情状态记录

---

### M13: 音频/UI ✓ COMPLETE (2026-09-23)
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

### M15: 2D骨骼动画系统 ✓ COMPLETE (2026-09-23)
**目标**: Action game feel (Terraria×Noita×Diablo)
- 骨骼层级 (parent index, local/world transforms)
- 关键帧动画 (位置/旋转/缩放插值)
- Slot/Attachment (精灵绑定到骨骼)
- 动画状态机 (play/loop, crossfade基础)
- 事件轨道 (hit, can_cancel等战斗钩子)
- 确定性采样 (fixed timestep驱动)
- JSON格式 (docs/skeleton_format.md)
- M15 Demo: Humanoid骨骼 (5 bones), idle/attack clips, 600 ticks

**验收**:
- 层级变换正确计算
- 关键帧插值平滑
- 事件在正确时间触发
- Headless运行成功
- 测试覆盖bind pose, 父子变换, 采样

---

## Demo设计原则

### Milestone Demos (M0-M14)

1. **短而精**: 5–15秒典型运行时长
2. **自验证**: Demo内部断言关键不变量
3. **确定性**: 相同seed必须产生相同结果
4. **报告驱动**: 所有验证指标输出到JSON
5. **AI自测**: 人类不手动测试中间产物，AI全自动验证

### SHOWCASE Demo (Unified Experience)

**SHOWCASE is the primary human experience entrypoint.**

- **Purpose**: Quick (~30-40s) unified tour of ALL engine features
- **Structure**: Short chapters (2-5s each) covering every implemented system
- **Rule**: **When a new feature/milestone ships, it MUST also get a chapter in SHOWCASE**
- **Priority**: Listed first in `list-demos`, tested in all PRs
- **Documentation**: See `SHOWCASE.md` for full guide

SHOWCASE complements milestone demos:
- **SHOWCASE**: Human experience, integration, completeness check
- **M* Demos**: Automated gates, isolated feature validation

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

---

# 🚀 深度开发路线 / Deep Development Phase

**Context**: 已实现M0-M15核心系统 + TERRARIA playable demo。下一阶段从"demo tour"转向"真实游戏世界"。

**Fantasy**: Terraria (exploration) × Noita (pixel physics/magic) × Diablo (loot/combat depth)

## 世界与生成 / World & Generation

### M16: 丰富世界生成 ✓ COMPLETE (2026-09-23)
**目标**: 替换简单噪声地形，实现分层生成器
- 地形特征:
  - 洞穴 (worm算法 + cellular automata)
  - 山脉、悬崖、湖泊
  - 生态区 (biomes): 沙漠、丛林、草地、泥土表层、沼泽
  - 熔岩池、矿脉 (铜/铁/金/魔法矿石)
- 确定性: 相同seed → 相同世界
- M16 Demo: 生成并验证种子42包含所有biome + 洞穴 + 矿石，10–15秒
- SHOWCASE chapter: 展示biome颜色变化 + 洞穴/矿脉

**验收**:
- Headless验证生态区多样性
- Windowed可见不同颜色biome
- 洞穴连通性自然
- 矿脉分布符合稀有度

---

### M17: 村庄/城镇结构生成 ✓ **COMPLETE**
**目标**: 程序生成人类聚落
- 结构模板: 房屋、商店、祭坛、地牢
- 分层放置: 先定位结构点，再雕刻到地形
- 守护NPC: 村民、商人
- M17 Demo: 生成3个村庄 + 1个地下地牢，AI探索，10秒

**实现**:
- ✓ `structures.rs`: 结构模板系统 (House, Shop, Altar, UndergroundDungeon)
- ✓ `StructureGenerator`: 自动寻找地表/地下位置，避免冲突
- ✓ M17 Demo headless: 3+ surface structures + 1+ dungeon, success:true
- ✓ SHOWCASE Chapter 18: Villages展示
- ✓ 确定性: seed 42 → 固定结构布局

**验收**:
- 结构不与地形冲突
- NPC正确生成在建筑内

---

### M18: 生态区扩散与动态
**目标**: 世界活着的感觉 (Terraria腐化/神圣扩散)
- Biome spreading: 腐化/魔法生态区缓慢侵蚀邻近区块
- 天气系统: 雨/雪影响像素 (水积累，雪覆盖)
- 动态事件: 流星坠落、血月、日食
- M18 Demo: 触发血月，观察生态区扩散1格，10秒

**验收**:
- 扩散算法确定性
- 天气效果可见

---

## 角色与动画 / Character & Animation

### M19: 下载人形模型 + 丰富动作集
**目标**: 替换矩形方块，使用真实骨骼角色
- 资产: 下载CC0/清晰许可证人形精灵 (例: Kenney, OpenGameArt)
- 动画集: 行走、奔跑、跳跃、攀爬、攻击、翻滚/冲刺
- M15骨骼系统集成: 加载JSON骨骼定义
- M19 Demo: AI角色执行所有动作，10秒

**验收**:
- 所有动画流畅过渡
- Sprite正确绑定到骨骼

---

### M20: NPC需求效用AI
**目标**: NPC作为世界模拟器 (而非任务机器人)
- 需求: 口渴 → 寻找水源，饥饿 → 狩猎/农作，困倦 → 找床
- 效用函数: 计算行为优先级
- 路径寻找: A*在像素世界
- M20 Demo: 3个NPC自主生存24小时模拟(加速)，10秒

**验收**:
- NPC正确响应需求
- 路径寻找不卡死

---

## 战斗与深度 / Combat & Depth

### M21: 武器系统重制
**目标**: Terraria风格武器多样性
- 武器类型: 近战 (剑/锤/矛), 远程 (弓/枪/魔杖), 魔法 (法杖/书)
- 攻击模式: 挥砍、刺击、抛物线射击、蓄力
- 击退 (knockback) + 无敌帧 (i-frames)
- M21 Demo: AI测试20种武器，10秒

**验收**:
- 攻击判定精确
- 伤害计算包含修饰符

---

### M22: 敌人AI深化
**目标**: 有趣的Boss战
- 敌人模板: 近战冲锋、远程狙击、飞行、地面钻洞
- Boss阶段系统: HP阈值触发新技能
- 弹幕模式 (Noita风格): 密集投射物
- M22 Demo: 与3阶段Boss战斗，10秒

**验收**:
- Boss阶段切换平滑
- 弹幕模式挑战但可躲避

---

### M23: 护甲/装备系统
**目标**: Diablo风格装备槽
- 装备槽: 头盔、胸甲、腿甲、鞋子、饰品×3
- 属性加成: 防御、速度、暴击、元素抗性
- 套装效果 (set bonus)
- M23 Demo: AI装备5套装备，对比属性，10秒

**验收**:
- 装备属性正确累加
- 套装效果触发

---

### M24: 状态效果系统
**目标**: 持续伤害/增益
- 状态: 燃烧、中毒、冰冻、速度提升、生命恢复
- 堆叠与持续时间
- 免疫机制
- M24 Demo: AI释放10种状态法术，观察效果，10秒

**验收**:
- 多状态共存正确
- 持续时间倒计时

---

## 魔法与Noita风格 / Magic & Noita-Style

### M25: 法术编辑器UI
**目标**: Noita风格法术自定义
- UI: 拖放法术组件 (投射物 + 修饰符 + 触发器)
- 实时预览: 显示法术效果
- 保存/加载: 法术配置序列化
- M25 Demo: AI创建5个自定义法术，10秒 (UI存根)

**验收**:
- 法术组件组合逻辑正确
- 保存后重新加载一致

---

### M26: 像素反应深化
**目标**: Noita级别的材料交互
- 新材料: 油、毒液、酸、魔法液体
- 复杂反应: 油+火=爆炸，酸+石头=溶解
- 连锁反应: 一次点燃触发连环爆炸
- M26 Demo: 倒50格油，点燃，观察爆炸链，10秒

**验收**:
- 反应链稳定不崩溃
- 视觉效果明显

---

### M27: 法术元素系统
**目标**: 5元素 (火/冰/电/毒/奥术) 交互
- 元素优势: 火克冰，冰克水，电克金属
- 环境反应: 火点燃草，冰冻结水
- 敌人弱点: 特定敌人对元素敏感
- M27 Demo: AI对5种元素敏感敌人测试元素法术，10秒

**验收**:
- 元素优势伤害加成
- 环境反应触发

---

## 经济与进度 / Economy & Progression

### M28: 货币/商店系统
**目标**: Diablo风格经济
- 货币: 金币、宝石
- 商人NPC: 固定商店 + 随机商品
- 价格波动: 基于稀有度 + 需求
- M28 Demo: AI购买10件商品，10秒

**验收**:
- 交易扣除正确金币
- 商品刷新逻辑

---

### M29: 制作系统
**目标**: Terraria风格合成
- 配方: 原料 → 产物
- 工作站: 铁砧、炼金台、附魔台
- 发现机制: 收集材料解锁配方
- M29 Demo: AI合成10种物品，10秒

**验收**:
- 配方逻辑正确
- 工作站检测

---

### M30: 技能树系统
**目标**: Diablo风格天赋
- 技能分支: 战士/法师/游侠
- 技能点分配: 击杀/任务获得
- 技能效果: 被动加成 + 主动技能
- M30 Demo: AI分配20技能点，测试技能，10秒

**验收**:
- 技能互斥逻辑
- 技能效果生效

---

### M31: 成就系统
**目标**: 进度追踪
- 成就类型: 击杀数、收集、探索、合成
- 解锁奖励: 称号、道具、货币
- 统计追踪: 持久化到存档
- M31 Demo: AI解锁5个成就，10秒

**验收**:
- 成就触发时机正确
- 奖励发放

---

## 多人与网络 / Multiplayer & Networking

### M32: 本地多人基础
**目标**: 分屏/热座模式
- 输入分离: 玩家1/2独立控制
- 相机分割: 2个viewport
- 同步Command: 所有玩家动作入队
- M32 Demo: 2个AI分屏探索，10秒

**验收**:
- 分屏渲染正确
- 玩家不干扰彼此输入

---

### M33: 网络同步基础 (可选)
**目标**: Peer-to-peer或Client-Server
- 网络库: 选择quinn (QUIC) 或 laminar (UDP)
- 同步模型: Command同步 (锁步或预测)
- M33 Demo: 2个本地进程通过loopback连接，10秒

**验收**:
- 网络包正确发送/接收
- 基础延迟补偿

---

## 打磨与内容 / Polish & Content

### M34: 粒子系统
**目标**: Juice + 视觉反馈
- 粒子发射器: 爆炸、烟雾、火花
- 物理模拟: 重力、空气阻力
- 渲染优化: GPU instancing
- M34 Demo: 50个粒子发射器，10秒

**验收**:
- 粒子性能 > 60fps
- 视觉效果明显

---

### M35: 音频系统深化
**目标**: 沉浸式音效
- 空间音频: 3D位置衰减
- 音乐系统: 根据战斗/探索切换
- 环境音效: 风、雨、地下回声
- M35 Demo: AI移动触发空间音效，10秒

**验收**:
- 音效位置感明显
- 音乐切换平滑

---

### M36: 摄像机抖动 + Screenshake
**目标**: 打击感
- 抖动系统: 击中/爆炸触发
- 衰减曲线: 幅度随时间减小
- 频率控制: 避免癫痫风险
- M36 Demo: AI触发20次抖动，10秒

**验收**:
- 抖动效果明显但不过度
- 玩家可关闭选项

---

### M37: UI/UX打磨
**目标**: 专业游戏界面
- 菜单系统: 主菜单、暂停、设置
- HUD重制: 血条、法力条、快捷栏美化
- 工具提示: Hover显示物品详情
- M37 Demo: AI导航菜单，查看物品，10秒

**验收**:
- UI响应流畅
- 所有界面支持手柄/键鼠

---

### M38: 教程系统
**目标**: 新玩家引导
- 教程任务: 移动、攻击、挖掘、合成
- 提示系统: 动态显示操作提示
- 跳过选项: 老玩家可跳过
- M38 Demo: AI完成教程流程，10秒

**验收**:
- 教程流程清晰
- 提示不打扰老玩家

---

### M39: 本地化支持
**目标**: 中文/英文双语
- 文本系统: 所有UI文本外部化
- 字体: 支持中文字符 (Noto Sans CJK)
- 切换机制: 设置菜单选择语言
- M39 Demo: AI切换语言，验证UI，10秒

**验收**:
- 中文显示正确
- 切换不需重启

---

### M40: 存档系统深化
**目标**: 多存档 + 云存档
- 多存档槽: 3个独立存档
- 自动保存: 每5分钟或关键事件
- 云同步 (可选): Steam Cloud或自建
- M40 Demo: AI创建3个存档，切换加载，10秒

**验收**:
- 存档独立不冲突
- 自动保存不卡顿

---

### M41: 性能优化与Profiling
**目标**: 60fps稳定
- Profiling: 集成tracy或puffin
- 热点优化: CPU密集区块
- GPU优化: Draw call合批
- M41 Demo: 压力测试场景 (10k实体)，10秒

**验收**:
- 平均帧时 < 16.67ms
- 无卡顿

---

### M42: Mod支持基础
**目标**: 社区内容
- Mod加载器: 从文件夹加载资源
- Lua脚本 (可选): mlua库
- API暴露: 实体生成、物品注册
- M42 Demo: 加载示例Mod，生成自定义敌人，10秒

**验收**:
- Mod不崩溃游戏
- API文档齐全

---

### M43: 成就/统计云同步
**目标**: 跨设备进度
- Steam成就集成 (或自建)
- 统计追踪: 总击杀、总死亡、总游玩时间
- 排行榜 (可选)
- M43 Demo: AI解锁成就，模拟同步，10秒

**验收**:
- 成就正确上传
- 统计数据一致

---

### M44: Beta测试与Bug修复
**目标**: 稳定性
- Bug追踪: GitHub Issues
- 崩溃报告: sentry或自建
- 社区反馈: Discord或论坛
- M44 Demo: 所有M0-M43 Demo通过，10秒

**验收**:
- 无已知Critical bug
- 所有Demo绿灯

---

### M45: 发布准备
**目标**: 上架Steam/itch.io
- 商店页面: 截图、视频、描述
- 安装包: Windows/Linux/macOS
- 启动器: 更新检测、设置向导
- M45 Demo: 安装包可运行，10秒

**验收**:
- 安装包无依赖缺失
- 商店页面审核通过

---

### M46: 后期内容与DLC计划
**目标**: 长期运营
- 内容路线图: 新biome、新Boss、新职业
- DLC架构: 独立可选内容包
- 活动系统: 限时事件、节日主题
- M46 Demo: 加载DLC示例，10秒

**验收**:
- DLC正确加载
- 不影响基础游戏

---

# 🎮 游戏设计目标 / Game Design Goals (G1-G4)

## G1: 探索感 (Terraria风格)
**目标**: 玩家总想看看"下一个洞穴里有什么"
- 地下结构: 宝箱、神殿、Boss房间
- 奖励驱动: 深度越深奖励越好
- 地图系统: 探索后自动填充
- G1 验证: Playtest显示玩家平均探索时间 > 30分钟

---

## G2: 实验感 (Noita风格)
**目标**: 玩家尝试疯狂法术组合
- 法术实验室: 无限资源的测试区域
- 意外发现: 隐藏组合产生强大效果
- 死亡复盘: 显示"你被X杀死"统计
- G2 验证: Playtest中 > 50% 玩家尝试 > 10种法术组合

---

## G3: 刷子感 (Diablo风格)
**目标**: 玩家沉迷于刷装备
- 掉落爽感: 稀有物品特效 + 音效
- 随机词缀: 每次掉落不同
- Build多样性: 100+ 可行的装备/技能组合
- G3 验证: Playtest中玩家平均游玩时间 > 10小时

---

## G4: 社交感 (多人协作)
**目标**: 朋友一起玩更有趣
- 协作机制: Boss需要团队配合
- 交易系统: 玩家间物品交换
- 共享进度: 世界探索进度共享
- G4 验证: 多人模式留存率 > 单人模式 20%

---

# 📋 开发优先级 / Development Priority

**Phase 1 (Foundation Deepening)**: M16-M20
- 重点: 世界生成 + NPC智能
- 目标: 让世界"活"起来

**Phase 2 (Combat & Magic)**: M21-M27
- 重点: 战斗深度 + Noita风格魔法
- 目标: 核心玩法有趣

**Phase 3 (Progression & Economy)**: M28-M31
- 重点: 进度循环 + 经济系统
- 目标: 长期留存

**Phase 4 (Multiplayer & Polish)**: M32-M38
- 重点: 多人 + UI打磨
- 目标: 社交与专业感

**Phase 5 (Release & Beyond)**: M39-M46 + G1-G4
- 重点: 发布准备 + 长期内容
- 目标: 商业化与社区

---

# ⚠️ 非目标 / Non-Goals

以下不在此路线图范围：
- **不做**: 3D渲染 (保持2D像素风格)
- **不做**: MMO级别网络 (最多4-8人联机)
- **不做**: 手机移植 (PC为主)
- **不做**: VR支持

---

# 📝 Agent开发注意事项

- **里程碑顺序**: 按M16→M17→...顺序开发，不跳跃
- **Demo要求**: 每个里程碑仍需5–15秒Demo验证
- **SHOWCASE更新**: 新特性必须加入SHOWCASE章节
- **确定性优先**: 所有新系统保持seeded determinism
- **性能基准**: 保持60 TPS/FPS目标
- **中文友好**: 文档/注释中英文均可

Ker最终审查所有里程碑，Agent完全自主开发中间过程。
