#!/bin/bash
mkcert -key-file key.pem -cert-file cert.pem localhost 127.0.0.1 ::1
mv key.pem abyss.key
mv cert.pem abyss.pem
