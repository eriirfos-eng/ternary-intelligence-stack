#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"

SDK=~/android-sdk
BT=$SDK/build-tools/35.0.1
ANDROID_JAR=$SDK/platforms/android-34/android.jar
AAPT2=$BT/aapt2
D8=$BT/d8
APKSIGNER=$BT/apksigner

SRC=src
RES=res
OBJ=obj
OUT=lofty.apk

echo "--- clean ---"
rm -rf $OBJ && mkdir -p $OBJ/{gen,classes,res,dex}

echo "--- compile resources ---"
$AAPT2 compile --dir $RES -o $OBJ/res.zip

echo "--- link resources + manifest ---"
$AAPT2 link \
  -I $ANDROID_JAR \
  --manifest AndroidManifest.xml \
  $OBJ/res.zip \
  --java $OBJ/gen \
  -o $OBJ/app.apk \
  --min-sdk-version 21 \
  --target-sdk-version 34 \
  --version-code 1 \
  --version-name "1.0"

echo "--- compile Java ---"
javac --release 11 -g:none \
  -classpath "$ANDROID_JAR:$OBJ/gen" \
  -sourcepath "$SRC:$OBJ/gen" \
  -d $OBJ/classes \
  $SRC/com/zabih/lofty/MainActivity.java

echo "--- DEX ---"
$D8 \
  --output $OBJ/dex \
  --lib $ANDROID_JAR \
  --min-api 21 \
  $(find $OBJ/classes -name "*.class")

echo "--- add DEX to APK ---"
cp $OBJ/app.apk $OBJ/app_unsigned.apk
(cd $OBJ/dex && zip -j ../app_unsigned.apk classes.dex)

echo "--- generate debug keystore ---"
if [ ! -f $OBJ/debug.keystore ]; then
  keytool -genkey -v \
    -keystore $OBJ/debug.keystore \
    -alias loftykey \
    -keyalg RSA -keysize 2048 -validity 10000 \
    -storepass loftypass -keypass loftypass \
    -dname "CN=Zabih,OU=Lofty,O=RFI,L=Graz,C=AT" 2>/dev/null
fi

echo "--- sign ---"
$APKSIGNER sign \
  --ks $OBJ/debug.keystore \
  --ks-key-alias loftykey \
  --ks-pass pass:loftypass \
  --key-pass pass:loftypass \
  --out $OUT \
  $OBJ/app_unsigned.apk

echo ""
echo "built: $(pwd)/$OUT ($(du -h $OUT | cut -f1))"
