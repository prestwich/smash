cd ./geth_bindings
rm go.sum
go get -u
make static_external

cd ../celo_bindings
rm go.sum
go get -u
make static_external