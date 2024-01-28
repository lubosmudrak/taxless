cd ../app

cargo build --target-dir ../deployment/deployed/

cp templates ../deployment/deployed/debug/ -r

