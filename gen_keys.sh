#!/bin/bash

openssl genrsa -out rsa_private.pem 2048
openssl rsa -in rsa_private.pem -pubout -out rsa_public.pem
openssl rsa -in rsa_private.pem -outform DER -put rsa_private.der