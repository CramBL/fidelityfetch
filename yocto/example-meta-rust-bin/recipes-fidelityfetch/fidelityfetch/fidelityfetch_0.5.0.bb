SUMMARY = "Serve files efficiently on a local network."
HOMEPAGE = "https://github.com/CramBL/fidelityfetch"
BUGTRACKER = "https://github.com/CramBL/fidelityfetch/issues"
LICENSE = "MIT"

inherit cargo_bin
do_compile[network] = "1"

LIC_FILES_CHKSUM = "file://LICENSE-MIT;md5=fd9fdf6d29095c4794820cc4b44d4882"

SRC_URI = "\
    git://github.com/CramBL/fidelityfetch.git;protocol=https;branch=master;tag=v${PV} \
    file://fife \
"

S = "${WORKDIR}/git"

FILES:${PN} += "${bindir}/fife"

# Suppress buildpaths: WARNING: fidelityfetch-0.5.0-r0 do_package_qa: QA Issue: File /usr/bin/.debug/fife in package fidelityfetch-dbg contains reference to TMPDIR [buildpaths]
# nooelint: oelint.vars.insaneskip
INSANE_SKIP = "\
    buildpaths \
"

do_install:append() {
    # Install environment file
    install -d ${D}${sysconfdir}/default
    install -m 0644 ${WORKDIR}/fife ${D}${sysconfdir}/default/

    # Install service file
    install -d ${D}${systemd_unitdir}/system
    install -m 0644 ${S}/package/fife.service ${D}${systemd_unitdir}/system
}

inherit systemd
SYSTEMD_SERVICE:${PN} = "fife.service"