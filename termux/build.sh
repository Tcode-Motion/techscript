TERMUX_PKG_HOMEPAGE=https://github.com/Tcode-Motion/techscript
TERMUX_PKG_DESCRIPTION="A friendly native programming language that reads like plain English."
TERMUX_PKG_LICENSE="MIT"
TERMUX_PKG_MAINTAINER="Tanmoy <tanmoy@example.com>"
TERMUX_PKG_VERSION=1.0.2
TERMUX_PKG_SRCURL=https://github.com/Tcode-Motion/techscript/archive/refs/tags/v1.0.2.tar.gz
TERMUX_PKG_DEPENDS="python" # Fallback dependency or wrapper execution
TERMUX_PKG_BUILD_IN_SRC=true

termux_step_make() {
    # If compiled with rust
    # cargo build --release --target aarch64-linux-android
    # For now, it delegates to python wrapper if prebuilt binary isn't available
    return
}

termux_step_make_install() {
    # Setup pip wrapper installing to Termux PREFIX
    pip install . --prefix $TERMUX_PREFIX
}
