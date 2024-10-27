#!/bin/bash

set -eu

WSL2_PATH=$(cd $(dirname $0); pwd)
source $WSL2_PATH/.env

# Topcoder MM の実行可能ファイルのパス
# 一時ファイルは /workdir に置くといいらしい
TEMP_PATH="/workdir/a.out"

rm -f submission.zip
cargo clean

# --target x86_64-unknown-linux-musl とすると、共有ライブラリに依存しない実行可能ファイルを作れる。
# いろいろ制限はあるが、競プロ目的であれば気にする必要はない。
cargo build --release --target x86_64-unknown-linux-musl --features build

# 以下のようなPythonファイルを生成する。
# やっていることは単純で、BINにBase64文字列として入っている実行可能ファイルを書き出して、
# パーミッションを変更して、実行する。
# 
# import base64
# import subprocess
# import os
# import stat
# BIN='${BINARY}'
# byte_array = base64.b64decode(BIN)
# with open('${TEMP_PATH}', mode='wb') as f:
#     f.write(byte_array)
# st = os.stat('${TEMP_PATH}')
# os.chmod('${TEMP_PATH}', st.st_mode | stat.S_IEXEC)
# subprocess.call('${TEMP_PATH}')

BINARY=$(base64 -w 0 ./target/x86_64-unknown-linux-musl/release/$PROJECT)
echo "import base64" > $FILENAME
echo "import subprocess" >> $FILENAME
echo "import os" >> $FILENAME
echo "import stat" >> $FILENAME
echo "BIN='${BINARY}'" >> $FILENAME
echo "byte_array = base64.b64decode(BIN)" >> $FILENAME
echo "with open('${TEMP_PATH}', mode='wb') as f:" >> $FILENAME
echo "    f.write(byte_array)" >> $FILENAME
echo "st = os.stat('${TEMP_PATH}')" >> $FILENAME
echo "os.chmod('${TEMP_PATH}', st.st_mode | stat.S_IEXEC)" >> $FILENAME
echo "subprocess.call('${TEMP_PATH}')" >> $FILENAME

# 提出用にzipする
zip -j submission.zip $FILENAME

cp submission.zip $WIN_PATH