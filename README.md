# Minecraft Server Config Script for Rust
**Minecraft Server Config Script for Rust, 简称MCSCS for Rust**<img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" width="20">

## 介绍
MCSCS for Rust 是一个用于配置/创建Minecraft服务器的Rust程序

## 使用方法
### 直接下载程序

* [下载最新版](../../releases/latest)

运行程序
```powershell
.\mcscs.exe
```

---

### 自行编译
#### 安装环境
* [下载Rustup](https://www.rust-lang.org/zh-CN/tools/install)
* [下载Git](https://github.com/git-for-windows/git/releases) 如果无法访问请尝试使用[Steamcommunity-302](https://www.dogfight360.com/blog/686)/[Watt toolkit(Steam++)](https://steampp.net)/任意可科学上网工具重试
#### 克隆仓库
* 打开Git Bash
* 执行以下命令(任意选一)
```bash
# gitee(国内用户推荐)
git clone https://gitee.com/Arama-Vanarana/MCSCS-Rust --depth 0
```
```bash
# github(国外用户推荐)
git clone https://github.com/Arama-Vanarana/MCSCS-Rust --depth 0
```
#### 构建程序
* 等待完成后执行以下命令构建并运行
```powershell
cd MCSCS-Rust
cargo run -r
```
* _`可选`_ 复制构建完成后的程序到目录
```powershell
# '.\' 可替换为任意目录
copy .\target\release\mcscs.exe .\
```

# 常见问题
## `sudo su`后还是权限错误
* 运行: `chmod +x mcscs`