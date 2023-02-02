#!/bin/bash
./rs_file_explorer
pathname=$(cat /etc/cc/cc_cwd)
cd "$pathname" || return
