MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

DEBUG_EMAIL := ""
COGNITO_SESSION := ""
BATCH_COMMAND := "sync-work"

clean:
	cargo clean
	rm -rf target_lambda

build-linux:
	cargo build --release --bin api --target x86_64-unknown-linux-musl

build:
	cd api && cargo build
	cd subscriber && cargo build
	cd batch && cargo build

build-api:
	cd api && cargo build

build-subscriber:
	cd subscriber && cargo build

build-batch:
	cd batch && cargo build

deploy:
	docker build -t lambda_builder .
	docker run -it --rm -v ~/.cargo/registry:/root/.cargo/registry:z -v $(PWD):/build:z lambda_builder
	sam deploy --profile me

run-local:
	cd api && SSM_PARAMETER=/canvas-nft/server/dotenv cargo run

run-batch:
	aws lambda invoke \
		--function-name canvas-nft-server-BatchFunction-Yiw850yJa7oH \
		--payload '{"command":"${BATCH_COMMAND}"}' \
		--cli-binary-format raw-in-base64-out \
		--profile me \
		/dev/null

debug-set-password:
	aws cognito-idp admin-set-user-password \
        --user-pool-id ap-northeast-1_omBvnPYzl \
        --username ${DEBUG_EMAIL} \
        --password Test1234 \
        --profile me

debug-challenge-password:
	aws cognito-idp admin-respond-to-auth-challenge \
		--user-pool-id ap-northeast-1_omBvnPYzl \
		--client-id ehd60ftsekljsqu683f2j6i0e \
    	--challenge-name NEW_PASSWORD_REQUIRED \
        --challenge-responses NEW_PASSWORD=Test1234,USERNAME=${DEBUG_EMAIL} \
        --session ${COGNITO_SESSION} \
    	--profile me

debug-token:
	aws cognito-idp admin-initiate-auth \
        --user-pool-id ap-northeast-1_omBvnPYzl \
        --client-id ehd60ftsekljsqu683f2j6i0e \
        --auth-flow ADMIN_NO_SRP_AUTH \
        --auth-parameters USERNAME=${DEBUG_EMAIL},PASSWORD=Test1234 \
        --profile me

extract-abi:
	cat ethereum/build/contracts/Canvas721.json | jq '.abi' > app/src/ethereum/canvas_erc721.abi.json
	cat ethereum/build/contracts/Canvas1155.json | jq '.abi' > app/src/ethereum/canvas_erc1155.abi.json