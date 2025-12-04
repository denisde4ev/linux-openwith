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
	cd "$startdir"
	
	# Remove the shebang 'rust-script' line and compile with rustc
	tail -n +2 openwith.rs > "$srcdir"/opener.rs
	rustc -O "$srcdir"/opener.rs -o "$srcdir"/openwith
}

package() {
	cd "$startdir"
	
	# Install the compiled binary
	install -Dm755 "$srcdir"/openwith "$pkgdir/usr/bin/openwith"


	# Use the install script with -e option to specify the binary path
	mkdir -p "$pkgdir/usr/share/applications"
	./install-desktop-file.sh \
		-e /usr/bin/openwith \
		-- \
		./opener-*.desktop \
		"$pkgdir/usr/share/applications/" \
	;
	
	# Install README if it exists
	if [ -f README.md ]; then
		install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
	fi
}

