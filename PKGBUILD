# Maintainer: Saphira Kai <kai.saphira@gmail.com>
pkgname='pipeline-portal'
pkgver=0
pkgrel=1
epoch=
pkgdesc='A simple shell scripting utility that allows you to manipulate the flow of pipelines'
arch=('x86_64')
url='http://g.aybit.ch'
license=('GPLv2')
groups=()
depends=('cat')
makedepends=('cargo')
checkdepends=()
optdepends=()
provides=('portal')
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=('./src/main.rs' 'Cargo.toml')
noextract=()
sha256sums=('SKIP')
validpgpkeys=()

pkgver() {
	mkdir -p "$pkgdir/usr/bin/"
	cd "$pkgdir"
	(
		set -o pipefail
		git describe --long 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' ||
		printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
	)
}

#prepare() {}

#build() {}

#check() {}

package() {
    install -Dm 755 './target/release/portal' "$pkgdir/usr/bin/portal"
}
