# jsrunner

This is an experimentation to lean implementing gRPC service to run JavaScript code in Rust with V8.

## Start gRPC server

```
cargo run
```

## gRPC call examples

### simple

```
time grpcurl \
  -plaintext \
  -import-path ./proto \
  -proto service.proto \
  -d '{"code": "1+1"}' \
  '[::1]:50051' jsrunner.RunnerService/Run
```

```
while true; do grpcurl \
  -plaintext \
  -import-path ./proto \
  -proto service.proto \
  -d '{"code": "new Date()"}' \
  '[::1]:50051' jsrunner.RunnerService/Run; sleep 1; done
```

### object
```
CODE=$(cat <<EOF
var name = "dragon3"
var obj = {"name": name, "ts": new Date()};
obj;
EOF
)

grpcurl \
  -plaintext \
  -import-path ./proto \
  -proto service.proto \
  -d "$(jq -cn --arg code "${CODE}" '$ARGS.named')" \
  '[::1]:50051' jsrunner.RunnerService/Run
```

### long running
```
CODE=$(cat <<EOF
var sum = 0;
for (var i = 0; i < 1e9; i++) {
    sum += i;
}
sum;
EOF
)

grpcurl \
  -plaintext \
  -import-path ./proto \
  -proto service.proto \
  -d "$(jq -cn --arg code "${CODE}" '$ARGS.named')" \
  '[::1]:50051' jsrunner.RunnerService/Run
```

### memory limit

```
CODE=$(cat <<EOF
var arr = [];
for (var i = 0; i < 1e7; i++) {
    arr.push(new Array(1000).fill('a'));
}
arr.length;
EOF
)

grpcurl \
  -plaintext \
  -import-path ./proto \
  -proto service.proto \
  -d "$(jq -cn --arg code "${CODE}" '$ARGS.named')" \
  '[::1]:50051' jsrunner.RunnerService/Run
```
