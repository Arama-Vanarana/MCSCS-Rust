# Minecraft Server Config Script for Rust
_**Minecraft Server Config Script for Rust, 简称MCSCS for Rust**_<img src="https://www.rust-lang.org/static/images/rust-logo-blk.svg" width="25">

## 介绍
MCSCS for Rust 是一个用于配置/创建Minecraft服务器的Rust脚本

## 使用方法
### 直接下载程序

* [下载最新版](../../releases/latest)

运行程序
```powershell
mcscs.exe
```

### 自行编译
* 从Rust官网下载Rustup
<div id="platform-instructions-unix" class="instructions db">
    <p>您似乎正在运行 macOS、Linux 或其它类 Unix 系统。要下载 Rustup 并安装 Rust，请在终端中运行以下命令，然后遵循屏幕上的指示。如果您在 Windows 上，请参见 <a href="https://forge.rust-lang.org/infra/other-installation-methods.html">“其他安装方式”</a>。</p>
    <pre><code class="db w-100">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
    </div>
    <div id="platform-instructions-win" class="instructions dn">
    <p>您似乎正在运行 Windows。要使用 Rust，请下载安装器，然后运行该程序并遵循屏幕上的指示。当看到相应提示时，您可能需要安装 <a href="https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/">Microsoft C++ 生成工具</a>。如果您不在 Windows 上，参看 <a href="https://forge.rust-lang.org/infra/other-installation-methods.html">“其他安装方式”</a>。</p>
    <div class="mw9 center ph3-ns">
    <div class="cf ph2-ns">
        <div class="fl w-100 w-50-ns pa2">
        <a href="https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe" class="button button-secondary">下载 <span class="nowrap">rustup-init.exe</span><span class="nowrap">（32位）</span></a>
        </div>
        <div class="fl w-100 w-50-ns pa2">
        <a href="https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" class="button button-secondary">下载 <span class="nowrap">rustup-init.exe</span><span class="nowrap">（64位）</span></a>
        </div>
    </div>
    </div>
    <h3 class="mt4">Windows 的 Linux 子系统（WSL）</h3>
    <p>如果您是 Windows 的 Linux 子系统（WSL）用户，要安装 Rust，请在终端中运行以下命令，然后遵循屏幕上的指示。</p>
    <pre><code class="db w-100">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
    </div>
    <div id="platform-instructions-unknown" class="instructions dn">
    <!-- unrecognized platform: ask for help -->
    <p>
        Rust 可在 Windows、Linux、macOS、FreeBSD 和 NetBSD 上运行。如果您在这些平台上看到了本条信息，请报告一个问题并附上以下内容：
    </p>
    <div class="install-details code">
        <div>navigator.platform:
        <span id="nav-plat">MacIntel</span>
        </div>
        <div>navigator.appVersion:
        <span id="nav-app">5.0 (Macintosh)</span>
        </div>
    </div>
    <br/>
    <a href="https://github.com/rust-lang/www.rust-lang.org/issues/new" class="button button-secondary">报告问题</a>
    <hr/>
    <div>
        <p>
        如果您正在运行 Unix，要安装 Rust，<br>请在终端中运行以下命令，然后遵循屏幕上的指示。
        </p>
        <code class="db w-100">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code>
    </div>
    <hr>
    <div>
        <p>
        如果您正在运行 Windows，<br>请下载并运行 <a href="https://win.rustup.rs">rustup‑init.exe</a>，然后遵循屏幕上的指示。
        </p>
    </div>
    </div>
    <div id="platform-instructions-default" class="instructions dn">
    <div>
        <p>
        </p>
        <pre><code class="db w-100">curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>
    </div>
    <hr>
    <div>
        <p>
        如果您正在运行 Windows，<br>请下载并运行 <a href="https://win.rustup.rs">rustup‑init.exe</a>，然后遵循屏幕上的指示。
        </p>
    </div>
    </div>
</div>