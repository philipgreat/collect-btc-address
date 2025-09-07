#!/bin/bash



function run_inst()
{
	local config_list="$@"
	for config in "$@"
	do
        echo "running config for ${config}"
		ps -ef|grep -v "grep" |grep "${config}.log" ||  nohup  ./data/collect-btc-address "${config}.log" >> ${config}.log &
	done

}

run_inst cba-001 cba-002 cba-003
