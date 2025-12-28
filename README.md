# Clip - 文本文件到剪贴板工具

一款基于 Rust 的轻量级 Windows 命令行工具，用于复制文本文件内容到剪贴板，支持自动编码检测。

## ✨ 特性

- 🔍 **自动编码检测**：智能识别文件编码（UTF-8、GBK、Shift-JIS、Big5 等）
- 🚀 **高性能转码**：使用 Windows API 进行编码转换，零额外依赖
- 📦 **小巧体积**：编译后约 200-300KB，UPX 压缩后更小
- 💡 **智能 BOM 识别**：自动检测并处理 UTF-8/UTF-16 BOM
- 🎯 **简单易用**：单个命令即可完成操作

## 🆚 与 Windows 自带 clip.exe 的区别

Windows 系统自带了 `clip.exe` 命令，但本项目提供了更强大的功能：

| 特性 | Windows clip.exe | 本项目 clip.exe |
|------|------------------|----------------|
| **使用方式** | 通过管道：`type file.txt \| clip` | 直接指定文件：`clip file.txt` |
| **编码检测** | ❌ 不支持，依赖系统代码页 | ✅ 自动检测 30+ 种编码 |
| **BOM 处理** | ⚠️ 可能出现乱码 | ✅ 智能识别和处理 |
| **多语言支持** | ⚠️ 容易乱码（日文/韩文等） | ✅ 完整支持 CJK 和欧洲语言 |
| **错误提示** | ❌ 无详细信息 | ✅ 显示编码检测结果和置信度 |
| **跨编码处理** | ❌ 无法处理 | ✅ 自动转换到 UTF-16 |

### 命名说明

本项目使用 `clip.exe` 作为可执行文件名，建议：
- 放在非系统路径中使用（避免与系统 `clip.exe` 冲突）
- 或重命名为 `clipx.exe`、`clip-rs.exe` 等
- 通过完整路径调用：`D:\Tools\clip.exe file.txt`

## 📥 安装

### 从源码编译

```bash
# 克隆项目
git clone https://github.com/maboloshi/clip.git
cd clip

# 一键编译
build.bat

# 手动编译
cargo build --bin clip --release
cargo build --bin clipw --release

# 可执行文件位于
# target/release/clip.exe   - 控制台版本（终端使用）
# target/release/clipw.exe  - GUI 版本（无窗口闪现）
```

**两个版本的区别：**

| 版本 | 文件名 | 适用场景 | 窗口行为 |
|------|--------|---------|---------|
| 控制台版本 | `clip.exe` | 终端/命令行使用 | 终端中显示信息 |
| GUI 版本 | `clipw.exe` | 快捷键/右键菜单/自动化 | 无窗口，完全静默 |

### 可选：进一步压缩

```bash
# 使用 UPX 压缩（需要先下载 UPX）
upx --best --lzma target/release/clip.exe
upx --best --lzma target/release/clipw.exe
```

## 🚀 使用方法

### 基本用法

```bash
# 控制台版本（终端使用）
clip.exe <文件路径>
clip.exe -s <文件路径>     # 静默模式
clip.exe -h                # 显示帮助

# GUI 版本（无窗口，适合快捷键）
clipw.exe <文件路径>       # 自动静默，无窗口闪现

# 示例
clip.exe document.txt
clip.exe -s chinese.txt
clipw.exe script.ps1       # 无窗口，直接复制
```

### 版本选择指南

**使用 `clip.exe` 当你需要：**
- ✅ 在终端/命令行中使用
- ✅ 查看编码检测结果
- ✅ 查看复制是否成功
- ✅ 在脚本中使用并查看输出

**使用 `clipw.exe` 当你需要：**
- ✅ 绑定到快捷键（无窗口闪现）
- ✅ 右键菜单调用
- ✅ 自动化脚本中静默运行

### 命令行选项

