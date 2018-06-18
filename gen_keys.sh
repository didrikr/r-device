#!/bin/bash

openssl req -x509 -newkey rsa:2048 -keyout rsa_private.pem -nodes -out rsa_cert.pem -subj "/CN=unused"
openssl rsa -in rsa_private.pem -outform DER -out rsa_private.der
