#!/bin/bash
set -e

script=$(readlink -f "$0")
route=$(dirname "$script")

tgt_version=$1
tgt_install_prefix=$2
pkg_name=jzhd

if [ "${tgt_version}" == "" ]; then
    tgt_version=$(git describe --tag 2>/dev/null || echo "0.0.0")
    if [ "${tgt_version}" == "0.0.0" ] && [ -e "${route}/../version" ]; then
        tgt_version=$(cat ${route}/../version)
        echo "No version number can be achieved from git, so we use value in '${pkg_name}/version' as the target version: ${tgt_version}"
    fi
fi
echo ${tgt_version} > ${route}/../version
echo "Version is ${tgt_version} and it has been written into 'proj_root/version'"

if [ "${tgt_install_prefix}" == "" ]; then
    tgt_install_prefix=/opt/jz
fi
echo "deb install prefix is ${tgt_install_prefix}"

uname_arch=$(uname -m)
if [ x"${uname_arch}" == x"x86_64" ]; then
    arch=amd64
elif [ x"${uname_arch}" == x"aarch64" ]; then
    arch=arm64
else
    echo "not support arch ${uname_arch}" >&2
    exit 2
fi

### 1. make the working dir
if [ -e ${route}/../dist ]; then
    rm -rf ${route}/../dist
fi
mkdir -p ${route}/../dist/${pkg_name}
mkdir -p ${route}/../dist/${pkg_name}/DEBIAN
mkdir -p ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}
mkdir -p ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}/bin

## 2. copy targets to deb ready dir
cp -r ${route}/../etc ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}/
cp -r ${route}/gen_my_jz_deps ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}/
cp -r ${route}/../install/* ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}/
cp -r ${route}/../target/x86_64-unknown-linux-gnu/release/jzhd ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}/bin
chmod +x ${route}/../dist/${pkg_name}/${tgt_install_prefix}/${pkg_name}/bin/jzhd

## 3. make various config files under DEBIAN dir
cd ${route}/../dist/${pkg_name}/DEBIAN
touch control
(cat << EOF
Package: ${pkg_name}
Version: ${tgt_version}
Section: x11
Priority: optional
Depends: 
Suggests:
Architecture: ${arch}
Maintainer: jz-basic-soft-group
CopyRight: commercial
Provider: JZ TECH Corp.
Description: Various ros package deps for JZ Robots.
EOF
) > control

touch postinst
(cat << EOF
#!/bin/bash
session_user=\`echo \${SUDO_USER:-\$USER}\`
uid=\`id -un \${session_user}\`
gid=\`id -gn \${session_user}\`
pkg_files=\$(dpkg -L ${pkg_name} | grep ${tgt_install_prefix}/${pkg_name})
chown \${uid}:\${gid} \${pkg_files}
EOF
) > postinst

touch postrm
(cat << EOF
#!/bin/bash
EOF
) > postrm

touch preinst
(cat << EOF
#!/bin/bash
EOF
) > preinst

touch prerm
(cat << EOF
#!/bin/bash
EOF
) >> prerm

chmod +x postinst postrm preinst prerm

## 4. start to make .deb package
cd ${route}/..
fakeroot dpkg -b dist/${pkg_name} dist/${pkg_name}_${tgt_version}_${arch}.deb || exit 40
rm -rf dist/${pkg_name}
echo "pack ${pkg_name} into deb finished."
exit 0