| 选项 | 简写 | 说明 | clip.exe | clipw.exe |
|------|------|------|----------|-----------|
| `--silent` | `-s` | 静默模式 | ✅ | 🔇 默认静默 |
| `--help` | `-h` | 显示帮助 | ✅ | ❌ 无输出 |

### 输出示例

**控制台版本（clip.exe）：**
```
⧗ 正在检测文件编码...
✓ 检测到编码: gbk (置信度: 95%)
✓ 已复制到剪贴板 (1234 字符)
```

**控制台版本静默模式（clip.exe -s）：**
```
（无任何输出，静默复制到剪贴板）
```

**GUI 版本（clipw.exe）：**
```
（无任何输出，无窗口闪现，直接复制到剪贴板）
```

### 退出码

| 退出码 | 说明 |
|-------|------|
| `0` | 成功（复制成功或主动请求帮助） |
| `1` | 失败（文件不存在、编码错误、参数错误等） |

### 实际使用场景

**场景 1：终端使用**
```bash
# 使用 clip.exe，查看编码信息
D:\> clip.exe chinese.txt
⧗ 正在检测文件编码...
✓ 检测到编码: gbk (置信度: 95%)
✓ 已复制到剪贴板 (1234 字符)
```

**场景 2：快捷键绑定（AutoHotkey）**
```ahk
; 按 Win+C 复制当前文件内容
#c::
{
    ; 使用 clipw.exe，无窗口闪现
    Run, clipw.exe "%A_ScriptDir%\data.txt", , Hide
}
```

**场景 3：右键菜单**
```registry
; 注册表脚本：添加"复制文件内容"到右键菜单
; 使用 clipw.exe 避免窗口闪现
[HKEY_CLASSES_ROOT\*\shell\CopyFileContent]
@="复制文件内容"
[HKEY_CLASSES_ROOT\*\shell\CopyFileContent\command]
@="\"C:\\Tools\\clipw.exe\" \"%1\""
```

**场景 4：批处理脚本**
```batch
@echo off
REM 使用 clip.exe 查看结果
clip.exe document.txt
if %ERRORLEVEL% EQU 0 (
    echo 复制成功
) else (
    echo 复制失败
)
```

**场景 5：PowerShell 静默脚本**
```powershell
# 使用 clipw.exe 完全静默
& clipw.exe $env:TEMP\data.txt
if ($LASTEXITCODE -eq 0) {
    # 检查剪贴板验证成功
    Get-Clipboard
}
```

## 📋 支持的编码

### 通过 BOM 自动识别
- UTF-8 (with BOM)
- UTF-16 LE
- UTF-16 BE

### 通过 chardet 自动检测
- UTF-8 (without BOM)
- GBK / GB2312 / GB18030（简体中文）
- Big5（繁体中文）
- Shift-JIS / EUC-JP（日文）
- EUC-KR / CP949（韩文）
- Windows-1252 / ISO-8859-1（西欧）
- 以及其他常见编码

### 通过 Windows API 转码
- 所有 Windows 支持的代码页（30+ 种）

## 🏗️ 技术架构

### 编码检测：chardet vs charset-normalizer-rs

本项目最终选择了 **chardet** 而非 charset-normalizer-rs，原因如下：

| 特性 | chardet | charset-normalizer-rs |
|------|---------|----------------------|
| **体积** | ~200-300KB ✅ | ~400-500KB |
| **准确率** | 80-85% | 90-95% |
| **依赖** | 轻量 ✅ | 重量级（含 regex） |
| **性能** | 快速 ✅ | 稍慢 |
| **使用场景** | 日常文本文件 ✅ | 复杂编码混合 |

**选择理由：**

1. **体积优先**：chardet 减少约 40-50% 的最终文件大小
2. **依赖简洁**：避免引入大量正则表达式库（regex_automata、aho_corasick 等）
3. **准确率足够**：对于常见文本文件，80-85% 的准确率已经满足日常使用
4. **性能优秀**：更快的检测速度，适合频繁使用

**如果你需要更高准确率**，可以修改 `Cargo.toml`：

