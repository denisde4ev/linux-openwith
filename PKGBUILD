# Maintainer: Your Name <your.email@example.com>
pkgname=openwith-denisde4ev
pkgver=1.0.0
pkgrel=1
pkgdesc="A Rust program that detects and launches applications for protocols and file types using kdialog"
arch=('x86_64')
# never tested arch=('x86_64' 'aarch64')
url="https://github.com/yourusername/openwith"
license=('MIT')
depends=('kdialog')
makedepends=('rust')
source=()
md5sums=()

build() {
	cd "$srcdir"
	
	# Remove the shebang 'rust-script' line and compile with rustc
	tail -n +2 "$startdir"/openwith.rs > openwith.rs
	rustc -O openwith.rs -o openwith
	
	# Process desktop files using the install script to srcdir
	mkdir -p applications
	"$startdir"/install-desktop-file.sh \
		-e /usr/bin/openwith \
		-- \
		"$startdir"/openwith-*.desktop \
		applications/ \
	;
}

package() {
	cd "$startdir"
	
	# Install the compiled binary
	install -Dm755 "$srcdir"/openwith "$pkgdir/usr/bin/openwith"

	# Install the processed desktop files
	mkdir -p "$pkgdir/usr/share/applications"
	install -m644 "$srcdir"/applications/*.desktop "$pkgdir/usr/share/applications/"
	
	# Install README if it exists
	if [ -f "$startdir/README.md" ]; then
		install -Dm644 "$startdir/README.md" "$pkgdir/usr/share/doc/$pkgname/README.md"
	fi
}

