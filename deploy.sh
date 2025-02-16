cargo build --target=x86_64-pc-windows-msvc --all-targets
cargo build --target=x86_64-pc-windows-msvc --release

pushd ../AV-Console/bin/Debug/net8.0 > /dev/null
rm -f pinshot_blue.*
popd > /dev/null
pushd ../AV-Console/bin/Release/net8.0 > /dev/null
rm -f pinshot_blue.*
popd > /dev/null
pushd ../AV-API/bin/Debug/net8.0
rm -f pinshot_blue.*
popd > /dev/null
pushd ../AV-API/bin/Release/net8.0
rm -f pinshot_blue.*
popd > /dev/null
pushd ../AV-Bible/bin/x64/Debug/net8.0-windows10.0.17763.0 > /dev/null
rm -f pinshot_blue.*
popd > /dev/null
pushd ../AV-Bible/bin/x64/Release/net8.0-windows10.0.17763.0 > /dev/null
rm -f pinshot_blue.*
popd > /dev/null
echo copy debug binaries to AV-Console
cp target/x86_64-pc-windows-msvc/debug/*.dll ../AV-Console/bin/Debug/net8.0
cp target/x86_64-pc-windows-msvc/debug/*.pdb ../AV-Console/bin/Debug/net8.0

echo copy debug binaries to AV-Bible and AV-API and AV-Data-Manager
cp target/x86_64-pc-windows-msvc/debug/*.dll ../AV-Bible/bin/x64/Debug/net8.0-windows10.0.17763.0
cp target/x86_64-pc-windows-msvc/debug/*.pdb ../AV-Bible/bin/x64/Debug/net8.0-windows10.0.17763.0
cp target/x86_64-pc-windows-msvc/debug/*.dll ../AV-API/bin/Debug/net8.0
cp target/x86_64-pc-windows-msvc/debug/*.pdb ../AV-API/bin/Debug/net8.0
cp target/x86_64-pc-windows-msvc/debug/*.dll ../AV-Data-Manager/bin/Debug/net8.0-windows7.0
cp target/x86_64-pc-windows-msvc/debug/*.pdb ../AV-Data-Manager/bin/Debug/net8.0-windows7.0

echo copy release binaries to AV-Bible and AV-API and AV-Data-Manager
cp target/x86_64-pc-windows-msvc/release/*.dll ../AV-Bible/bin/x64/Release/net8.0-windows10.0.17763.0
cp target/x86_64-pc-windows-msvc/release/*.dll ../AV-API/bin/Release/net8.0
cp target/x86_64-pc-windows-msvc/release/*.dll ../AV-Data-Manager/bin/Release/net8.0-windows7.0
