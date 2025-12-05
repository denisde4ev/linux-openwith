pkgname=openwith-denisde4ev-git
pkgver=1.0.0
pkgrel=1
pkgdesc="OpenWith dialog for files and URL/URI"
arch=('x86_64')
# never tested arch=('x86_64' 'aarch64')
url="https://github.com/denisde4ev/linux-openwith"
license=('MIT')
depends=('kdialog')
makedepends=('rust')
source=()
# source=("https://github.com/denisde4ev/linux-openwith")
md5sums=()

build() {
	cd "$srcdir"

	#rustc -O -C debuginfo=0 "$startdir"/opener.rs -o openwith
	rustc \
		-C opt-level=3 \
		-C lto=fat \
		-C codegen-units=1 \
		-C panic=abort \
		"$startdir"/openwith.rs -o openwith \
	;
	
	# Process desktop files using the install script to srcdir
	mkdir -p applications
	"$startdir"/install-desktop-file.sh \
		-e /usr/bin/openwith \
		-- \
		"$startdir"/desktop-files/openwith-*.desktop \
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
