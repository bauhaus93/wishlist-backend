#!/bin/sh

FULL_REBUILD=0
CLEANUP=0
while getopts ":fc" option; do
	case "${option}" in
	f) FULL_REBUILD=1 ;;
	c) CLEANUP=1 ;;
	esac
done

if [ $CLEANUP -eq 1 ]; then
	docker image rm wishlist-backend-build wishlist-backend
	exit 0
fi

if [ $FULL_REBUILD -eq 1 ]; then
	docker build -t wishlist-backend-build -f docker/Dockerfile-build .
fi

docker build -t wishlist-backend -f docker/Dockerfile .
