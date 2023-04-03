#!/bin/sh 

sudo -s 
echo "deb [arch=amd64, trusted=yes] https://mkproj.github.io/texcreate-deb texcreate-deb main" >> /etc/apt/sources.list
apt-get update && apt-get upgrade 
apt-get install texcreate