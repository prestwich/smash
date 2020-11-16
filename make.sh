cd ./call_geth && \
rm -f call_geth && \
go build -o call_geth && \
cd ..

cd ./call_celo && \
rm -f call_celo && \
go build -o call_celo && \
cd ..