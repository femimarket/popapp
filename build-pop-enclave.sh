  PATH="/opt/homebrew/opt/llvm/bin:$PATH" \
  GOPATH=/Users/u/g \
  PATH=$GOPATH/bin:$PATH \
  cd cycles-quartz/examples/transfers
  quartz --mock-sgx enclave build