```toml
[dependencies]
# chardet = "0.2"
charset-normalizer-rs = "1.1"
```

并相应修改 `src/main.rs` 中的 API 调用。

### 编码转换：Windows API

使用 Windows 原生的 `MultiByteToWideChar` API 进行编码转换：

- ✅ 零依赖（不需要 encoding_rs）
- ✅ 系统级可靠性
- ✅ 支持所有 Windows 代码页
- ✅ 高性能本地调用

## 📊 性能数据

### 文件体积对比

| 配置 | 编译后大小 | UPX 压缩后 |
|------|-----------|-----------|
| Debug 版本 | ~2.4MB | - |
| Release (chardet) | ~230KB | ~150KB |
| Release (charset-normalizer-rs) | ~450KB | ~250KB |

### cargo bloat 分析（chardet 版本）

```
主要占用：
- chardet: ~30-40KB
- std 库: ~150KB
- 其他依赖: ~50KB
```

## ⚙️ 编译优化

### Cargo.toml 配置

```toml
[profile.release]
opt-level = "z"          # 体积优化
lto = "fat"              # 链接时优化
codegen-units = 1        # 最大优化
panic = "abort"          # 移除 panic 展开
strip = true             # 移除符号表
```

### 可选：链接器优化

创建 `.cargo/config.toml`：

```toml
[target.x86_64-pc-windows-msvc]
rustflags = [
    "-C", "link-arg=/MERGE:.rdata=.text",
    "-C", "target-cpu=x86-64-v2",
]
```

## 🔧 项目结构

```
clip/
├── .cargo/
│   └── config.toml          # 编译器标志配置
├── src/
│   └── main.rs              # 主程序
├── Cargo.toml               # 包配置
└── README.md
```

## 📝 实现细节

### 编码检测流程

```
1. 检测 BOM 标记
   ├─ UTF-8 BOM (0xEF 0xBB 0xBF) → UTF-8
   ├─ UTF-16 LE (0xFF 0xFE) → UTF-16 LE
   └─ UTF-16 BE (0xFE 0xFF) → UTF-16 BE
   
2. 如果无 BOM，使用 chardet 检测
   └─ 返回编码名称和置信度
   
3. 使用 Windows API 转码
   └─ MultiByteToWideChar(codepage, bytes) → String
```

### 支持的代码页映射

程序内置了常见编码到 Windows 代码页的映射：

- UTF-8 → CP 65001
- GBK/GB2312 → CP 936
- Big5 → CP 950
- Shift-JIS → CP 932
- 等等...

## 🐛 已知限制

1. **编码检测不是 100% 准确**：没有 BOM 的情况下，短文本可能误判
2. **仅支持文本文件**：二进制文件会导致错误
3. **仅支持 Windows**：使用了 Windows 特定的 API

## 📜 更新日志

### v0.1.2 (2025-12-28)
- **新增**: 编译 GUI 版本（clipw.exe），无窗口闪现

### v0.1.1 (2025-12-27)
- **新增**: 添加 `-h/--help` 参数显示帮助信息
- **新增**: 添加 `-s/--silent` 静默模式
- **改进**: 优化输出逻辑，使用宏简化代码
- **修复**: 帮助信息输出到 stdout 而非 stderr
- **修复**: `--help` 退出码改为 0（正常），缺少参数仍为 1（错误）

### v0.1.0 (2025-12-27)
- 初始版本发布
- 自动编码检测（使用 chardet）
- Windows API 编码转换
- 支持静默模式
- 支持 30+ 种编码

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License - 可自由使用、修改和分发

## 🙏 致谢

- [chardet](https://crates.io/crates/chardet) - 轻量级编码检测库
- [charset-normalizer-rs](https://crates.io/crates/charset-normalizer-rs) - 备选高精度编码检测方案
- Windows API - 提供可靠的系统级编码转换
- [Claude](https://claude.ai) (Anthropic) - AI 助手，协助项目开发与架构设计