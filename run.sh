#!/bin/sh

export ENV_CONVERSION_TYPE=FIXED_DWORD
export ENV_CONVERSION_TYPE=FIXED_QWORD

strings(){
	echo hello, world
	echo hello
	echo world
	echo hw
}

strings | ./rs-string2fixed
