  PATH="/opt/homebrew/opt/llvm/bin:$PATH" \
  GOPATH=/Users/u/g \
  PATH=$GOPATH/bin:$PATH \
  cd cycles-quartz/examples/pop2-example
  quartz --mock-sgx enclave build
