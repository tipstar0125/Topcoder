#!/usr/bin/bash

WSL2_PATH=$(cd $(dirname $0); pwd)
source $WSL2_PATH/.env
cd $WIN_PATH
echo $PASSWORD | sudo -S rm -rf /workdir
echo $PASSWORD | sudo -S mkdir /workdir
echo $PASSWORD | sudo -S chmod 777 /workdir
if [ $1 = "m" ]; then
    # multi test
    java.exe -jar tester.jar -exec "wsl -e /usr/bin/python3 $WSL2_PATH/$FILENAME" -delay 1 -seed 1+100*2 -novis -th 4 > $WSL2_PATH/result.txt
    cat $WSL2_PATH/result.txt | grep Score
else
    # single test
    java.exe -jar tester.jar -exec "wsl -e /usr/bin/python3 $WSL2_PATH/$FILENAME" -delay 1 -seed $1
fi

echo $PASSWORD | sudo -S rm -rf /workdir