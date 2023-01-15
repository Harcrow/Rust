#! usr/bin/bash
while getopts p: flag 
do
	case "${flag}" in 
		p) path=${OPTARG};;
	esac
done
gdb-multiarch -x openocd.gdb $path
