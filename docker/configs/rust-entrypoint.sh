#!/usr/bin/env bash
CMD=$1

case "$CMD" in
    "build" )
        exec cargo build
        ;;

    "test" )
        echo > /syslog-ng/messages.json
        cargo test --features full-integration-env ${@:2}
        ;;

    "test-debug" )
        echo > /syslog-ng/messages.json
        env RUST_BACKTRACE=1 cargo test --features full-integration-env -- --nocapture ${@:2}
        ;;

    "reset" )
        exec echo > /syslog-ng/messages.json
        ;;

    "test-inspect" )
        echo > /syslog-ng/messages.json
        env RUST_BACKTRACE=1 cargo test --features full-integration-env -- --nocapture ${@:2}
        cat /syslog-ng/messages.json | jq --slurp '.[]'
        ;;

    "test-clean" )
        echo > /syslog-ng/messages.json
        cargo clean
        env RUST_BACKTRACE=1 cargo test --features full-integration-env
        cat /syslog-ng/messages.json | jq --slurp '.[]'
        ;;

    * )
        # Run custom command. Thanks to this line we can still use
        # "docker run our_image /bin/bash" and it will work
        exec $CMD ${@:2}
        ;;
esac
