#!/usr/bin/env bash

# Copy certificates to shared volume
cp -f /etc/syslog-ng/*.pem /syslog-ng/

CMD=$1

case "$CMD" in

    "syslog-ng" )
        exec /usr/sbin/syslog-ng ${@:2}
        ;;

    "syslog-ng-debug" )
        exec /usr/sbin/syslog-ng -F -d -v ${@:2}
        ;;

    * )
        # Run custom command. Thanks to this line we can still use
        # "docker run our_image /bin/bash" and it will work
        exec $CMD ${@:2}
        ;;
esac
