# Maintainer: Saphira Kai <kai.saphira@gmail.com>
pkgname='pipeline-portal'
pkgver=r7.459838e
pkgrel=1
epoch=
pkgdesc='A simple shell scripting utility that allows you to manipulate the flow of pipelines'
arch=('x86_64')
url='http://g.aybit.ch'
license=('MIT')
groups=()
depends=('coreutils' 'glibc' 'gcc-libs' )
makedepends=('cargo' 'git')
checkdepends=()
optdepends=()
provides=('portal')
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=("git+https://github.com/SaphiraKai/$pkgname")
noextract=()
sha256sums=('SKIP')
validpgpkeys=()

pkgver() {
	cd "$pkgname"
	(
		set -o pipefail
		git describe --long 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
		printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
	)
}

prepare() {
    cd "$srcdir/pipeline-portal/"
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$srcdir/pipeline-portal/"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    cd "$srcdir/pipeline-portal/"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm 755 "$srcdir/$pkgname/target/release/portal" "$pkgdir/usr/bin/portal"
    install -Dm 644 "$srcdir/$pkgname/LICENSE" "$pkgdir/usr/share/licenses/pipeline-portal/LICENSE"
}
