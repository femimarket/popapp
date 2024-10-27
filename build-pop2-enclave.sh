export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export GOPATH=/Users/u/go
export PATH=$GOPATH/bin:$PATH
export ADMIN_SK=$(yes | neutrond keys export admin --unsafe --unarmored-hex)
cd cycles-quartz/examples/pop2-example
quartz --mock-sgx enclave build
