#!/bin/bash

ps -ef|grep -v grep | grep collect-btc |awk '{print "kill -9 "$2}'|bash

bash keep-run.sh
