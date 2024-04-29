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
pushd ../AVBible/bin/Debug/net8.0-windows10.0.17763.0 > /dev/null
rm -f pinshot_blue.*
popd > /dev/null
pushd ../AVBible/bin/x64/Release/net8.0-windows10.0.17763.0 > /dev/null
rm -f pinshot_blue.*
popd > /dev/null
echo copy debug binaries to AV-Console
cp target/debug/*.dll ../AV-Console/bin/Debug/net8.0
cp target/debug/*.pdb ../AV-Console/bin/Debug/net8.0

echo copy debug binaries to AV-Bible
cp target/debug/*.dll ../AVBible/bin/Debug/net8.0-windows10.0.17763.0
cp target/debug/*.pdb ../AVBible/bin/Debug/net8.0-windows10.0.17763.0

echo copy debug binaries to AV-API
cp target/debug/*.dll ../AV-API/bin/Debug/net8.0
cp target/debug/*.pdb ../AV-API/bin/Debug/net8.0

echo copy release binaries to AV-Bible and AV-API
cp target/release/*.dll ../AVBible/bin/x64/Release/net8.0-windows10.0.17763.0
cp target/release/*.dll ../AV-API/bin/Release/net8.0
