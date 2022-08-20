#!/bin/zsh
export BUILD_ID=dontKillMe
source $HOME/.cargo/env
cargo build --release

if [ $? -ne 0 ]
then
	echo Error:Build failed,exit
	exit 1
fi

pid=`ps -ef|grep edgeless-backend|grep -v grep|awk '{print $2}'`
if [ -n "$pid" ]
then 
	echo "kill " $pid
	kill -9 $pid
fi

if [ -e "nohup.out" ]
then
	rm nohup.out
fi

nohup ./target/release/edgeless-backend&