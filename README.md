# collect-btc-address

## 编译(第一次使用)

```shell
git clone https://github.com/philipgreat/collect-btc-address.git
cd collect-btc-address
make

```

## 运行

```shell
bash keep-run.sh
```

## 调整实例个数(默认6个，大多数用户可用)

```shell
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
```

修改最后一行，增加或者减少，默认三个,  比如跑一个:

```shell
run_inst cba-001
```

跑两个:

```shell
run_inst cba-001 cba-002
```


## 移动设备注意


地址来源

https://bitinfocharts.com/top-100-richest-bitcoin-addresses-3.html

https://github.com/Pymmdrza/Rich-Address-Wallet/blob/main/10000BitcoinRichWalletAdd.txt
