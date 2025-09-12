#!/bin/bash

ps -ef|grep -v grep | grep collect-btc |awk '{print "kill -9 "$2}'|bash
grep -r key *.log
rm *.log
cargo build --release
bash keep-run.sh
