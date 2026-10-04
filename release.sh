# source this file to use

# RELEASE UTILITY
# This script helps with the release process on Github (musl & glibc builds for Linux)

# TODO: unfinished publishing targets
# echo "Update the README instructions for v$CLI_VERSION"
# echo " ✓ Publish on crates.io"
# echo " ✓ Release on Github with Git tag v$CLI_VERSION"

printf 'available build functions:

	build-macos-aarch64-runtime
	build-linux-intel-deb-package
	build-linux-intel-glibc-runtime
	build-linux-intel-musl-runtime
'
# CLI_VERSION is universal, but setting macos TARGET as default for temp placeholder
export CAT="$(which cat)"
export CLI_VERSION=$("${CAT}" ipscan/Cargo.toml | egrep "version = (.*)" | egrep -o --color=never "([0-9]+\.?){3}" | head -n 1)
export TARGET="aarch64-apple-darwin"
# note: this only works correctly inside other functions because of required variables
function create-or-wipe-builds-dir () {
	if ! [ -d ./builds/ipscan-v$CLI_VERSION-$TARGET ]; then
		mkdir -pv ./builds/ipscan-v$CLI_VERSION-$TARGET; else
		rm -rf ./builds/ipscan-v$CLI_VERSION-$TARGET && mkdir -pv ./builds/ipscan-v$CLI_VERSION-$TARGET
	fi
}

function clean-variables-from-env() {
	unset CLI_VERSION; unset TARGET; unset ARCH; unset CAT;
	echo "if you need to use script again, re-source the file to initialize variables."
}

# build a MacOS copy for M1-M5 Macs
function build-macos-aarch64-runtime() {
	create-or-wipe-builds-dir
	echo "Building v$CLI_VERSION for MacOS M1-M5 (no Intel support)"
	cargo build --release --target="$TARGET" --locked
	cp -pv "./target/$TARGET/release/ipscan" "./builds/ipscan-v$CLI_VERSION-$TARGET"
	"./builds/ipscan-v$CLI_VERSION-$TARGET --version"
	unset TARGET
}

# Build the deb archive
function build-linux-deb-package() {
	create-or-wipe-builds-dir
	export ARCH="$(uname -m)"
	echo "note: script builds architecture specified by host `uname -m` command: $ARCH"
	export TARGET="$ARCH/DEBIAN"
	echo "Building v$CLI_VERSION for Debian and Ubuntu"
	mkdir -pv "./builds/ipscan_$CLI_VERSION-1_$TARGET"
	echo "Package: ipscan" > "./builds/ipscan_$CLI_VERSION-1_$TARGET/control"
	echo "Version: $CLI_VERSION" >> "./builds/ipscan_$CLI_VERSION-1_$TARGET/control"
	echo "Architecture: $ARCH" >> "./builds/ipscan_$CLI_VERSION-1_$TARGET/control"
	echo "Maintainer: nu_plugin_ipscan contributors" >> "./builds/ipscan_$CLI_VERSION-1_$TARGET/control"
	echo "Description: Local-network ARP discovery written in Rust" >> "./builds/ipscan_$CLI_VERSION-1_$TARGET/control"
	mkdir -pv ./builds/ipscan_$CLI_VERSION-1_$ARCH/usr/local/bin
	cp "./builds/ipscan-v$CLI_VERSION-$ARCH-unknown-linux-glibc" "./builds/ipscan_$CLI_VERSION-1_$ARCH/usr/local/bin/ipscan"
	(cd "./builds" && dpkg-deb --build --root-owner-group "ipscan_$CLI_VERSION-1_$ARCH")
	unset ARCH; unset TARGET;
}

# Build a 'musl' release for Linux x86_64
function build-linux-intel-musl-runtime() {
	create-or-wipe-builds-dir
	export TARGET=x86_64-unknown-linux-musl
	echo "Building v$CLI_VERSION for GNU musl targets"
	cargo build --release --target=$TARGET --locked
	cp -pv ./target/$TARGET/release/ipscan ./builds/ipscan-v$CLI_VERSION-$TARGET
	./builds/ipscan-v$CLI_VERSION-$TARGET --version
}

# Build a 'glibc' (GNU) release for Linux x86_64
function build-linux-intel-glibc-runtime() {
	create-or-wipe-builds-dir
	local TARGET=x86_64-unknown-linux-glibc
	echo "Building v$CLI_VERSION for GNU glibc targets"
	cargo build --release --target=$TARGET --locked
	cp -pv ./target/$TARGET/release/ipscan ./builds/ipscan-v$CLI_VERSION-$TARGET
	./builds/ipscan-v$CLI_VERSION-$TARGET --version
}