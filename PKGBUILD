# Maintainer: L14ms1 <l14ms1@outlook.fr>
pkgname=assbreak
pkgver=0.2.1
pkgrel=1
makedepends=('chromedriver')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="a tool that helps brute-force a website and address mail."
url="https://github.com/L14ms111/assbreak.git"
license=('MPL-2.0')

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/L14ms111/assbreak
}
