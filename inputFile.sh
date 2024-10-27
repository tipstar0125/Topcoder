WIN_PATH=/mnt/c/Users/06106/src/git/kyopro_py/mm155
WSL2_PATH=/home/yoke/src/topcoder/mm155
cd $WIN_PATH
for i in {1..10} ; do
    java.exe -jar tester.jar -exec "wsl -e /usr/bin/python3 $WSL2_PATH/Arrows.py" -novis -delay 1 -seed ${i} > $WSL2_PATH/in/${i}.txt
done