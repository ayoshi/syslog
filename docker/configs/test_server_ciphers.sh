#!/usr/bin/env bash
# adapted from https://superuser.com/questions/109213/how-do-i-list-the-ssl-tls-cipher-suites-a-particular-website-offers

# OpenSSL requires the port number.
# SERVER=192.168.1.1:443
SERVER=$1
if [[ -z "$SERVER" ]]; then echo "ERROR: no server specified"; exit 1; fi;

## Set up colors, if possible
if [[ $(tput colors) ]];then
    COLOR_BOLD="$(tput bold)"     # "\e[1;32m"
    COLOR_GREEN="$(tput setaf 2)" # "\e[1;32m"
    COLOR_RESET="$(tput sgr0)"    # "\e[0m"
fi


SERVER=$1:443
echo Server is ${COLOR_BOLD}"$SERVER"${COLOR_RESET}

DELAY=1
ciphers=$(openssl ciphers 'ALL:eNULL' | sed -e 's/:/ /g')

echo Obtaining cipher list from $(openssl version).

for cipher in ${ciphers[@]}
do
    printf "%-42s" "Testing $cipher... "
    result=$(echo -n | openssl s_client -cipher "$cipher" -connect $SERVER 2>&1)
    if [[ "$result" =~ ":error:" ]] ; then
        error=$(echo -n $result | cut -d':' -f6)
        echo NO \($error\)
    else
        if [[ "$result" =~ "Cipher is ${cipher}" || "$result" =~ "Cipher    :" ]] ; then
            echo ${COLOR_BOLD}${COLOR_GREEN}YES${COLOR_RESET}
        else
            echo UNKNOWN RESPONSE
            echo $result
        fi
    fi
    sleep $DELAY
done
